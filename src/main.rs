//! Yet another static site generator for who cares hyphenation in Western words and space between
//! CJK and Western parts.
//!
//! mblog insert U+2009 between Chinese and Western parts across inline tags, insert U+00AD in the
//! appropriate place inside Western words according to Liang's Hyphenation algorithm and LaTeX's
//! corpus.
//!
//! # Usage
//!
//! Process file in arg, or data from stdin
//!
//! ```
//! $ mblog ulysses.md
//! ```
//!
//! or
//!
//! ```
//! $ cat ulysses.md | mblog
//! ```
//!
//! # Format
//!
//! Front matter and body are just plain markdown. Posts are joined with newline, which is the
//! default format exported from [Ulysses](https://ulyssesapp.com).
//!
//! ```plain
//! # 文章标题1
//!
//!     本文发表于：2018-01-01T12:45:00+08:00
//!     最后修改于：2018-01-12T06:15:00+08:00
//!     分类：category
//!     地址：url-slug-a
//!
//! ...
//!
//! # 文章标题2
//!
//!     本文发表于：2017-10-24T18:00:00+08:00
//!     最后修改于：2017-10-30T21:30:00+08:00
//!     分类：category
//!     地址：url-slug-b
//!
//! ...
//!
//! ```
#![recursion_limit = "128"]
#[macro_use]
extern crate fomat_macros;
extern crate pulldown_cmark;
extern crate rayon;
use std::env;
use std::fs::File;
use std::io::{stdin, Read};

#[macro_use]
mod marker;
mod linter;
mod parser;
mod render;

use parser::Blog;
use render::Site;

/// Process file in arg, or data from stdin
fn main() {
    timer!("total");
    let mut data = String::new();

    match env::args().nth(1) {
        Some(ref path) if path != "-" => {
            File::open(path)
                .and_then(|mut f| f.read_to_string(&mut data))
                .unwrap();
        }
        _ => {
            stdin().read_to_string(&mut data).unwrap();
        }
    }

    Site::new(Blog::from(&data)).render();
}
