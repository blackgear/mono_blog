#![allow(unused_imports, dead_code)]

extern crate fnv;
#[macro_use]
extern crate nom;
extern crate rand;
extern crate rayon;
use fnv::FnvHashSet;
use nom::line_ending;
use std::fs::File;
use std::io::{Read, Write};
use std::mem;

mod scs;
mod trie;
use trie::{ACdat, DATrie, Trie};

named!(point<&str, u8>,
    map!(opt!(one_of!("0123456789")), |x| x.unwrap_or('0').to_digit(10).unwrap() as u8)
);

named!(pchar<&str, char>,
    one_of!(".abcdefghijklmnopqrstuvwxyz")
);

named!(parse<&str, Vec<(std::string::String, std::vec::Vec<u8>)> >,
    many0!(
        terminated!(
            do_parse!(
                init: point >>
                pair: many0!(pair!(pchar, point)) >>
                (
                    pair.iter().fold(
                        (String::new(),vec![init]),
                        |mut r, &(x, n): &(char, u8)| {
                            r.0.push(x);
                            r.1.push(n);
                            r
                        }
                    )
                )
            ),
            line_ending
        )
    )
);

fn main() {
    let mut data = String::new();
    let mut file = File::open("hyph-en-us.pat").unwrap();
    file.read_to_string(&mut data).unwrap();

    let mut t = Trie::new();
    for &(ref text, ref points) in parse(&data).to_result().unwrap().iter() {
        t.insert(text, points.clone());
    }
    let mut r = DATrie::new();
    r.convert(&mut t);
    r.prepare(&t);
    println!("Fillrate {:.2}%", r.fillrate());
    println!("New Pattern Generated");

    let raw = scs::process(r.datalist());

    let g = ACdat::new(r, &raw);
    let dfa = g.pack();

    let mut f = File::create("EN_dfa.in").unwrap();
    write!(f, "{:?}", dfa).unwrap();

    let mut f = File::create("EN_raw.in").unwrap();
    write!(f, "{:?}", raw).unwrap();
    println!("New Pattern Compressed");
}
