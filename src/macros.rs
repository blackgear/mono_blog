use std::time::Instant;

pub struct MeasureTime<'a> {
    data: &'a str,
    time: Instant,
}

impl<'a> MeasureTime<'a> {
    pub fn new(data: &'a str) -> MeasureTime<'a> {
        MeasureTime {
            data: data,
            time: Instant::now(),
        }
    }
}

impl<'a> Drop for MeasureTime<'a> {
    fn drop(&mut self) {
        let elapsed = self.time.elapsed();
        let elapsed = elapsed.as_secs() as f64 * 1000.0
            + elapsed.subsec_nanos() as f64 / 1000000000.0 * 1000.0;
        println!("{:>7} costs {:>6.3} ms", self.data, elapsed);
    }
}

macro_rules! timer {($e:expr) => {
    #[allow(unused_variables)]
    let time = ::macros::MeasureTime::new($e);
}}
