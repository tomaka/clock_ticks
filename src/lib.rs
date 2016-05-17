// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::time;

fn duration_since_epoch() -> time::Duration {
    return match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(dur) => dur,
        Err(err) => err.duration(),
    };
}

/**
 * Returns the current value of a high-resolution performance counter
 * in nanoseconds since an unspecified epoch.
 */
pub fn precise_time_ns() -> u64 {
    let dur = duration_since_epoch();
    return dur.as_secs() * 1000_000_000 + dur.subsec_nanos() as u64;
}

/**
 * Returns the current value of a high-resolution performance counter
 * in seconds since an unspecified epoch.
 */
pub fn precise_time_s() -> f64 {
    let dur = duration_since_epoch();
    return dur.as_secs() as f64 + (dur.subsec_nanos() as f64 / 1000_000_000.);
}

/**
 * Returns the current value of a high-resolution performance counter
 * in milliseconds since an unspecified epoch.
 */
pub fn precise_time_ms() -> u64 {
    let dur = duration_since_epoch();
    return dur.as_secs() * 1000 + (dur.subsec_nanos() / 1000_000) as u64;
}
