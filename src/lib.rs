// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios")))]
mod imp {
    use libc::{c_int, timespec};

    // Apparently android provides this in some other library?
    #[cfg(all(not(target_os = "android"),
              not(target_os = "nacl")))]
    #[link(name = "rt")]
    extern {}

    extern {
        pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    }

}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod imp {
    use libc::{timeval, timezone, c_int, mach_timebase_info as timebase_info};
    use std::sync::{Once, ONCE_INIT};

    extern {
        pub fn gettimeofday(tp: *mut timeval, tzp: *mut timezone) -> c_int;
        pub fn mach_absolute_time() -> u64;
        pub fn mach_timebase_info(info: *mut timebase_info) -> c_int;
    }

    pub fn info() -> &'static timebase_info {
        static mut INFO: timebase_info = timebase_info {
            numer: 0,
            denom: 0,
        };
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                mach_timebase_info(&mut INFO);
            });
            &INFO
        }
    }
}

#[cfg(windows)]
mod imp {
    use libc;
    use std::sync::{Once, ONCE_INIT};

    pub fn frequency() -> libc::LARGE_INTEGER {
        static mut FREQUENCY: libc::LARGE_INTEGER = 0;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                libc::QueryPerformanceFrequency(&mut FREQUENCY);
            });
            FREQUENCY
        }
    }
}

/**
 * Returns the current value of a high-resolution performance counter
 * in nanoseconds since an unspecified epoch.
 */
pub fn precise_time_ns() -> u64 {
    return os_precise_time_ns();

    #[cfg(windows)]
    fn os_precise_time_ns() -> u64 {
        let mut ticks = 0;
        assert_eq!(unsafe {
            libc::QueryPerformanceCounter(&mut ticks)
        }, 1);

        mul_div_i64(ticks as i64, 1000000000, imp::frequency() as i64) as u64
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn os_precise_time_ns() -> u64 {
        unsafe {
            let time = imp::mach_absolute_time();
            let info = imp::info();
            time * info.numer as u64 / info.denom as u64
        }
    }

    #[cfg(not(any(windows, target_os = "macos", target_os = "ios")))]
    fn os_precise_time_ns() -> u64 {
        let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe {
            imp::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
        }
        return (ts.tv_sec as u64) * 1000000000 + (ts.tv_nsec as u64)
    }
}


/**
 * Returns the current value of a high-resolution performance counter
 * in seconds since an unspecified epoch.
 */
pub fn precise_time_s() -> f64 {
    return (precise_time_ns() as f64) / 1000000000.;
}

/**
 * Returns the current value of a high-resolution performance counter
 * in milliseconds since an unspecified epoch.
 */
pub fn precise_time_ms() -> u64 {
    return precise_time_ns() / 1000000;
}

// Computes (value*numer)/denom without overflow, as long as both
// (numer*denom) and the overall result fit into i64 (which is the case
// for our time conversions).
#[allow(dead_code)]
fn mul_div_i64(value: i64, numer: i64, denom: i64) -> i64 {
    let q = value / denom;
    let r = value % denom;
    // Decompose value as (value/denom*denom + value%denom),
    // substitute into (value*numer)/denom and simplify.
    // r < denom, so (denom*numer) is the upper bound of (r*numer)
    q * numer + r * numer / denom
}
