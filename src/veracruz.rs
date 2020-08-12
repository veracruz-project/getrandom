//! Implementation for Veracruz, building on top of libveracruz.
//!
//! Copyright (c) Arm Limited, 2020.  All rights reserved (r).

use crate::error;

use libveracruz::host;

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), error::Error> {
    match host::getrandom(dest) {
        host::HCallReturnCode::Success(_) => Ok(()),
        host::HCallReturnCode::ErrorServiceUnavailable => Err(error::UNSUPPORTED),
        _otherwise => Err(error::UNKNOWN_IO_ERROR), // Hmm?
    }
}
