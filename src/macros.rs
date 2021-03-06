//! Marker to measure the time cost within scope.
//!
//! MeasureTimer prints tag and elapsed time when dropped.
//!
//! # Example
//!
//! ```
//! {
//!     timer!("Parser");
//! }
//! ```
use std::time::Instant;

/// Print tag and elapsed when dropped.
pub struct MeasureTimer<'a> {
    tag: &'a str,
    timer: Instant,
}

impl<'a> MeasureTimer<'a> {
    pub fn new(tag: &'a str) -> MeasureTimer<'a> {
        MeasureTimer {
            tag,
            timer: Instant::now(),
        }
    }
}

impl<'a> Drop for MeasureTimer<'a> {
    fn drop(&mut self) {
        let elapsed = self.timer.elapsed();
        let elapsed = elapsed.as_secs() as f64 * 1000.0
            + elapsed.subsec_nanos() as f64 / 1_000_000_000.0 * 1000.0;
        println!("{:>7} {:>6.3} ms", self.tag, elapsed);
    }
}

/// Warper for ergonomic time measuring
///
/// # Example
///
/// ```
/// {
///     timer!("Parser");
/// }
/// ```
macro_rules! timer {
    ($e:expr) => {
        let _timer = ::macros::MeasureTimer::new($e);
    };
}

/// Warper for ergonomic performance optimization
///
/// # Example
///
/// ```
/// {
///     unreachable!();
/// }
/// ```
macro_rules! unreachable {
    () => {
        unsafe {
            ::std::hint::unreachable_unchecked();
        };
    };
}
