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

//#![allow(dead_code)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]

pub mod error;
pub mod headerflags;
pub mod helpers;
pub mod schema;
pub mod schema_extensions;
pub mod schema_gen;
pub mod session;

use crate::{headerflags::*, schema::*, schema_gen::*};
use apache_avro::from_value;
use error::Error;
use http_auth_basic::Credentials;

#[allow(unused_imports)]
use log::{info, trace, warn};

use session::Session;

use tungstenite::{connect, handshake::client::generate_key, http::Request};

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    etp_user: &'static str,
    #[default("")]
    etp_password: &'static str,
}

pub fn etp_get_server_capabilities() {
    // ETP Spec  4.3.1
    // TODO: GetCap
    // http://{url}/.well-known/etp-server- capabilities
    // Strip the wss: or ws: if present
}

pub fn etp_connect(
    url: &str,
    uname: &str,
    password: &str,
    request_session: RequestSession,
) -> Result<Session, error::Error> {
    let credentials = Credentials::new(uname, password);
    let credentials = credentials.as_http_header();

    let request = Request::builder()
        .uri(url)
        .header("Host", "localhost:9999")
        .header("Authorization", credentials)
        .header("Connection", "upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-Websocket-Key", generate_key())
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Protocol", "etp12.energistics.org")
        .header("MaxWebSocketFramePayloadSize", "4194304")
        .header("MaxWebSocketMessagePayloadSize", "16777216")
        .body(())
        .unwrap();

    // MAY need to handle redirects, and possibly special Auth Handling in this, will see later.
    let (ws_con, _response) = connect(request)?;
    info!("Connected to server at {}", "fred");

    let mut session = Session::new(ws_con);

    session.send_message(
        &request_session,
        CORE_REQUESTSESSION,
        0,
        MessageHeaderFlags::default(),
        None,
    )?;

    let (msg_hdr, msg_body) = session.read_message()?;
    match msg_hdr.msgtype() {
        CORE_PROTOCOLEXCEPTION => {
            // Request Fail!
            let pe = from_value::<ProtocolException>(&msg_body)?;
            match pe.error {
                Some(errinfo) => {
                    session.close();
                    return Err(Error::ProtocolException(errinfo.code, errinfo.message));
                }
                // None should never happen here, as not using map PE's in Core.
                None => {
                    session.close();
                    return Err(Error::ProtocolException(
                        0,
                        "Empty Exception, Unknown Reason".to_string(),
                    ));
                }
            }
        }
        CORE_OPENSESSION => {
            // Request Success ! Lets get setup
            let open_session = from_value::<OpenSession>(&msg_body)?;

            let compression_ok = open_session.supported_compression == "gzip";
            let extension_ok = false;

            session.set_session_open(
                compression_ok,
                compression_ok,
                extension_ok,
                open_session.session_id,
            );

            // Store for later reference
            session.open_session_msg = open_session;
            session.request_session_msg = request_session;

            return Ok(session);
        }
        (_, _) => {
            session.close();
            return Err(Error::UnsupportedWSMessage);
        }
    }
}

#[test]
fn test_connect() {
    let app_config = CONFIG;

    let request_session = RequestSession::default();

    let mut session = match etp_connect(
        "ws://localhost:9999/eml/etp",
        app_config.etp_user,
        app_config.etp_password,
        request_session,
    ) {
        Ok(session) => session,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    // Simple Discovery
    let gr_root = GetResources {
        context: ContextInfo {
            uri: "eml:///".to_string(),
            depth: 10,
            data_object_types: vec![],
            navigable_edges: RelationshipKind::Both,
            include_secondary_targets: false,
            include_secondary_sources: false,
        },
        scope: ContextScopeKind::SourcesOrSelf,
        count_objects: false,
        store_last_write_filter: None,
        active_status_filter: None,
        include_edges: false,
    };

    let _result = session.send_message(
        gr_root,
        DISCOVERY_GETRESOURCES,
        0,
        MessageHeaderFlags::default(),
        None,
    );

    let gr_response = match session.read_message() {
        Ok(msg) => msg,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match gr_response.0.msgtype() {
        DISCOVERY_GETRESOURCESRESPONSE => {
            let grr = match from_value::<GetResourcesResponse>(&gr_response.1) {
                Ok(msg) => msg,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            };
            println!("{:?}", grr);
        }
        (_, _) => {}
    }

    //println!("{:?}", result);

    // match result {
    //     Err(err) => {
    //         println!("{:?}", err)
    //     }
    //     Ok() => {

    //     }
    //}
}
