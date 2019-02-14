//
// Copyright 2019 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Running average example for Project Oak.
//!
//! This shows how an Oak Mode can maintain some internal state across multiple invocations.
//!
//! Clients invoke the module by providing a string representation of a non-negative number
//! expressed in base 10, and get back a string representation of the accumulated average value up
//! to and including the value provided in the request.

extern crate oak;

use std::io::{Read, Write};

#[no_mangle]
pub extern "C" fn oak_initialize() {
    oak::print("Oak initialize\n");
}

#[no_mangle]
pub extern "C" fn oak_finalize() {
    oak::print("Oak finalize\n");
}

static mut sum: u64 = 0;
static mut count: u64 = 0;

#[no_mangle]
pub extern "C" fn oak_invoke() {
    oak::print("Oak invoke\n");

    let mut in1 = oak::get_input();
    let mut s = String::new();
    in1.read_to_string(&mut s).expect("could not read string");
    let val = s.parse::<u64>().expect("could not parse value");
    let current_average;
    // TODO: Expose a more ergonomic API that avoids global mutable state.
    unsafe {
        sum += val;
        count += 1;
        current_average = sum / count;
    }
    oak::get_output().write(format!("{}", current_average).as_bytes());
}