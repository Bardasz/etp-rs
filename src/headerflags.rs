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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MessageHeaderFlags {
    pub finalmsg: bool,  // Final part of a Multi Part Response
    pub compress: bool,  // This message body is compressed
    pub reqack: bool,    // Request acknowledgement of this Message
    pub extension: bool, // Indicates this message has an extension between header and body.
}

impl Default for MessageHeaderFlags {
    fn default() -> MessageHeaderFlags {
        MessageHeaderFlags {
            finalmsg: true,
            compress: true,
            reqack: false,
            extension: false,
        }
    }
}

impl MessageHeaderFlags {
    pub fn not_final() -> MessageHeaderFlags {
        MessageHeaderFlags {
            finalmsg: false,
            compress: true,
            reqack: false,
            extension: false,
        }
    }

    pub fn parse(flags: i32) -> MessageHeaderFlags {
        return MessageHeaderFlags {
            finalmsg: (flags & MSG_FLAG_FINAL) != 0,
            compress: (flags & MSG_FLAG_COMPRESS) != 0,
            reqack: (flags & MSG_FLAG_REQACK) != 0,
            extension: (flags & MSG_FLAG_EXTENSION) != 0,
        };
    }

    pub fn as_i32(&self) -> i32 {
        let mut v: i32 = 0;

        if self.finalmsg {
            v = v | MSG_FLAG_FINAL;
        }

        if self.compress {
            v = v | MSG_FLAG_COMPRESS;
        }

        if self.reqack {
            v = v | MSG_FLAG_REQACK;
        }

        if self.extension {
            v = v | MSG_FLAG_EXTENSION;
        }

        return v;
    }
}

pub const MSG_FLAG_FINAL: i32 = 0x02;
pub const MSG_FLAG_COMPRESS: i32 = 0x08;
pub const MSG_FLAG_REQACK: i32 = 0x10;
pub const MSG_FLAG_EXTENSION: i32 = 0x20;

#[test]
fn test_flags_roundtrip() {
    let flags = MessageHeaderFlags {
        finalmsg: true,
        compress: true,
        reqack: false,
        extension: true,
    };

    let fin = flags.as_i32();
    assert_eq!(fin, 0x2a);
    let fout = MessageHeaderFlags::parse(fin);

    assert_eq!(fout, flags);
}
