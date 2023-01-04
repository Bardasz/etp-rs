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

use crate::{headerflags::*, helpers::time_to_etp, schema::*, schema_gen::*};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------------------
// Extensions and Implementations to some of the Message Structs (such as sane defaults)
// ------------------------------------------------------------------------------------------------------------

impl MessageHeader {
    pub fn get_flags(&self) -> MessageHeaderFlags {
        return MessageHeaderFlags::parse(self.message_flags);
    }

    pub fn msgtype(&self) -> (usize, usize) {
        return (self.protocol as usize, self.message_type as usize);
    }
}

impl SupportedProtocol {
    fn default_core(role: Role) -> SupportedProtocol {
        SupportedProtocol {
            protocol: Protocol::Core as i32,
            protocol_version: ETP12VERSION,
            role: role.to_string(),
            protocol_capabilities: HashMap::new(),
        }
    }

    fn default_discovery(role: Role) -> SupportedProtocol {
        SupportedProtocol {
            protocol: Protocol::Discovery as i32,
            protocol_version: ETP12VERSION,
            role: role.to_string(),
            protocol_capabilities: HashMap::new(),
        }
    }

    fn default_store(role: Role) -> SupportedProtocol {
        SupportedProtocol {
            protocol: Protocol::Store as i32,
            protocol_version: ETP12VERSION,
            role: role.to_string(),
            protocol_capabilities: HashMap::new(),
        }
    }
}

// Some Sane defaults for basic connection.
// Assumes a CLIENT connecting to a Server/Store.  Would need different settings for a Server.

impl Default for RequestSession {
    fn default() -> RequestSession {
        let protocols = vec![
            SupportedProtocol::default_core(Role::Server),
            SupportedProtocol::default_discovery(Role::Store),
            SupportedProtocol::default_store(Role::Store),
        ];

        let now = SystemTime::now();

        RequestSession {
            application_name: "etp-rs Client Library Application".to_string(),
            application_version: "0.1".to_string(),
            client_instance_id: *Uuid::new_v4().as_bytes(),
            requested_protocols: protocols,
            supported_data_objects: vec![],
            supported_compression: vec!["gzip".to_string()],
            supported_formats: vec!["xml".to_string(), "json".to_string()],
            current_date_time: time_to_etp(now),
            earliest_retained_change_time: time_to_etp(now),
            server_authorization_required: false,
            endpoint_capabilities: HashMap::new(),
        }
    }
}

impl RequestSession {
    pub fn default_protocols(protocols: Vec<SupportedProtocol>) -> RequestSession {
        let now = SystemTime::now();
        RequestSession {
            application_name: "etp-rs Client Library Application".to_string(),
            application_version: "0.1".to_string(),
            client_instance_id: *Uuid::new_v4().as_bytes(),
            requested_protocols: protocols,
            supported_data_objects: vec![],
            supported_compression: vec!["gzip".to_string()],
            supported_formats: vec!["xml".to_string(), "json".to_string()],
            current_date_time: time_to_etp(now),
            earliest_retained_change_time: time_to_etp(now),
            server_authorization_required: false,
            endpoint_capabilities: HashMap::new(),
        }
    }
}

impl Default for OpenSession {
    fn default() -> OpenSession {
        let protocols = vec![
            SupportedProtocol::default_core(Role::Client),
            SupportedProtocol::default_discovery(Role::Client),
            SupportedProtocol::default_store(Role::Store),
        ];

        let now = SystemTime::now();

        OpenSession {
            application_name: "etp-rs Client Library Application".to_string(),
            application_version: "0.1".to_string(),
            server_instance_id: *Uuid::new_v4().as_bytes(),
            supported_protocols: protocols,
            supported_data_objects: vec![],
            supported_compression: "gzip".to_string(),
            supported_formats: vec!["xml".to_string(), "json".to_string()],
            current_date_time: time_to_etp(now),
            earliest_retained_change_time: time_to_etp(now),
            endpoint_capabilities: HashMap::new(),
            session_id: *Uuid::new_v4().as_bytes(),
        }
    }
}
