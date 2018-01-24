#[macro_use]
extern crate nom;
extern crate pulldown_cmark;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate inflate;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{Read, Error, BufReader};
use std::collections::HashSet;

mod parser;
mod render;
mod linter;

fn load_blog() -> Result<String, Error> {
    let mut data = String::new();
    let mut file = BufReader::new(File::open("ulysses.md")?);
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn main() {
    let content = load_blog().unwrap();
    let blog = parser::blog(&content).to_result().unwrap();
    let categories: HashSet<&String> = blog.iter().map(|x| &x.category).collect();
    for category in categories {
        let fd = blog.iter().filter(|x| &x.category == category);
        println!("{:?}", category);
        for i in fd {
            println!("{:?}", i);
        }
    }

    blog.iter().for_each(render::post);
}
