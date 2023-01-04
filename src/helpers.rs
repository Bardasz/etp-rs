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

use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_to_etp(time: SystemTime) -> i64 {
    let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    return in_ms.try_into().unwrap();
}
