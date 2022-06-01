// Copyright 2021 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! NOOP implementation to allow compilation of WASM without `getrandom` support.
use crate::Error;

pub fn getrandom_inner(_dest: &mut [u8]) -> Result<(), Error> {
    panic!("getrandom_inner: not supported");
}
