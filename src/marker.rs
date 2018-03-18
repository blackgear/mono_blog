use std::time::Instant;

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

macro_rules! timer {($e:expr) => {
    #[allow(unused_variables)]
    let time = ::marker::MeasureTimer::new($e);
}}
