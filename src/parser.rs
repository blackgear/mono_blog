extern crate pulldown_cmark;

use pulldown_cmark::{html, Parser, Event, Tag};
use nom::rest_s;
use linter::process;

fn markdown<S: AsRef<str>>(content: S) -> String {
    let mut buffer = String::with_capacity(content.as_ref().len() * 3 / 2);
    let parser = Parser::new(content.as_ref()).scan(true, |state, event| {
        match event {
            Event::Start(Tag::Code) |
            Event::Start(Tag::CodeBlock(_)) => *state = false,
            Event::End(Tag::Code) |
            Event::End(Tag::CodeBlock(_)) => *state = true,
            Event::Text(ref text) if *state => return Some(Event::Text(process(text))),
            _ => (),
        }
        Some(event)
    });
    html::push_html(&mut buffer, parser);
    buffer
}

#[derive(Debug)]
pub struct Post {
    pub data: String,
    pub title: String,
    pub released: String,
    pub modified: String,
    pub category: String,
    pub pagename: String,
}

named!(pub blog<&str, Vec<Post>>,
    many0!(
        do_parse!(
            title: preceded!(take_until_and_consume!("# "), take_until_s!("\n")) >>
            released: preceded!(take_until_and_consume!("本文发表于："), take_until_s!("\n")) >>
            modified: preceded!(take_until_and_consume!("最后修改于："), take_until_s!("\n")) >>
            category: preceded!(take_until_and_consume!("分类："), take_until_s!("\n")) >>
            pagename: preceded!(take_until_and_consume!("地址："), take_until_s!("\n")) >>
            data: ws!(alt!(take_until!("\n# ") | call!(rest_s))) >>
            ( Post {
                data: markdown(data),
                title: process(title).into(),
                released: released.into(),
                modified: modified.into(),
                category: category.into(),
                pagename: pagename.into(),
            } )
        )
    )
);
