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

use crate::schema_gen::*;
use apache_avro::{
    from_avro_datum, from_avro_datum_schemata, from_value, to_avro_datum, to_avro_datum_schemata,
    to_value, types::Value, AvroResult, Error, Schema,
};
use std::collections::HashMap;
use std::io::Read;

// ------------------------------------------------------
// Helper
// ------------------------------------------------------
#[derive(Debug)]
pub struct MsgSchema {
    parsed_schemata: Vec<Schema>,
    messageid_schema: HashMap<(usize, usize), usize>,
    messageid_name: HashMap<(usize, usize), String>,
    messageheader_schema: Schema,
}

impl MsgSchema {
    pub fn new() -> Self {
        // Pre-Parse the Msg Header Schema's and store.
        let messageheader_schema = Schema::parse_str(ETP_MESSAGE_HEADER).unwrap();

        // Parse the bulk schema's for all Messages
        let schemata: Vec<Schema> = Schema::parse_list(&ETP_SCHEMA_EMBED).unwrap();

        let mut messageid_schema: HashMap<(usize, usize), usize> = HashMap::new();
        let mut messageid_name: HashMap<(usize, usize), String> = HashMap::new();

        for (pos, schema) in schemata.iter().enumerate() {
            // Make the lookup table for protocol message Schemas

            match schema {
                Schema::Record {
                    name, attributes, ..
                } => {
                    let ns = match &name.namespace {
                        Some(v) => v.as_str(),
                        None => "",
                    };

                    let n = ns.strip_prefix("Energistics.Etp.v12.Protocol.");

                    match n {
                        Some(v) => {
                            // Errors ignored with unwrap, as this is a one off from Static Content.  If the Test passes, it will pass in production.
                            let msgtype: usize = attributes
                                .get("messageType")
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .parse()
                                .unwrap();
                            let protocol: usize = attributes
                                .get("protocol")
                                .unwrap()
                                .as_str()
                                .unwrap()
                                .parse()
                                .unwrap();

                            // Protocol/MsgID to Schema
                            messageid_schema.insert((protocol, msgtype), pos);

                            // Protocol/MsgID's to Protocol/Message Name
                            let key = format!("{}.{}", v, &name.name).to_string();
                            messageid_name.insert((protocol, msgtype), key);
                        }
                        None => {}
                    }
                }
                _ => (), // Default Match
            }
        }

        Self {
            parsed_schemata: schemata,
            messageid_schema: messageid_schema,
            messageid_name: messageid_name,
            messageheader_schema: messageheader_schema,
        }
    }

    pub fn msg_name(&self, message: (usize, usize)) -> Option<&str> {
        match self.messageid_name.get(&message) {
            None => None,
            Some(v) => Some(v),
        }
    }

    pub fn serialize_message<T: Into<Value>>(
        &self,
        message: (usize, usize),
        value: T,
    ) -> AvroResult<Vec<u8>> {
        match self.messageid_schema.get(&message) {
            // Lookup root Schema
            None => {
                return Err(Error::ValidationWithReason(
                    "Can't find root schema for specified message".to_string(),
                ))
            }
            Some(v) => {
                let schema_root = &self.parsed_schemata[*v];
                let schemata: Vec<&Schema> = self.parsed_schemata.iter().collect();
                return to_avro_datum_schemata(&schema_root, schemata.as_slice(), value);
            }
        }
    }

    pub fn deserialize_message<R: Read>(
        &self,
        message: (usize, usize),
        reader: &mut R,
    ) -> AvroResult<Value> {
        match self.messageid_schema.get(&message) {
            // Lookup root Schema
            None => {
                return Err(Error::ValidationWithReason(
                    "Can't find root schema for specified message".to_string(),
                ))
            }
            Some(v) => {
                let schema_root = &self.parsed_schemata[*v];
                let schemata: Vec<&Schema> = self.parsed_schemata.iter().collect();
                return from_avro_datum_schemata(&schema_root, schemata.as_slice(), reader, None);
            }
        }
    }

    pub fn serialize_header(&self, header: &MessageHeader) -> AvroResult<Vec<u8>> {
        let hdr_value = to_value(header)?;
        return to_avro_datum(&self.messageheader_schema, hdr_value);
    }
    pub fn deserialize_header<R: Read>(&self, header: &mut R) -> AvroResult<MessageHeader> {
        let record = from_avro_datum(&self.messageheader_schema, header, None)?;
        return from_value::<MessageHeader>(&record);
    }
}

pub enum Role {
    Client,
    Server,
    Customer,
    Store,
    Producer,
    Consumer,
}

impl Role {
    pub fn to_string(self) -> String {
        match self {
            Self::Client => "client".to_string(),
            Self::Server => "server".to_string(),
            Self::Customer => "customer".to_string(),
            Self::Store => "store".to_string(),
            Self::Producer => "producer".to_string(),
            Self::Consumer => "consumer".to_string(),
        }
    }
}

#[test]
fn test_roundtrip_message_header() {
    let header = MessageHeader {
        protocol: 0,
        message_type: 1,
        correlation_id: 52,
        message_id: 51,
        message_flags: 19,
    };
    let es = MsgSchema::new();

    let hdr_encoded = es.serialize_header(&header).unwrap();

    let expected: Vec<u8> = vec![0, 2, 104, 102, 38];
    assert_eq!(hdr_encoded, expected);

    let mut hdr_bytes = hdr_encoded.as_slice();
    let newhdr = es.deserialize_header(&mut hdr_bytes).unwrap();

    assert_eq!(header, newhdr);
}

#[test]
fn test_roundtrip() {
    let pe = ProtocolException {
        error: Some(ErrorInfo {
            message: String::from("some Error Message"),
            code: 245,
        }),
        errors: ::std::collections::HashMap::new(),
    };

    let es = MsgSchema::new();

    let pe_value = to_value(pe).unwrap();
    //println!("{:?}", pe_value);
    let expected: Vec<u8> = vec![
        2, 36, 115, 111, 109, 101, 32, 69, 114, 114, 111, 114, 32, 77, 101, 115, 115, 97, 103, 101,
        234, 3, 0,
    ];

    let pe_encoded = es
        .serialize_message(CORE_PROTOCOLEXCEPTION, pe_value)
        .unwrap();

    assert_eq!(pe_encoded, expected);

    //println!("{:?}", pe_encoded);

    let mut pe_bytes = pe_encoded.as_slice();
    let _pe_back = es
        .deserialize_message(CORE_PROTOCOLEXCEPTION, &mut pe_bytes)
        .unwrap();

    //println!("{:?}", pe_back);
}

#[test]
fn test_roundtrip_channeldata() {
    // let cs = ChannelDataCdl {
    //     data: vec![DataItem {
    //         channel_id: 1,
    //         indexes: vec![IndexValue {
    //             item: Some(UnionLongDoublePassIndexedDepth::Double(34.1)),
    //         }],
    //         value: DataValue { item: Some(UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray::Double(34.1)) },
    //         value_attributes: Vec::new(),
    //     }],
    // };

    let cs = ChannelDataCdl {
        data: vec![DataItem {
            channel_id: 1,
            indexes: Vec::new(),
            value: DataValue {
                item: DataValueEnum::Double(34.1),
            },
            value_attributes: Vec::new(),
        }],
    };

    let es = MsgSchema::new();

    let cs_value = to_value(cs).unwrap();
    println!("{:?}", cs_value);
    let expected: Vec<u8> = vec![2, 2, 0, 10, 205, 204, 204, 204, 204, 12, 65, 64, 0, 0];

    let cs_encoded = es
        .serialize_message(CHANNELSUBSCRIBE_CHANNELDATA, cs_value)
        .unwrap();

    println!("{:?}", cs_encoded);
    assert_eq!(cs_encoded, expected);

    //println!("{:?}", pe_encoded);

    let mut cs_bytes = cs_encoded.as_slice();
    let pe_back = es
        .deserialize_message(CHANNELSUBSCRIBE_CHANNELDATA, &mut cs_bytes)
        .unwrap();

    println!("{:?}", pe_back);
}

#[test]
fn test_msg_name() {
    let es = MsgSchema::new();

    let name = es.msg_name(CORE_OPENSESSION).unwrap();
    assert_eq!(name, "Core.OpenSession");

    let name = es.msg_name(TRANSACTION_COMMITTRANSACTION).unwrap();
    assert_eq!(name, "Transaction.CommitTransaction");
}

// Test func to generate the Protocol_Message Constants used in this file, from the Schemas.
// #[test]
// fn test_gen_const_id() {
//     let es = Etpschema::new();

//     let mut n: Vec<(String, usize, usize)> = Vec::new();
//     for (k, v) in es.message_schema {
//         n.push((k, v.0, v.1));
//     }

//     n.sort();

//     for v in n {
//         let v1 = v.0.replace(".", "_").to_uppercase();
//         println!("pub const {}: (usize, usize) = ({},{});", v1, v.1, v.2)
//     }
// }

pub const ETP12VERSION: Version = Version {
    major: 1,
    minor: 2,
    revision: 0,
    patch: 0,
};

pub const ETP11VERSION: Version = Version {
    major: 1,
    minor: 1,
    revision: 0,
    patch: 0,
};

// ------------------------------------------------------
// Protocol/Message ID Constants - Used for MapKeys.
// ------------------------------------------------------

// Channel Streaming
pub const CHANNELSTREAMING_CHANNELMETADATA: (usize, usize) = (1, 1);
pub const CHANNELSTREAMING_CHANNELDATA_CS: (usize, usize) = (1, 2);
pub const CHANNELSTREAMING_TRUNCATECHANNELS_CS: (usize, usize) = (1, 5);

// Channel Data Frame
pub const CHANNELDATAFRAME_GETFRAMEMETADATA: (usize, usize) = (2, 1);
pub const CHANNELDATAFRAME_GETFRAMEMETADATARESPONSE: (usize, usize) = (2, 2);
pub const CHANNELDATAFRAME_GETFRAME: (usize, usize) = (2, 3);
pub const CHANNELDATAFRAME_GETFRAMERESPONSEHEADER: (usize, usize) = (2, 4);
pub const CHANNELDATAFRAME_CANCELGETFRAME: (usize, usize) = (2, 5);
pub const CHANNELDATAFRAME_GETFRAMERESPONSEROWS: (usize, usize) = (2, 6);

// Channel Data Load
pub const CHANNELDATALOAD_OPENCHANNELS: (usize, usize) = (22, 1);
pub const CHANNELDATALOAD_OPENCHANNELSRESPONSE: (usize, usize) = (22, 2);
pub const CHANNELDATALOAD_CLOSECHANNELS: (usize, usize) = (22, 3);
pub const CHANNELDATALOAD_CHANNELDATA_CDL: (usize, usize) = (22, 4);
pub const CHANNELDATALOAD_REPLACERANGE: (usize, usize) = (22, 6);
pub const CHANNELDATALOAD_CHANNELSCLOSED: (usize, usize) = (22, 7);
pub const CHANNELDATALOAD_REPLACERANGERESPONSE: (usize, usize) = (22, 8);
pub const CHANNELDATALOAD_TRUNCATECHANNELS: (usize, usize) = (22, 9);
pub const CHANNELDATALOAD_TRUNCATECHANNELSRESPONSE: (usize, usize) = (22, 10);

// Channel Subscribe
pub const CHANNELSUBSCRIBE_GETCHANNELMETADATA: (usize, usize) = (21, 1);
pub const CHANNELSUBSCRIBE_GETCHANNELMETADATARESPONSE: (usize, usize) = (21, 2);
pub const CHANNELSUBSCRIBE_SUBSCRIBECHANNELS: (usize, usize) = (21, 3);
pub const CHANNELSUBSCRIBE_CHANNELDATA: (usize, usize) = (21, 4);
pub const CHANNELSUBSCRIBE_RANGEREPLACED: (usize, usize) = (21, 6);
pub const CHANNELSUBSCRIBE_UNSUBSCRIBECHANNELS: (usize, usize) = (21, 7);
pub const CHANNELSUBSCRIBE_SUBSCRIPTIONSSTOPPED: (usize, usize) = (21, 8);
pub const CHANNELSUBSCRIBE_GETRANGES: (usize, usize) = (21, 9);
pub const CHANNELSUBSCRIBE_GETRANGESRESPONSE: (usize, usize) = (21, 10);
pub const CHANNELSUBSCRIBE_CANCELGETRANGES: (usize, usize) = (21, 11);
pub const CHANNELSUBSCRIBE_CHANNELSTRUNCATED: (usize, usize) = (21, 13);
pub const CHANNELSUBSCRIBE_GETCHANGEANNOTATIONS: (usize, usize) = (21, 14);
pub const CHANNELSUBSCRIBE_GETCHANGEANNOTATIONSRESPONSE: (usize, usize) = (21, 15);
pub const CHANNELSUBSCRIBE_SUBSCRIBECHANNELSRESPONSE: (usize, usize) = (21, 12);

// Core
pub const CORE_REQUESTSESSION: (usize, usize) = (0, 1);
pub const CORE_OPENSESSION: (usize, usize) = (0, 2);
pub const CORE_CLOSESESSION: (usize, usize) = (0, 5);
pub const CORE_AUTHORIZE: (usize, usize) = (0, 6);
pub const CORE_AUTHORIZERESPONSE: (usize, usize) = (0, 7);
pub const CORE_PING: (usize, usize) = (0, 8);
pub const CORE_PONG: (usize, usize) = (0, 9);
pub const CORE_PROTOCOLEXCEPTION: (usize, usize) = (0, 1000);
pub const CORE_ACK: (usize, usize) = (0, 1001);

// Data Array
pub const DATAARRAY_GETDATAARRAYSRESPONSE: (usize, usize) = (9, 1);
pub const DATAARRAY_GETDATAARRAYS: (usize, usize) = (9, 2);
pub const DATAARRAY_GETDATASUBARRAYS: (usize, usize) = (9, 3);
pub const DATAARRAY_PUTDATAARRAYS: (usize, usize) = (9, 4);
pub const DATAARRAY_PUTDATASUBARRAYS: (usize, usize) = (9, 5);
pub const DATAARRAY_GETDATAARRAYMETADATA: (usize, usize) = (9, 6);
pub const DATAARRAY_GETDATAARRAYMETADATARESPONSE: (usize, usize) = (9, 7);
pub const DATAARRAY_GETDATASUBARRAYSRESPONSE: (usize, usize) = (9, 8);
pub const DATAARRAY_PUTUNINITIALIZEDDATAARRAYS: (usize, usize) = (9, 9);
pub const DATAARRAY_PUTDATAARRAYSRESPONSE: (usize, usize) = (9, 10);
pub const DATAARRAY_PUTDATASUBARRAYSRESPONSE: (usize, usize) = (9, 11);
pub const DATAARRAY_PUTUNINITIALIZEDDATAARRAYSRESPONSE: (usize, usize) = (9, 12);

//DataSpace
pub const DATASPACE_GETDATASPACES: (usize, usize) = (24, 1);
pub const DATASPACE_GETDATASPACESRESPONSE: (usize, usize) = (24, 2);
pub const DATASPACE_PUTDATASPACES: (usize, usize) = (24, 3);
pub const DATASPACE_DELETEDATASPACES: (usize, usize) = (24, 4);
pub const DATASPACE_DELETEDATASPACESRESPONSE: (usize, usize) = (24, 5);
pub const DATASPACE_PUTDATASPACESRESPONSE: (usize, usize) = (24, 6);

//Discovery
pub const DISCOVERY_GETRESOURCES: (usize, usize) = (3, 1);
pub const DISCOVERY_GETRESOURCESRESPONSE: (usize, usize) = (3, 4);
pub const DISCOVERY_GETDELETEDRESOURCES: (usize, usize) = (3, 5);
pub const DISCOVERY_GETDELETEDRESOURCESRESPONSE: (usize, usize) = (3, 6);
pub const DISCOVERY_GETRESOURCESEDGESRESPONSE: (usize, usize) = (3, 7);

//Discovery Query
pub const DISCOVERYQUERY_FINDRESOURCES: (usize, usize) = (13, 1);
pub const DISCOVERYQUERY_FINDRESOURCESRESPONSE: (usize, usize) = (13, 2);

//Growing Object
pub const GROWINGOBJECT_DELETEPARTS: (usize, usize) = (6, 1);
pub const GROWINGOBJECT_GETPARTS: (usize, usize) = (6, 3);
pub const GROWINGOBJECT_GETPARTSBYRANGE: (usize, usize) = (6, 4);
pub const GROWINGOBJECT_PUTPARTS: (usize, usize) = (6, 5);
pub const GROWINGOBJECT_GETPARTSRESPONSE: (usize, usize) = (6, 6);
pub const GROWINGOBJECT_REPLACEPARTSBYRANGE: (usize, usize) = (6, 7);
pub const GROWINGOBJECT_GETPARTSMETADATA: (usize, usize) = (6, 8);
pub const GROWINGOBJECT_GETPARTSMETADATARESPONSE: (usize, usize) = (6, 9);
pub const GROWINGOBJECT_GETPARTSBYRANGERESPONSE: (usize, usize) = (6, 10);
pub const GROWINGOBJECT_DELETEPARTSRESPONSE: (usize, usize) = (6, 11);
pub const GROWINGOBJECT_PUTPARTSRESPONSE: (usize, usize) = (6, 13);
pub const GROWINGOBJECT_GETGROWINGDATAOBJECTSHEADER: (usize, usize) = (6, 14);
pub const GROWINGOBJECT_GETGROWINGDATAOBJECTSHEADERRESPONSE: (usize, usize) = (6, 15);
pub const GROWINGOBJECT_PUTGROWINGDATAOBJECTSHEADER: (usize, usize) = (6, 16);
pub const GROWINGOBJECT_PUTGROWINGDATAOBJECTSHEADERRESPONSE: (usize, usize) = (6, 17);
pub const GROWINGOBJECT_GETCHANGEANNOTATIONS_GO: (usize, usize) = (6, 19);
pub const GROWINGOBJECT_GETCHANGEANNOTATIONSRESPONSE_GO: (usize, usize) = (6, 20);

// Growing Object Notification
pub const GROWINGOBJECTNOTIFICATION_PARTSCHANGED: (usize, usize) = (7, 2);
pub const GROWINGOBJECTNOTIFICATION_PARTSDELETED: (usize, usize) = (7, 3);
pub const GROWINGOBJECTNOTIFICATION_UNSUBSCRIBEPARTNOTIFICATION: (usize, usize) = (7, 4);
pub const GROWINGOBJECTNOTIFICATION_PARTSREPLACEDBYRANGE: (usize, usize) = (7, 6);
pub const GROWINGOBJECTNOTIFICATION_SUBSCRIBEPARTNOTIFICATIONS: (usize, usize) = (7, 7);
pub const GROWINGOBJECTNOTIFICATION_PARTSUBSCRIPTIONENDED: (usize, usize) = (7, 8);
pub const GROWINGOBJECTNOTIFICATION_UNSOLICITEDPARTNOTIFICATIONS: (usize, usize) = (7, 9);
pub const GROWINGOBJECTNOTIFICATION_SUBSCRIBEPARTNOTIFICATIONSRESPONSE: (usize, usize) = (7, 10);

// Growing object Query
pub const GROWINGOBJECTQUERY_FINDPARTS: (usize, usize) = (16, 1);
pub const GROWINGOBJECTQUERY_FINDPARTSRESPONSE: (usize, usize) = (16, 2);

// Store
pub const STORE_GETDATAOBJECTS: (usize, usize) = (4, 1);
pub const STORE_PUTDATAOBJECTS: (usize, usize) = (4, 2);
pub const STORE_DELETEDATAOBJECTS: (usize, usize) = (4, 3);
pub const STORE_GETDATAOBJECTSRESPONSE: (usize, usize) = (4, 4);
pub const STORE_CHUNK: (usize, usize) = (4, 8);
pub const STORE_PUTDATAOBJECTSRESPONSE: (usize, usize) = (4, 9);
pub const STORE_DELETEDATAOBJECTSRESPONSE: (usize, usize) = (4, 10);

// Store Notification
pub const STORENOTIFICATION_OBJECTCHANGED: (usize, usize) = (5, 2);
pub const STORENOTIFICATION_OBJECTDELETED: (usize, usize) = (5, 3);
pub const STORENOTIFICATION_UNSUBSCRIBENOTIFICATIONS: (usize, usize) = (5, 4);
pub const STORENOTIFICATION_OBJECTACCESSREVOKED: (usize, usize) = (5, 5);
pub const STORENOTIFICATION_SUBSCRIBENOTIFICATIONS: (usize, usize) = (5, 6);
pub const STORENOTIFICATION_SUBSCRIPTIONENDED: (usize, usize) = (5, 7);
pub const STORENOTIFICATION_UNSOLICITEDSTORENOTIFICATIONS: (usize, usize) = (5, 8);
pub const STORENOTIFICATION_CHUNK_SN: (usize, usize) = (5, 9);
pub const STORENOTIFICATION_SUBSCRIBENOTIFICATIONSRESPONSE: (usize, usize) = (5, 10);
pub const STORENOTIFICATION_OBJECTACTIVESTATUSCHANGED: (usize, usize) = (5, 11);

// Store Query
pub const STOREQUERY_FINDDATAOBJECTS: (usize, usize) = (14, 1);
pub const STOREQUERY_FINDDATAOBJECTSRESPONSE: (usize, usize) = (14, 2);
pub const STOREQUERY_CHUNK_SQ: (usize, usize) = (14, 3);

// Supported Types
pub const SUPPORTEDTYPES_GETSUPPORTEDTYPES: (usize, usize) = (25, 1);
pub const SUPPORTEDTYPES_GETSUPPORTEDTYPESRESPONSE: (usize, usize) = (25, 2);

// Transaction
pub const TRANSACTION_STARTTRANSACTION: (usize, usize) = (18, 1);
pub const TRANSACTION_STARTTRANSACTIONRESPONSE: (usize, usize) = (18, 2);
pub const TRANSACTION_COMMITTRANSACTION: (usize, usize) = (18, 3);
pub const TRANSACTION_ROLLBACKTRANSACTION: (usize, usize) = (18, 4);
pub const TRANSACTION_COMMITTRANSACTIONRESPONSE: (usize, usize) = (18, 5);
pub const TRANSACTION_ROLLBACKTRANSACTIONRESPONSE: (usize, usize) = (18, 6);

// WITSML SOAP (Private Protocol)
pub const WITSMLSOAP_WMLS_ADDTOSTORE: (usize, usize) = (2100, 1);
pub const WITSMLSOAP_WMLS_ADDTOSTORERESPONSE: (usize, usize) = (2100, 2);
pub const WITSMLSOAP_WMLS_DELETEFROMSTORE: (usize, usize) = (2100, 3);
pub const WITSMLSOAP_WMLS_DELETEFROMSTORERESPONSE: (usize, usize) = (2100, 4);
pub const WITSMLSOAP_WMLS_GETBASEMSG: (usize, usize) = (2100, 5);
pub const WITSMLSOAP_WMLS_GETBASEMSGRESPONSE: (usize, usize) = (2100, 6);
pub const WITSMLSOAP_WMLS_GETCAP: (usize, usize) = (2100, 7);
pub const WITSMLSOAP_WMLS_GETCAPRESPONSE: (usize, usize) = (2100, 8);
pub const WITSMLSOAP_WMLS_GETFROMSTORE: (usize, usize) = (2100, 9);
pub const WITSMLSOAP_WMLS_GETFROMSTORERESPONSE: (usize, usize) = (2100, 10);
pub const WITSMLSOAP_WMLS_GETVERSION: (usize, usize) = (2100, 11);
pub const WITSMLSOAP_WMLS_GETVERSIONRESPONSE: (usize, usize) = (2100, 12);
pub const WITSMLSOAP_WMLS_UPDATEINSTORE: (usize, usize) = (2100, 13);
pub const WITSMLSOAP_WMLS_UPDATEINSTORERESPONSE: (usize, usize) = (2100, 14);

// ------------------------------------------------------------------------------------------------------------
// Schemas
// These are ORDERED by dependency.  Do not change the order of the entries in the slice.
// ------------------------------------------------------------------------------------------------------------
static ETP_MESSAGE_HEADER: &str = r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "MessageHeader","fields":[{ "name": "protocol", "type": "int" },{ "name": "messageType", "type": "int" },{ "name": "correlationId", "type": "long" },{ "name": "messageId", "type": "long" },{ "name": "messageFlags", "type": "int" }]}"##;

static ETP_SCHEMA_EMBED: [&str; 207] = [
    // Kinds
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes","name": "Protocol","symbols":["Core","ChannelStreaming","ChannelDataFrame","Discovery","Store","StoreNotification","GrowingObject","GrowingObjectNotification","DEPRECATED_8","DataArray","RESERVED_10","RESERVED_11","RESERVED_12","DiscoveryQuery","StoreQuery","RESERVED_15","GrowingObjectQuery","RESERVED_17","Transaction","RESERVED_19","RESERVED_20","ChannelSubscribe","ChannelDataLoad","RESERVED_23","Dataspace","SupportedTypes"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ActiveStatusKind","symbols":["Active","Inactive"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "ChannelIndexKind","symbols":["DateTime","ElapsedTime","MeasuredDepth","TrueVerticalDepth","PassIndexedDepth","Pressure","Temperature","Scalar"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "ChannelDataKind","symbols":["DateTime","ElapsedTime","MeasuredDepth","PassIndexedDepth","TrueVerticalDepth","typeBoolean","typeInt","typeLong","typeFloat","typeDouble","typeString","typeBytes"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "RelationshipKind","symbols":["Primary","Secondary","Both"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ContextScopeKind","symbols":["self","sources","targets","sourcesOrSelf","targetsOrSelf"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ObjectChangeKind","symbols":["insert","update","authorized","joined","unjoined","joinedSubscription","unjoinedSubscription"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes","name": "EndpointCapabilityKind","symbols":["ActiveTimeoutPeriod","AuthorizationDetails","ChangePropagationPeriod","ChangeRetentionPeriod","MaxConcurrentMultipart","MaxDataObjectSize","MaxPartSize","MaxSessionClientCount","MaxSessionGlobalCount","MaxWebSocketFramePayloadSize","MaxWebSocketMessagePayloadSize","MultipartMessageTimeoutPeriod","ResponseTimeoutPeriod","RequestSessionTimeoutPeriod","SessionEstablishmentTimeoutPeriod","SupportsAlternateRequestUris","SupportsMessageHeaderExtensions"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes","name": "ProtocolCapabilityKind","symbols":["FrameChangeDetectionPeriod","MaxDataArraySize","MaxDataObjectSize","MaxFrameResponseRowCount","MaxIndexCount","MaxRangeChannelCount","MaxRangeDataItemCount","MaxResponseCount","MaxStreamingChannelsSessionCount","MaxSubscriptionSessionCount","MaxTransactionCount","SupportsSecondaryIndexFiltering","TransactionTimeoutPeriod"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes","name": "DataObjectCapabilityKind","symbols":["ActiveTimeoutPeriod","MaxContainedDataObjectCount","MaxDataObjectSize","OrphanedChildrenPrunedOnDelete","SupportsGet","SupportsPut","SupportsDelete","MaxSecondaryIndexCount"]}"##,
    // No Dependencies
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ErrorInfo","fields":[{ "name": "message", "type": "string" },{ "name": "code", "type": "int" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "Version","fields":[{ "name": "major", "type": "int", "default": 0 },{ "name": "minor", "type": "int", "default": 0 },{ "name": "revision", "type": "int", "default": 0 },{ "name": "patch", "type": "int", "default": 0 }]}"##,
    r##"{"type": "fixed","namespace": "Energistics.Etp.v12.Datatypes","name": "Uuid","size": 16}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "Contact","fields":[{ "name": "organizationName", "type": "string", "default": "" },{ "name": "contactName", "type": "string", "default": "" },{ "name": "contactPhone", "type": "string", "default": "" },{ "name": "contactEmail", "type": "string", "default": "" }]}"##,
    // Data Value & Attributes tree
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes","name": "AnyArrayType","symbols":["arrayOfBoolean","arrayOfInt","arrayOfLong","arrayOfFloat","arrayOfDouble","arrayOfString","bytes"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes","name": "AnyLogicalArrayType","symbols":["arrayOfBoolean","arrayOfInt8","arrayOfUInt8","arrayOfInt16LE","arrayOfInt32LE","arrayOfInt64LE","arrayOfUInt16LE","arrayOfUInt32LE","arrayOfUInt64LE","arrayOfFloat32LE","arrayOfDouble64LE","arrayOfInt16BE","arrayOfInt32BE","arrayOfInt64BE","arrayOfUInt16BE","arrayOfUInt32BE","arrayOfUInt64BE","arrayOfFloat32BE","arrayOfDouble64BE","arrayOfString","arrayOfCustom"]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfString","fields":[{ "name": "values","type": { "type": "array", "items": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfNullableBoolean","fields":[{"name": "values","type": { "type": "array", "items": ["null", "boolean"] }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfLong","fields":[{ "name": "values","type": { "type": "array", "items": "long" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfBytes","fields":[{ "name": "values","type": { "type": "array", "items": "bytes" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfInt","fields":[{ "name": "values","type": { "type": "array", "items": "int" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfBoolean","fields":[{ "name": "values","type": { "type": "array", "items": "boolean" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfFloat","fields":[{ "name": "values","type": { "type": "array", "items": "float" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfDouble","fields":[{ "name": "values","type": { "type": "array", "items": "double" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfNullableLong","fields":[{"name": "values","type": { "type": "array", "items": ["null", "long"] }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ArrayOfNullableInt","fields":[{"name": "values","type": { "type": "array", "items": ["null", "int"] }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "AnyArray","fields":[{"name": "item","type": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean","Energistics.Etp.v12.Datatypes.ArrayOfInt","Energistics.Etp.v12.Datatypes.ArrayOfLong","Energistics.Etp.v12.Datatypes.ArrayOfFloat","Energistics.Etp.v12.Datatypes.ArrayOfDouble","Energistics.Etp.v12.Datatypes.ArrayOfString","bytes"]}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "AnySubarray","fields":[{ "name": "start", "type": "long" },{ "name": "slice", "type": "Energistics.Etp.v12.Datatypes.AnyArray" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "AnySparseArray","fields":[{ "name": "slices","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.AnySubarray" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "DataValue","fields":[{"name": "item","type": ["null","boolean","int","long","float","double","string","Energistics.Etp.v12.Datatypes.ArrayOfBoolean","Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean","Energistics.Etp.v12.Datatypes.ArrayOfInt","Energistics.Etp.v12.Datatypes.ArrayOfNullableInt","Energistics.Etp.v12.Datatypes.ArrayOfLong","Energistics.Etp.v12.Datatypes.ArrayOfNullableLong","Energistics.Etp.v12.Datatypes.ArrayOfFloat","Energistics.Etp.v12.Datatypes.ArrayOfDouble","Energistics.Etp.v12.Datatypes.ArrayOfString","Energistics.Etp.v12.Datatypes.ArrayOfBytes","bytes","Energistics.Etp.v12.Datatypes.AnySparseArray"]}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "DataAttribute","fields":[{ "name": "attributeId", "type": "int" },{ "name": "attributeValue", "type": "Energistics.Etp.v12.Datatypes.DataValue" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "AttributeMetadataRecord","fields":[{ "name": "attributeId", "type": "int" },{ "name": "attributeName", "type": "string" },{ "name": "dataKind", "type": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind" },{ "name": "uom", "type": "string" },{ "name": "depthDatum", "type": "string" },{ "name": "attributePropertyKindUri", "type": "string" },{ "name": "axisVectorLengths","type": { "type": "array", "items": "int" }}]}"##,
    // DataArray
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes","name": "DataArrayIdentifier","fields":[{ "name": "uri", "type": "string" },{ "name": "pathInResource", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes","name": "DataArray","fields":[{ "name": "dimensions","type": { "type": "array", "items": "long" }},{ "name": "data", "type": "Energistics.Etp.v12.Datatypes.AnyArray" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes","name": "GetDataSubarraysType","fields":[{ "name": "uid", "type": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier" },{"name": "starts","type": { "type": "array", "items": "long" }, "default": []},{"name": "counts","type": { "type": "array", "items": "long" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes","name": "DataArrayMetadata","fields":[{ "name": "dimensions","type": { "type": "array", "items": "long" }},{"name": "preferredSubarrayDimensions","type": { "type": "array", "items": "long" }, "default": []},{ "name": "transportArrayType", "type": "Energistics.Etp.v12.Datatypes.AnyArrayType" },{ "name": "logicalArrayType", "type": "Energistics.Etp.v12.Datatypes.AnyLogicalArrayType" },{ "name": "storeLastWrite", "type": "long" },{ "name": "storeCreated", "type": "long" },{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes","name": "PutDataArraysType","fields":[{ "name": "uid", "type": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier" },{ "name": "array", "type": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArray" },{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes","name": "PutDataSubarraysType","fields":[{ "name": "uid", "type": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier" },{ "name": "data", "type": "Energistics.Etp.v12.Datatypes.AnyArray" },{ "name": "starts","type": { "type": "array", "items": "long" }},{ "name": "counts","type": { "type": "array", "items": "long" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes","name": "PutUninitializedDataArrayType","fields":[{ "name": "uid", "type": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier" },{ "name": "metadata", "type": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayMetadata" }]}"##,
    // Channel
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "IndexDirection","symbols":["Increasing","Decreasing","Unordered"]}"##,
    r##"{"type": "enum","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "PassDirection","symbols":["Up","HoldingSteady","Down"]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "PassIndexedDepth","fields":[{ "name": "pass", "type": "long" },{ "name": "direction", "type": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection" },{ "name": "depth", "type": "double" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "IndexValue","fields":[{"name": "item","type": ["null","long","double","Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"]}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "IndexInterval","fields":[{ "name": "startIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue" },{ "name": "endIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue" },{ "name": "uom", "type": "string" },{ "name": "depthDatum", "type": "string", "default": "" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "DataItem","fields":[{ "name": "channelId", "type": "long" },{ "name": "indexes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.IndexValue" }},{ "name": "value", "type": "Energistics.Etp.v12.Datatypes.DataValue" },{"name": "valueAttributes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.DataAttribute" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "FramePoint","fields":[{ "name": "value", "type": "Energistics.Etp.v12.Datatypes.DataValue" },{"name": "valueAttributes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.DataAttribute" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "FrameRow","fields":[{ "name": "indexes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.IndexValue" }},{ "name": "points","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.FramePoint" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "FrameChannelMetadataRecord","fields":[{ "name": "uri", "type": "string" },{ "name": "channelName", "type": "string" },{ "name": "dataKind", "type": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind" },{ "name": "uom", "type": "string" },{ "name": "depthDatum", "type": "string" },{ "name": "channelPropertyKindUri", "type": "string" },{ "name": "status", "type": "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind" },{ "name": "source", "type": "string" },{ "name": "axisVectorLengths","type": { "type": "array", "items": "int" }},{"name": "attributeMetadata","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.AttributeMetadataRecord" }, "default": []},{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "IndexMetadataRecord","fields":[{ "name": "indexKind", "type": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelIndexKind", "default": "DateTime" },{ "name": "interval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{ "name": "direction", "type": "Energistics.Etp.v12.Datatypes.ChannelData.IndexDirection", "default": "Increasing" },{ "name": "name", "type": "string", "default": "" },{ "name": "uom", "type": "string" },{ "name": "depthDatum", "type": "string", "default": "" },{ "name": "indexPropertyKindUri", "type": "string" },{ "name": "filterable", "type": "boolean", "default": true }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "ChannelMetadataRecord","fields":[{ "name": "uri", "type": "string" },{ "name": "id", "type": "long" },{ "name": "indexes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.IndexMetadataRecord" }},{ "name": "channelName", "type": "string" },{ "name": "dataKind", "type": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind" },{ "name": "uom", "type": "string" },{ "name": "depthDatum", "type": "string" },{ "name": "channelClassUri", "type": "string" },{ "name": "status", "type": "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind" },{ "name": "source", "type": "string" },{ "name": "axisVectorLengths","type": { "type": "array", "items": "int" }},{"name": "attributeMetadata","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.AttributeMetadataRecord" }, "default": []},{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "OpenChannelInfo","fields":[{ "name": "metadata", "type": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelMetadataRecord" },{ "name": "preferRealtime", "type": "boolean", "default": true },{ "name": "dataChanges", "type": "boolean", "default": true }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "ChannelChangeRequestInfo","fields":[{ "name": "sinceChangeTime", "type": "long" },{ "name": "channelIds","type": { "type": "array", "items": "long" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "ChannelSubscribeInfo","fields":[{ "name": "channelId", "type": "long" },{ "name": "startIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue" },{ "name": "dataChanges", "type": "boolean", "default": true },{ "name": "requestLatestIndexCount", "type": ["null", "int"] }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "ChannelRangeInfo","fields":[{ "name": "channelIds","type": { "type": "array", "items": "long" }},{ "name": "interval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{"name": "secondaryIntervals","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.ChannelData","name": "TruncateInfo","fields":[{ "name": "channelId", "type": "long" },{ "name": "newEndIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue" }]}"##,
    // Object & Resource
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ObjectPart","fields":[{ "name": "uid", "type": "string" },{ "name": "data", "type": "bytes" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ChangeAnnotation","fields":[{ "name": "changeTime", "type": "long" },{ "name": "interval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "PutResponse","fields":[{"name": "createdContainedObjectUris","type": { "type": "array", "items": "string" }, "default": []},{"name": "deletedContainedObjectUris","type": { "type": "array", "items": "string" }, "default": []},{"name": "joinedContainedObjectUris","type": { "type": "array", "items": "string" }, "default": []},{"name": "unjoinedContainedObjectUris","type": { "type": "array", "items": "string" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ContextInfo","fields":[{ "name": "uri", "type": "string" },{ "name": "depth", "type": "int" },{"name": "dataObjectTypes","type": { "type": "array", "items": "string" }, "default": []},{ "name": "navigableEdges", "type": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind" },{ "name": "includeSecondaryTargets", "type": "boolean", "default": false },{ "name": "includeSecondarySources", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "DeletedResource","fields":[{ "name": "uri", "type": "string" },{ "name": "deletedTime", "type": "long" },{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "Dataspace","fields":[{ "name": "uri", "type": "string" },{ "name": "path", "type": "string", "default": "" },{ "name": "storeLastWrite", "type": "long" },{ "name": "storeCreated", "type": "long" },{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "Edge","fields":[{ "name": "sourceUri", "type": "string" },{ "name": "targetUri", "type": "string" },{ "name": "relationshipKind", "type": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind" },{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "SupportedType","fields":[{ "name": "dataObjectType", "type": "string" },{ "name": "objectCount", "type": ["null", "int"] },{ "name": "relationshipKind", "type": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "Resource","fields":[{ "name": "uri", "type": "string" },{"name": "alternateUris","type": { "type": "array", "items": "string" }, "default": []},{ "name": "name", "type": "string" },{ "name": "sourceCount", "type": ["null", "int"], "default": null },{ "name": "targetCount", "type": ["null", "int"], "default": null },{ "name": "lastChanged", "type": "long" },{ "name": "storeLastWrite", "type": "long" },{ "name": "storeCreated", "type": "long" },{ "name": "activeStatus", "type": "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind" },{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "DataObject","fields":[{ "name": "resource", "type": "Energistics.Etp.v12.Datatypes.Object.Resource" },{ "name": "format", "type": "string", "default": "xml" },{ "name": "blobId", "type": ["null", "Energistics.Etp.v12.Datatypes.Uuid"] },{ "name": "data", "type": "bytes", "default": "" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ObjectChange","fields":[{ "name": "changeKind", "type": "Energistics.Etp.v12.Datatypes.Object.ObjectChangeKind" },{ "name": "changeTime", "type": "long" },{ "name": "dataObject", "type": "Energistics.Etp.v12.Datatypes.Object.DataObject" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "SubscriptionInfo","fields":[{ "name": "context", "type": "Energistics.Etp.v12.Datatypes.Object.ContextInfo" },{ "name": "scope", "type": "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "includeObjectData", "type": "boolean" },{ "name": "format", "type": "string", "default": "xml" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "ChangeResponseInfo","fields":[{ "name": "responseTimestamp", "type": "long" },{"name": "changes","type": { "type": "map", "values": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.ChangeAnnotation" } }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes.Object","name": "PartsMetadataInfo","fields":[{ "name": "uri", "type": "string" },{ "name": "name", "type": "string" },{ "name": "index", "type": "Energistics.Etp.v12.Datatypes.ChannelData.IndexMetadataRecord" },{"name": "customData","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    // DataTypes With Dependencies
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "MessageHeaderExtension","fields":[{"name": "extension","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "SupportedDataObject","fields":[{ "name": "qualifiedType", "type": "string" },{"name": "dataObjectCapabilities","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "SupportedProtocol","fields":[{ "name": "protocol", "type": "int" },{ "name": "protocolVersion", "type": "Energistics.Etp.v12.Datatypes.Version" },{ "name": "role", "type": "string" },{"name": "protocolCapabilities","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Datatypes","name": "ServerCapabilities","fields":[{ "name": "applicationName", "type": "string" },{ "name": "applicationVersion", "type": "string" },{ "name": "contactInformation", "type": "Energistics.Etp.v12.Datatypes.Contact" },{"name": "supportedCompression","type": { "type": "array", "items": "string" }, "default": []},{"name": "supportedEncodings","type": { "type": "array", "items": "string" }, "default": ["binary"]},{"name": "supportedFormats","type": { "type": "array", "items": "string" }, "default": ["xml"]},{ "name": "supportedDataObjects","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedDataObject" }},{ "name": "supportedProtocols","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedProtocol" }},{"name": "endpointCapabilities","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    //
    // ------------------------------------------------------
    // Protocols
    // ------------------------------------------------------
    // 0 - CORE
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "AuthorizeResponse","protocol": "0","messageType": "7","senderRole": "client,server","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "success", "type": "boolean" },{ "name": "challenges","type": { "type": "array", "items": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "ProtocolException","protocol": "0","messageType": "1000","senderRole": "*","protocolRoles": "client, server","multipartFlag": true,"fields":[{ "name": "error", "type": ["null", "Energistics.Etp.v12.Datatypes.ErrorInfo"] },{"name": "errors","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.ErrorInfo" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "RequestSession","protocol": "0","messageType": "1","senderRole": "client","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "applicationName", "type": "string" },{ "name": "applicationVersion", "type": "string" },{ "name": "clientInstanceId", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "requestedProtocols","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedProtocol" }},{ "name": "supportedDataObjects","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedDataObject" }},{"name": "supportedCompression","type": { "type": "array", "items": "string" }, "default": []},{"name": "supportedFormats","type": { "type": "array", "items": "string" }, "default": ["xml"]},{ "name": "currentDateTime", "type": "long" },{ "name": "earliestRetainedChangeTime", "type": "long" },{ "name": "serverAuthorizationRequired", "type": "boolean", "default": false },{"name": "endpointCapabilities","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    //r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "RequestSession","protocol": "0","messageType": "1","senderRole": "client","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "applicationName", "type": "string" },{ "name": "applicationVersion", "type": "string" },{ "name": "clientInstanceId", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "requestedProtocols","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedProtocol" }},{ "name": "supportedDataObjects","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedDataObject" }},{"name": "supportedCompression","type": { "type": "array", "items": "string" }, "default": []},{"name": "supportedFormats","type": { "type": "array", "items": "string" }, "default": ["xml"]},{ "name": "currentDateTime", "type": "long" },{ "name": "earliestRetainedChangeTime", "type": "long" },{ "name": "serverAuthorizationRequired", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "OpenSession","protocol": "0","messageType": "2","senderRole": "server","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "applicationName", "type": "string" },{ "name": "applicationVersion", "type": "string" },{ "name": "serverInstanceId", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "supportedProtocols","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedProtocol" }},{ "name": "supportedDataObjects","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.SupportedDataObject" }},{ "name": "supportedCompression", "type": "string", "default": "" },{"name": "supportedFormats","type": { "type": "array", "items": "string" }, "default": ["xml"]},{ "name": "currentDateTime", "type": "long" },{ "name": "earliestRetainedChangeTime", "type": "long" },{ "name": "sessionId", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{"name": "endpointCapabilities","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "Authorize","protocol": "0","messageType": "6","senderRole": "client,server","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "authorization", "type": "string" },{"name": "supplementalAuthorization","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "Ping","protocol": "0","messageType": "8","senderRole": "client,server","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "currentDateTime", "type": "long" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "CloseSession","protocol": "0","messageType": "5","senderRole": "client,server","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "reason", "type": "string", "default": "" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Core","name": "Pong","protocol": "0","messageType": "9","senderRole": "client,server","protocolRoles": "client, server","multipartFlag": false,  "fields":[{ "name": "currentDateTime", "type": "long" }]}"##,
    // ------------------------------------------------------
    // 1 - ChannelStreaming
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelStreaming","name": "TruncateChannels_CS","protocol": "1","messageType": "5","senderRole": "producer","protocolRoles": "producer,consumer","multipartFlag": false,  "fields":[{ "name": "channels","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.TruncateInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelStreaming","name": "ChannelMetadata","protocol": "1","messageType": "1","senderRole": "producer","protocolRoles": "producer,consumer","multipartFlag": false,  "fields":[{ "name": "channels","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelMetadataRecord" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelStreaming","name": "ChannelData_CS","protocol": "1","messageType": "2","senderRole": "producer","protocolRoles": "producer,consumer","multipartFlag": false,  "fields":[{ "name": "data","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.DataItem" }}]}"##,
    // ------------------------------------------------------
    // 2 - ChannelDataFrame
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame","name": "GetFrameResponseHeader","protocol": "2","messageType": "4","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "channelUris","type": { "type": "array", "items": "string" }},{ "name": "indexes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.IndexMetadataRecord" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame","name": "CancelGetFrame","protocol": "2","messageType": "5","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame","name": "GetFrameMetadataResponse","protocol": "2","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "uri", "type": "string" },{ "name": "indexes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.IndexMetadataRecord" }},{ "name": "channels","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.FrameChannelMetadataRecord" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame","name": "GetFrameMetadata","protocol": "2","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "includeAllChannelSecondaryIndexes", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame","name": "GetFrameResponseRows","protocol": "2","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "frame","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.FrameRow" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame","name": "GetFrame","protocol": "2","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "includeAllChannelSecondaryIndexes", "type": "boolean", "default": false },{ "name": "requestedInterval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{"name": "requestedSecondaryIntervals","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" }, "default": []}]}"##,
    // ------------------------------------------------------
    // 3 - Discovery
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Discovery","name": "GetResources","protocol": "3","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "context", "type": "Energistics.Etp.v12.Datatypes.Object.ContextInfo" },{ "name": "scope", "type": "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind" },{ "name": "countObjects", "type": "boolean", "default": false },{ "name": "storeLastWriteFilter", "type": ["null", "long"] },{ "name": "activeStatusFilter", "type": ["null", "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind"] },{ "name": "includeEdges", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Discovery","name": "GetResourcesEdgesResponse","protocol": "3","messageType": "7","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "edges","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.Edge" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Discovery","name": "GetDeletedResources","protocol": "3","messageType": "5","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "dataspaceUri", "type": "string" },{ "name": "deleteTimeFilter", "type": ["null", "long"] },{"name": "dataObjectTypes","type": { "type": "array", "items": "string" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Discovery","name": "GetResourcesResponse","protocol": "3","messageType": "4","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "resources","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.Resource" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Discovery","name": "GetDeletedResourcesResponse","protocol": "3","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "deletedResources","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.DeletedResource" }, "default": []}]}"##,
    // ------------------------------------------------------
    // 4 - Store
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Store","name": "GetDataObjects","protocol": "4","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "uris","type": { "type": "map", "values": "string" }},{ "name": "format", "type": "string", "default": "xml" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Store","name": "DeleteDataObjects","protocol": "4","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "uris","type": { "type": "map", "values": "string" }},{ "name": "pruneContainedObjects", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Store","name": "Chunk","protocol": "4","messageType": "8","senderRole": "store,customer","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "blobId", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "data", "type": "bytes" },{ "name": "final", "type": "boolean" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Store","name": "GetDataObjectsResponse","protocol": "4","messageType": "4","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "dataObjects","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.DataObject" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Store","name": "DeleteDataObjectsResponse","protocol": "4","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "deletedUris","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.ArrayOfString" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Store","name": "PutDataObjects","protocol": "4","messageType": "2","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "dataObjects","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.DataObject" }},{ "name": "pruneContainedObjects", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Store","name": "PutDataObjectsResponse","protocol": "4","messageType": "9","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.PutResponse" }}]}"##,
    // ------------------------------------------------------
    // 5 - StoreNotification
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "Chunk_SN","protocol": "5","messageType": "9","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "blobId", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "data", "type": "bytes" },{ "name": "final", "type": "boolean" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "ObjectAccessRevoked","protocol": "5","messageType": "5","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "changeTime", "type": "long" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "ObjectActiveStatusChanged","protocol": "5","messageType": "11","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "activeStatus", "type": "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind" },{ "name": "changeTime", "type": "long" },{ "name": "resource", "type": "Energistics.Etp.v12.Datatypes.Object.Resource" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "ObjectChanged","protocol": "5","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "change", "type": "Energistics.Etp.v12.Datatypes.Object.ObjectChange" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "UnsolicitedStoreNotifications","protocol": "5","messageType": "8","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "subscriptions","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.SubscriptionInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "SubscribeNotificationsResponse","protocol": "5","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "ObjectDeleted","protocol": "5","messageType": "3","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "changeTime", "type": "long" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "UnsubscribeNotifications","protocol": "5","messageType": "4","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "SubscriptionEnded","protocol": "5","messageType": "7","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "reason", "type": "string" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreNotification","name": "SubscribeNotifications","protocol": "5","messageType": "6","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "request","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.SubscriptionInfo" }}]}"##,
    // ------------------------------------------------------
    // 6 - GrowingObject
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "DeletePartsResponse","protocol": "6","messageType": "11","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "DeleteParts","protocol": "6","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{"name": "uids","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetGrowingDataObjectsHeaderResponse","protocol": "6","messageType": "15","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "dataObjects","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.DataObject" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetChangeAnnotationsResponse_GO","protocol": "6","messageType": "20","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "changes","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.ChangeResponseInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetPartsResponse","protocol": "6","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "uri", "type": "string" },{ "name": "format", "type": "string", "default": "xml" },{"name": "parts","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.ObjectPart" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "ReplacePartsByRange","protocol": "6","messageType": "7","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "uri", "type": "string" },{ "name": "deleteInterval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{ "name": "includeOverlappingIntervals", "type": "boolean" },{ "name": "format", "type": "string", "default": "xml" },{ "name": "parts","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.ObjectPart" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetPartsByRange","protocol": "6","messageType": "4","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "format", "type": "string", "default": "xml" },{ "name": "indexInterval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{ "name": "includeOverlappingIntervals", "type": "boolean" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetPartsMetadataResponse","protocol": "6","messageType": "9","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "metadata","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.PartsMetadataInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "PutGrowingDataObjectsHeader","protocol": "6","messageType": "16","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataObjects","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.DataObject" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "PutGrowingDataObjectsHeaderResponse","protocol": "6","messageType": "17","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "PutParts","protocol": "6","messageType": "5","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "format", "type": "string", "default": "xml" },{"name": "parts","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.ObjectPart" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetPartsByRangeResponse","protocol": "6","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "uri", "type": "string" },{ "name": "format", "type": "string", "default": "xml" },{ "name": "parts","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.ObjectPart" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetParts","protocol": "6","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "format", "type": "string", "default": "xml" },{"name": "uids","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetChangeAnnotations_GO","protocol": "6","messageType": "19","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "sinceChangeTime", "type": "long" },{"name": "uris","type": { "type": "map", "values": "string" }},{ "name": "latestOnly", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "PutPartsResponse","protocol": "6","messageType": "13","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetPartsMetadata","protocol": "6","messageType": "8","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "uris","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObject","name": "GetGrowingDataObjectsHeader","protocol": "6","messageType": "14","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "uris","type": { "type": "map", "values": "string" }},{ "name": "format", "type": "string", "default": "xml" }]}"##,
    // ------------------------------------------------------
    // 7 - GrowingObjectNotification
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "SubscribePartNotificationsResponse","protocol": "7","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "PartsReplacedByRange","protocol": "7","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "uri", "type": "string" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "changeTime", "type": "long" },{ "name": "deletedInterval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{ "name": "includeOverlappingIntervals", "type": "boolean" },{ "name": "format", "type": "string", "default": "" },{ "name": "parts","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.ObjectPart" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "UnsolicitedPartNotifications","protocol": "7","messageType": "9","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "subscriptions","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.SubscriptionInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "PartsChanged","protocol": "7","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "changeKind", "type": "Energistics.Etp.v12.Datatypes.Object.ObjectChangeKind" },{ "name": "changeTime", "type": "long" },{ "name": "format", "type": "string", "default": "" },{ "name": "parts","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.ObjectPart" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "PartSubscriptionEnded","protocol": "7","messageType": "8","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "reason", "type": "string" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "UnsubscribePartNotification","protocol": "7","messageType": "4","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "PartsDeleted","protocol": "7","messageType": "3","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "changeTime", "type": "long" },{ "name": "uids","type": { "type": "array", "items": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification","name": "SubscribePartNotifications","protocol": "7","messageType": "7","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "request","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.SubscriptionInfo" }}]}"##,
    // ------------------------------------------------------
    // 9 - DataArray
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "PutUninitializedDataArrays","protocol": "9","messageType": "9","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataArrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.PutUninitializedDataArrayType" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "PutDataArraysResponse","protocol": "9","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "PutDataSubarrays","protocol": "9","messageType": "5","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataSubarrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.PutDataSubarraysType" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "GetDataArraysResponse","protocol": "9","messageType": "1","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "dataArrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArray" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "PutUninitializedDataArraysResponse","protocol": "9","messageType": "12","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "GetDataArrays","protocol": "9","messageType": "2","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataArrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "PutDataSubarraysResponse","protocol": "9","messageType": "11","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "PutDataArrays","protocol": "9","messageType": "4","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataArrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.PutDataArraysType" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "GetDataSubarraysResponse","protocol": "9","messageType": "8","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "dataSubarrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArray" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "GetDataSubarrays","protocol": "9","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataSubarrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.GetDataSubarraysType" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "GetDataArrayMetadata","protocol": "9","messageType": "6","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataArrays","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DataArray","name": "GetDataArrayMetadataResponse","protocol": "9","messageType": "7","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "arrayMetadata","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayMetadata" }, "default": {}}]}"##,
    // ------------------------------------------------------
    // 13 - DiscoveryQuery
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DiscoveryQuery","name": "FindResourcesResponse","protocol": "13","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "resources","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.Resource" }, "default": []},{ "name": "serverSortOrder", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.DiscoveryQuery","name": "FindResources","protocol": "13","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "context", "type": "Energistics.Etp.v12.Datatypes.Object.ContextInfo" },{ "name": "scope", "type": "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind" },{ "name": "storeLastWriteFilter", "type": ["null", "long"] },{ "name": "activeStatusFilter", "type": ["null", "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind"] }]}"##,
    // ------------------------------------------------------
    // 14 - StoreQuery
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreQuery","name": "Chunk_SQ","protocol": "14","messageType": "3","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "blobId", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "data", "type": "bytes" },{ "name": "final", "type": "boolean" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreQuery","name": "FindDataObjectsResponse","protocol": "14","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "dataObjects","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.DataObject" }, "default": []},{ "name": "serverSortOrder", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.StoreQuery","name": "FindDataObjects","protocol": "14","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "context", "type": "Energistics.Etp.v12.Datatypes.Object.ContextInfo" },{ "name": "scope", "type": "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind" },{ "name": "storeLastWriteFilter", "type": ["null", "long"] },{ "name": "activeStatusFilter", "type": ["null", "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind"] },{ "name": "format", "type": "string", "default": "xml" }]}"##,
    // ------------------------------------------------------
    // 16 - GrowingObjectQuery
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectQuery","name": "FindPartsResponse","protocol": "16","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "uri", "type": "string" },{ "name": "serverSortOrder", "type": "string" },{ "name": "format", "type": "string", "default": "xml" },{"name": "parts","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.ObjectPart" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.GrowingObjectQuery","name": "FindParts","protocol": "16","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "format", "type": "string", "default": "xml" }]}"##,
    // ------------------------------------------------------
    // 18 - Transaction
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Transaction","name": "RollbackTransactionResponse","protocol": "18","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "transactionUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "successful", "type": "boolean", "default": true },{ "name": "failureReason", "type": "string", "default": "" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Transaction","name": "CommitTransaction","protocol": "18","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "transactionUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Transaction","name": "StartTransaction","protocol": "18","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "readOnly", "type": "boolean" },{ "name": "message", "type": "string", "default": "" },{"name": "dataspaceUris","type": { "type": "array", "items": "string" }, "default": [""]}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Transaction","name": "CommitTransactionResponse","protocol": "18","messageType": "5","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "transactionUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "successful", "type": "boolean", "default": true },{ "name": "failureReason", "type": "string", "default": "" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Transaction","name": "RollbackTransaction","protocol": "18","messageType": "4","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "transactionUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Transaction","name": "StartTransactionResponse","protocol": "18","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "transactionUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "successful", "type": "boolean", "default": true },{ "name": "failureReason", "type": "string", "default": "" }]}"##,
    // ------------------------------------------------------
    // 21 - ChannelSubscribe
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "GetRangesResponse","protocol": "21","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "data","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.DataItem" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "ChannelsTruncated","protocol": "21","messageType": "13","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "channels","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.TruncateInfo" }},{ "name": "changeTime", "type": "long" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "SubscribeChannels","protocol": "21","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "channels","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelSubscribeInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "GetChangeAnnotationsResponse","protocol": "21","messageType": "15","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "changes","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.ChangeResponseInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "UnsubscribeChannels","protocol": "21","messageType": "7","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "channelIds","type": { "type": "map", "values": "long" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "GetChannelMetadataResponse","protocol": "21","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "metadata","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelMetadataRecord" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "GetChannelMetadata","protocol": "21","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "uris","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "GetRanges","protocol": "21","messageType": "9","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" },{ "name": "channelRanges","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelRangeInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "ChannelData","protocol": "21","messageType": "4","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "data","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.DataItem" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "CancelGetRanges","protocol": "21","messageType": "11","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "requestUuid", "type": "Energistics.Etp.v12.Datatypes.Uuid" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "RangeReplaced","protocol": "21","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "changeTime", "type": "long" },{ "name": "channelIds","type": { "type": "array", "items": "long" }},{ "name": "changedInterval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{ "name": "data","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.DataItem" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "GetChangeAnnotations","protocol": "21","messageType": "14","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "channels","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelChangeRequestInfo" }},{ "name": "latestOnly", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "SubscribeChannelsResponse","protocol": "21","messageType": "12","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe","name": "SubscriptionsStopped","protocol": "21","messageType": "8","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "reason", "type": "string" },{"name": "channelIds","type": { "type": "map", "values": "long" }, "default": {}}]}"##,
    // ------------------------------------------------------
    // 22 - ChannelDataLoad
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "ChannelsClosed","protocol": "22","messageType": "7","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "reason", "type": "string" },{"name": "id","type": { "type": "map", "values": "long" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "TruncateChannels","protocol": "22","messageType": "9","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "channels","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.ChannelData.TruncateInfo" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "ReplaceRange","protocol": "22","messageType": "6","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": true,"fields":[{ "name": "changedInterval", "type": "Energistics.Etp.v12.Datatypes.Object.IndexInterval" },{ "name": "channelIds","type": { "type": "array", "items": "long" }},{ "name": "data","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.DataItem" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "ChannelData_CDL","protocol": "22","messageType": "4","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "data","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.ChannelData.DataItem" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "OpenChannels","protocol": "22","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "uris","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "ReplaceRangeResponse","protocol": "22","messageType": "8","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "channelChangeTime", "type": "long" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "TruncateChannelsResponse","protocol": "22","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "channelsTruncatedTime","type": { "type": "map", "values": "long" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "OpenChannelsResponse","protocol": "22","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "channels","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.ChannelData.OpenChannelInfo" }, "default": {}}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad","name": "CloseChannels","protocol": "22","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "id","type": { "type": "map", "values": "long" }}]}"##,
    // ------------------------------------------------------
    // 24 - Dataspace
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Dataspace","name": "PutDataspacesResponse","protocol": "24","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Dataspace","name": "GetDataspacesResponse","protocol": "24","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "dataspaces","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.Dataspace" }, "default": []}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Dataspace","name": "PutDataspaces","protocol": "24","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "dataspaces","type": { "type": "map", "values": "Energistics.Etp.v12.Datatypes.Object.Dataspace" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Dataspace","name": "GetDataspaces","protocol": "24","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "storeLastWriteFilter", "type": ["null", "long"] }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Dataspace","name": "DeleteDataspacesResponse","protocol": "24","messageType": "5","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "success","type": { "type": "map", "values": "string" }}]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.Dataspace","name": "DeleteDataspaces","protocol": "24","messageType": "4","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{"name": "uris","type": { "type": "map", "values": "string" }}]}"##,
    // ------------------------------------------------------
    // 25 - SupportedTypes
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.SupportedTypes","name": "GetSupportedTypes","protocol": "25","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "uri", "type": "string" },{ "name": "scope", "type": "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind" },{ "name": "returnEmptyTypes", "type": "boolean", "default": false },{ "name": "countObjects", "type": "boolean", "default": false }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.SupportedTypes","name": "GetSupportedTypesResponse","protocol": "25","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": true,"fields":[{"name": "supportedTypes","type": { "type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.SupportedType" }, "default": []}]}"##,
    // ------------------------------------------------------
    // 2100 - PRIVATE - WitsmlSoap
    // ------------------------------------------------------
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_UpdateInStore","protocol": "2100","messageType": "13","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "WMLtypeIn", "type": "string" },{ "name": "XMLin", "type": "string" },{ "name": "OptionsIn", "type": "string" },{ "name": "CapabilitiesIn", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_DeleteFromStore","protocol": "2100","messageType": "3","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "WMLtypeIn", "type": "string" },{ "name": "XMLin", "type": "string" },{ "name": "OptionsIn", "type": "string" },{ "name": "CapabilitiesIn", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetCapResponse","protocol": "2100","messageType": "8","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "Result", "type": "int" },{ "name": "CapabilitiesOut", "type": "string" },{ "name": "SuppMsgOut", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetFromStoreResponse","protocol": "2100","messageType": "10","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "Result", "type": "int" },{ "name": "XMLout", "type": "string" },{ "name": "SuppMsgOut", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_UpdateInStoreResponse","protocol": "2100","messageType": "14","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "Result", "type": "int" },{ "name": "SuppMsgOut", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_AddToStoreResponse","protocol": "2100","messageType": "2","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "Result", "type": "int" },{ "name": "SuppMsgOut", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetFromStore","protocol": "2100","messageType": "9","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "WMLtypeIn", "type": "string" },{ "name": "XMLin", "type": "string" },{ "name": "OptionsIn", "type": "string" },{ "name": "CapabilitiesIn", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_DeleteFromStoreResponse","protocol": "2100","messageType": "4","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "Result", "type": "int" },{ "name": "SuppMsgOut", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetCap","protocol": "2100","messageType": "7","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "OptionsIn", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetVersionResponse","protocol": "2100","messageType": "12","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "Result", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetVersion","protocol": "2100","messageType": "11","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_AddToStore","protocol": "2100","messageType": "1","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "WMLtypeIn", "type": "string" },{ "name": "XMLin", "type": "string" },{ "name": "OptionsIn", "type": "string" },{ "name": "CapabilitiesIn", "type": "string" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetBaseMsg","protocol": "2100","messageType": "5","senderRole": "customer","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "ReturnValueIn", "type": "int" }]}"##,
    r##"{"type": "record","namespace": "Energistics.Etp.v12.Protocol.WitsmlSoap","name": "WMLS_GetBaseMsgResponse","protocol": "2100","messageType": "6","senderRole": "store","protocolRoles": "store,customer","multipartFlag": false,  "fields":[{ "name": "Result", "type": "string" }]}"##,
];
