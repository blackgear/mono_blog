#![recursion_limit = "128"]
#[macro_use]
extern crate fomat_macros;
extern crate pulldown_cmark;
use std::io::{stdin, Read};

#[macro_use]
mod macros;
mod linter;
mod parser;
mod render;

use parser::Blog;
use render::Site;

fn main() {
    let mut data = String::new();
    stdin().read_to_string(&mut data).unwrap();

    timer!("Overall");
    Site::new(Blog::from(&data)).render();
}
