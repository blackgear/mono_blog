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
            tag: tag,
            timer: Instant::now(),
        }
    }
}

impl<'a> Drop for MeasureTimer<'a> {
    fn drop(&mut self) {
        let elapsed = self.timer.elapsed();
        let elapsed = elapsed.as_secs() as f64 * 1000.0
            + elapsed.subsec_nanos() as f64 / 1000000000.0 * 1000.0;
        println!("{:>7} {:>6.3} ms", self.tag, elapsed);
    }
}

/// Warper for easily use MeasureTimer
///
/// # Example
///
/// ```
/// {
///     timer!("Parser");
/// }
/// ```
macro_rules! timer {($e:expr) => {
    #[allow(unused_variables)]
    let time = ::marker::MeasureTimer::new($e);
}}
