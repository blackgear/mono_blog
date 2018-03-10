extern crate fnv;
use std::fs::File;
use std::io::Read;

mod linter;
use linter::hyphen;

extern crate hyphenation;
use hyphenation::Hyphenation;
use hyphenation::Language::English_US;

fn prepare() -> String {
    let mut data = String::new();
    let mut file = File::open("benches/bench.dict").unwrap();
    file.read_to_string(&mut data).unwrap();
    data
}

#[test]
fn test_equal() {
    let data = prepare();
    let english_us = hyphenation::load(English_US).unwrap();

    for word in data.split("\n") {
        let g = word.hyphenate(&english_us);
        let s: String = g.punctuate().collect();
        assert_eq!(hyphen(word), s);
    }
}
