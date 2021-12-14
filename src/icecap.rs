// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for WASI
use crate::Error;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;

// HACK
// Placeholder generator with a fixed seed and a period of 2**61
static STATE: AtomicU64 = AtomicU64::new(0);

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    for b in dest {
        let state = STATE.fetch_add(1, Ordering::SeqCst);
        *b = state.to_ne_bytes()[(state & 0b111) as usize];
    }
    Ok(())
}
