//! Yet another static site generator for who cares hyphenation in Western words
//! and space between CJK and Western parts.
//!
//! mblog insert U+2009 between Chinese and Western parts, crossing inline tags.
//! also insert U+00AD in the appropriate place inside Western words, according
//! to Liang's Hyphenation algorithm and LaTeX's corpus.
//!
//! # How to use mblog
//!
//! ```
//! $ cat ulysses.md | mblog
//! ```
//!
//! or
//!
//! ```
//! $ mblog ulysses.md
//! ```
#![recursion_limit = "128"]
#[macro_use]
extern crate fomat_macros;
extern crate pulldown_cmark;
extern crate rayon;
use std::io::{stdin, Read};
use std::fs::File;
use std::env;

#[macro_use]
mod marker;
mod linter;
mod parser;
mod render;

use parser::Blog;
use render::Site;

fn main() {
    timer!("total");
    let mut data = String::new();

    if let Some(path) = env::args().nth(1) {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut data).unwrap();
    } else {
        let stdin = stdin();
        stdin.lock().read_to_string(&mut data).unwrap();
    }

    Site::new(Blog::from(&data)).render();
}
