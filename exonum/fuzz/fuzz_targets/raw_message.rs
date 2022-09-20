// Copyright 2020 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate exonum;
extern crate exonum_merkledb;

use exonum::messages::{CoreMessage, Precommit, SignedMessage, Verified};
use exonum_merkledb::BinaryValue;

fn fuzz_target(data: &[u8]) -> Option<()> {
    let msg = SignedMessage::from_bytes(data.into()).ok()?;
    let _: Result<Verified<Precommit>, _> = msg.clone().into_verified();
    let _: Result<Verified<CoreMessage>, _> = msg.into_verified();

    None
}

fuzz_target!(|data| {
    fuzz_target(data);
});
