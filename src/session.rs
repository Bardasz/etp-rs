// Copyright 2023 - The Bardasz Group & etp-rs authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// ETP Schemas from Energistics Organisation are licenced under the Energistics Licence.
// You may not use those schema's except in compliance with the license.
// You can find a copy of the License at: schema/ENERGISTICS_LICENCE
//
// The following Energistics (c) products were used in the creation of this work: ETP 1.2 Specification.
//
// Author: Mark Farnan

use crate::{error::Error, headerflags::*, helpers::time_to_etp, schema::*, schema_gen::*};
use apache_avro::{to_value, types::Value};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
#[allow(unused_imports)]
use log::{info, trace, warn};
use serde::Serialize;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::SystemTime;
use std::{usize, vec};
use tungstenite::{stream::*, Message, WebSocket};

#[derive(Debug)]
pub struct Session {
    pub ws_conn: WebSocket<MaybeTlsStream<TcpStream>>,
    etp_schema: MsgSchema, // Processed ETP Schema.  Singleton created for the entire connection.  (might even do one for the whole server, //TODO)
    sent_msg_id: i64,      // Last sent MessageID
    rcv_msg_id: i64,       // Last Received MessageID
    session_id: [u8; 16],  // Session UUID
    version: usize,        // ETP Version negotiated
    open: bool,            // Is this ETP Session fully 'open' ?  (Traded Request/OpenSession etc)
    gzip: bool,            // If GZip is enabled on the connection
    compress_all: bool, // If we want to force compression for all messages, regardless of what the caller to send_message wants in the header
    extension_allowed: bool, // If message Extensions are allowed to this endpoint
    pub request_session_msg: RequestSession, // Message sent to request the session, - stored for later reference use (Protocols, etc)
    pub open_session_msg: OpenSession, // Message returned from Request Session - Stored for later use
}

impl Session {
    pub fn new(ws_con: WebSocket<MaybeTlsStream<TcpStream>>) -> Session {
        Session {
            ws_conn: ws_con,
            etp_schema: MsgSchema::new(),
            sent_msg_id: 0,
            rcv_msg_id: 0,
            session_id: [0; 16],
            version: 12,
            open: false,
            gzip: false,
            compress_all: true,
            extension_allowed: false,
            request_session_msg: RequestSession::default(),
            open_session_msg: OpenSession::default(),
        }
    }

    pub fn set_session_open(
        &mut self,
        gzip: bool,
        compress_all: bool,
        extension_allowed: bool,
        session_id: [u8; 16],
    ) {
        self.gzip = gzip;
        self.compress_all = compress_all;
        self.extension_allowed = extension_allowed;
        self.open = true;
        self.session_id = session_id;
        self.version = 12;
    }

    // Ack is special, as it has no body, just a header.
    pub fn send_ack(&mut self, corr_id: i64) -> Result<(), Error> {
        let hdr = MessageHeader {
            protocol: CORE_ACK.0 as i32,
            message_type: CORE_ACK.1 as i32,
            correlation_id: corr_id,
            message_id: self.sent_msg_id,
            message_flags: MessageHeaderFlags::default().as_i32(),
        };

        // Make and Send the Message
        self.sent_msg_id = self.sent_msg_id + 2; // Next msgID. Even Client, Odd Server.  Global for Connection.
        let message = self.etp_schema.serialize_header(&hdr)?;
        self.ws_conn.write_message(Message::Binary(message))?;
        return Ok(());
    }

    // Returns MessageID of the sent message
    pub fn send_message<S: Serialize>(
        &mut self,
        body: S,
        msgtype: (usize, usize), // Protocol, MsgType
        correlationid: i64,
        msgflags: MessageHeaderFlags,
        extension: Option<MessageHeaderExtension>,
    ) -> Result<i64, Error> {
        // Handle Header
        let mut flags = msgflags;

        // Check if compression is allowed, and turn it OFF if it's not
        if flags.compress && !self.gzip {
            flags.compress = false;
        }

        // If 'compressAll'  override the header and force it to compress all (if gzip enabled)
        if self.compress_all && self.gzip {
            flags.compress = true
        }

        // Disable compress for messages that must NEVER be compressed
        // ACK, ProtocolException,  or anything in protocol 0
        // .1 = MsgType,  .0 = Protocol
        if msgtype.1 == 1000 || msgtype.1 == 1001 || msgtype.0 == 0 {
            flags.compress = false
        }

        // Message Extension needs to be packaged and sent, if permitted
        if self.extension_allowed {
            match extension {
                Some(_ext) => flags.extension = true,
                None => {}
            }
        }

        // Make the Message Header
        self.sent_msg_id = self.sent_msg_id + 2; // Next msgID. Even Client, Odd Server.  Global for Connection.

        let hdr = MessageHeader {
            protocol: msgtype.0 as i32,
            message_type: msgtype.1 as i32,
            correlation_id: correlationid,
            message_id: self.sent_msg_id,
            message_flags: flags.as_i32(),
        };

        // Make the Message
        let body_value = to_value(body)?;
        let msg = self.etp_schema.serialize_message(msgtype, body_value)?;

        let mut message = self.etp_schema.serialize_header(&hdr)?;

        if flags.compress {
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            e.write_all(&mut msg.as_slice())?;
            let msg_compressed = e.finish()?;
            message.extend(msg_compressed);
        } else {
            // No compression
            message.extend(msg);
        }
        self.ws_conn.write_message(Message::Binary(message))?;
        return Ok(hdr.message_id);
    }

    // Loop until valid msg to return
    // Handles responding to Ping internally and waits for next message
    // Automatically Responds with 'Ack's to received messages if required.
    pub fn read_message(&mut self) -> Result<(MessageHeader, Value), Error> {
        loop {
            let message = self.ws_conn.read_message()?;
            match message {
                Message::Binary(msg) => {
                    let mut msg_bytes = msg.as_slice();
                    let msg_hdr = self.etp_schema.deserialize_header(&mut msg_bytes)?;

                    self.rcv_msg_id = msg_hdr.message_id; // Store last rcvd ID

                    let mut msg_unzip: Vec<u8> = vec![];
                    if msg_hdr.get_flags().compress {
                        let mut gz = GzDecoder::new(&mut msg_bytes);
                        let _result = gz.read_to_end(&mut msg_unzip);
                        msg_bytes = msg_unzip.as_slice();
                        //println!("{:?}", result);
                    }

                    let msg_value = self.etp_schema.deserialize_message(
                        (msg_hdr.protocol as usize, msg_hdr.message_type as usize),
                        &mut msg_bytes,
                    )?;

                    // Handle Ping, pong and any other housekeeping.
                    match msg_hdr.msgtype() {
                        CORE_PING => {
                            _ = self.send_message(
                                Pong {
                                    current_date_time: time_to_etp(SystemTime::now()),
                                },
                                CORE_PONG,
                                0,
                                MessageHeaderFlags::default(),
                                None,
                            )?
                        }

                        _ => {
                            // Handle 'Ack Requested'.
                            if msg_hdr.get_flags().reqack {
                                self.send_ack(msg_hdr.message_id)?;
                            }
                            return Ok((msg_hdr, msg_value));
                        }
                    }
                }
                _ => {
                    // Note: Websocket Ping/Pong already handled inside ws_conn.read_message
                    return Err(Error::UnsupportedWSMessage);
                }
            }
        }
    }
    // Closes the session.  This is very conservative and dosn't error.
    // Checks if Session is open or not, if it is, sends Close Session
    // If the WS connection is already closed, it just returns (dosn't error)
    pub fn close(&mut self) {
        // Already closed, just bail
        if !self.ws_conn.can_write() {
            return;
        }

        // Close, and bail if error
        let result = self.ws_conn.close(None);
        match result {
            Err(_err) => return,
            Ok(()) => {}
        }

        loop {
            let result = self.ws_conn.read_message();
            match result {
                Err(_err) => return, // This should be 'Error::ConnectionClosed' but really, ANY error, and we are done
                Ok(_msg) => {}
            }
        }
    }
}
