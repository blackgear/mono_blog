#![feature(test)]
extern crate rand;
extern crate test;
use test::Bencher;
use rand::{Rng, XorShiftRng};
use std::fs::File;
use std::io::Read;

mod linter;
use linter::hyphen;

extern crate hyphenation;
use hyphenation::Hyphenation;
use hyphenation::Language::English_US;

fn prepare(length: usize) -> Vec<String> {
    let mut data = String::new();
    let mut file = File::open("benches/bench.dict").unwrap();
    file.read_to_string(&mut data).unwrap();
    let mut rng = XorShiftRng::new_unseeded();
    let words: Vec<&str> = data.split("\n").filter(|x| x.len() == length).collect();
    (0..10000)
        .map(|_| rng.choose(&words).unwrap().to_string())
        .collect()
}

macro_rules! bench {
    ($a:ident, $b:ident, $length:expr) => (
        #[bench]
        fn $a(b: &mut Bencher) {
            let data = prepare($length);
            b.iter(|| for word in data.clone() {
                hyphen(&word);
            });
        }

        #[bench]
        fn $b(b: &mut Bencher) {
            let english_us = hyphenation::load(English_US).unwrap();
            let data = prepare($length);

            b.iter(|| for word in data.clone() {
                let _s: String = word.hyphenate(&english_us).punctuate().collect();
            });
        }
    )
}

bench!(bench05_acdat, bench05_crate, 5);
bench!(bench06_acdat, bench06_crate, 6);
bench!(bench07_acdat, bench07_crate, 7);
bench!(bench08_acdat, bench08_crate, 8);
bench!(bench09_acdat, bench09_crate, 9);
bench!(bench10_acdat, bench10_crate, 10);
bench!(bench11_acdat, bench11_crate, 11);
bench!(bench12_acdat, bench12_crate, 12);
bench!(bench13_acdat, bench13_crate, 13);
bench!(bench14_acdat, bench14_crate, 14);
bench!(bench15_acdat, bench15_crate, 15);
bench!(bench16_acdat, bench16_crate, 16);
bench!(bench17_acdat, bench17_crate, 17);
bench!(bench18_acdat, bench18_crate, 18);
