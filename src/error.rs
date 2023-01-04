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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Avro Error {0}")]
    AvroError(apache_avro::Error),

    #[error("IO Error {0}")]
    IoError(std::io::Error),

    #[error("IO Error {0}")]
    WSError(tungstenite::Error),

    #[error("Unsupported Websocket messages received")]
    UnsupportedWSMessage,

    #[error("ProtocolException: {0}, {1}")]
    ProtocolException(i32, String),

    #[error("URL Parse Error: {0}")]
    ParseError(url::ParseError),

    #[error("{0}")]
    Simple(String),
}

impl From<apache_avro::Error> for Error {
    fn from(err: apache_avro::Error) -> Self {
        Error::AvroError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Error::WSError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::ParseError(err)
    }
}
