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
