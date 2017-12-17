#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate pulldown_cmark;

use std::fs::File;
use std::io::Read;
use std::borrow::Cow;
use pulldown_cmark::{html, Parser, Event, Tag};
use std::io::Write;
use std::io::Error;
use std::io::BufReader;

struct Blog {
    Posts: Vec<Post>,
}

impl Blog {
    fn new<S: AsRef<str>>(content: S) -> Blog {
        let posts = content
            .as_ref()
            .match_indices("\n# ")
            .map(|(x, _)| x)
            .scan(0, |state, x| {
                let post = Post::new(&content.as_ref()[*state..x]);
                *state = x;
                Some(post)
            })
            .collect();
        Blog { Posts: posts }
    }
}

struct Post {
    data: String,
    title: String,
    released: String,
    modified: String,
    category: String,
    pagename: String,
}

impl Post {
    fn new<S: AsRef<str>>(content: S) -> Post {
        let parser = Parser::new(content.as_ref());
        let mut buffer = String::with_capacity(content.as_ref().len() * (3 / 2));

        let parser = parser.scan(0, |state, event| {
            let event = match event {
                Event::Text(text) => Event::Text(process(text, *state)),
                _ => event,
            };
            Some(event)
        });

        html::push_html(&mut buffer, parser);

        Post {
            data: buffer.into(),
            title: extract(content.as_ref(), "# "),
            released: extract(content.as_ref(), "\t本文发表于："),
            modified: extract(content.as_ref(), "\t最后修改于："),
            category: extract(content.as_ref(), "\t分类："),
            pagename: extract(content.as_ref(), "\t地址："),
        }
    }
}

fn extract<S: AsRef<str>>(content: S, prefix: &str) -> String {
    content
        .as_ref()
        .lines()
        .filter(|x| x.starts_with(prefix))
        .next()
        .unwrap_or("")
        [prefix.len()..]
        .into()
}

fn process<'a>(content: Cow<'a, str>, state: u8) -> Cow<'a, str> {
    content
        .as_ref()
        .chars()
        .scan(0, |state, c| Some(c))
        .collect()
}

fn to_html(post: Post) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="cmn-Hans" manifest="/mono.appcache">
<head>
<meta charset="UTF-8">
<title>{}</title>
<meta name="author" content="Daniel Zeng">
<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no">
<link rel="stylesheet" type="text/css" href="/mono.css">
<link rel="icon" type="image/png" href="/favicon.png">
<link rel="alternate" type="application/atom+xml" title="RSS" href="/atom.xml">
</head>
<body>
<header>
<a href="/"><h1>DarkNode</h1><h2>Life, the Universe and Everything</h2></a>
</header>
<article>
{}
</article>
<footer>
<p>&copy;&nbsp;2014-2017&nbsp;<a href="/about/">Daniel Zeng</a>&nbsp;</p>
<p><a href="https://creativecommons.org/licenses/by-nc-sa/4.0/">CC BY-NC-SA 4.0</a></p>
</footer>
</body>
</html>"#,
        post.title,
        post.data
    )
}

fn load_blog() -> Result<String, Error> {
    let mut data = String::new();
    let mut file = BufReader::new(File::open("ulysses.md")?);
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn main() {
    let blog = Blog::new(load_blog().unwrap_or("".to_owned()));
    for i in blog.Posts {
        println!("{:?}", i.title);
        println!("{:?}", i.released);
        println!("{:?}", i.modified);
        println!("{:?}", to_html(i));
    }
}
