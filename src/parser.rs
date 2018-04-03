//! Parser for Ulysses exported MarkDown bundle.
//!
//! Extract Post from a MarkDown bundle by iterating Blog.
//!
//! # Example
//!
//! ```
//! use parser::Blog;
//! let blog = Blog::from(&data);
//!
//! for post in blog {
//!     println("{}", post.title);
//! }
//! ```
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Arguments, Write};
use pulldown_cmark::{Alignment, Event, Parser, Tag, OPTION_ENABLE_TABLES};
use linter::{process, Scripts};

#[derive(Eq, PartialEq)]
pub struct Post {
    pub title: String,
    pub released: String,
    pub modified: String,
    pub category: String,
    pub pagename: String,
    pub data: String,
}

enum TableState {
    Head,
    Body,
}

pub struct Blog<'a> {
    iter: Parser<'a>,
    size: usize,

    title: String,
    released: String,
    modified: String,
    category: String,
    pagename: String,
    data: String,

    reference: HashMap<Cow<'a, str>, usize>,
    space_state: Scripts,
    space_buffer: String,
    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
}

impl<'a> Blog<'a> {
    pub fn from(content: &'a str) -> Blog<'a> {
        let mut iter = Parser::new_ext(content, OPTION_ENABLE_TABLES);

        while let Some(event) = iter.next() {
            if event == Event::Start(Tag::Header(1)) {
                break;
            }
        }

        Blog {
            iter,
            size: content.len(),

            title: String::with_capacity(64),
            released: String::with_capacity(25),
            modified: String::with_capacity(25),
            category: String::with_capacity(16),
            pagename: String::with_capacity(32),
            data: String::with_capacity(16384),

            reference: HashMap::default(),
            space_state: Scripts::Unknown,
            space_buffer: String::with_capacity(64),
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
        }
    }

    fn clear(&mut self) {
        self.title.clear();
        self.released.clear();
        self.modified.clear();
        self.category.clear();
        self.pagename.clear();
        self.data.clear();
        self.reference.clear();
    }

    fn parse_meta(&mut self) {
        let mut header = true;
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(Tag::CodeBlock(_)) => header = false,
                Event::Text(ref text) if header => {
                    process(&mut self.title, text);
                }
                Event::Text(ref text) if text.starts_with("本文发表于：") => {
                    self.released
                        .push_str(text["本文发表于：".len()..].trim_right());
                }
                Event::Text(ref text) if text.starts_with("最后修改于：") => {
                    self.modified
                        .push_str(text["最后修改于：".len()..].trim_right());
                }
                Event::Text(ref text) if text.starts_with("分类：") => {
                    self.category
                        .push_str(text["分类：".len()..].trim_right());
                }
                Event::Text(ref text) if text.starts_with("地址：") => {
                    self.pagename
                        .push_str(text["地址：".len()..].trim_right());
                }
                Event::End(Tag::CodeBlock(_)) => break,
                _ => (),
            }
        }
    }

    fn parse_body(&mut self) {
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(Tag::Header(1)) => break,
                Event::Start(tag) => self.start_tag(tag),
                Event::End(tag) => self.end_tag(tag),
                Event::Text(text) => self.push_text(&text),
                Event::Html(html) => self.push_html(&html),
                Event::InlineHtml(html) => self.push_html(&html),
                Event::SoftBreak => self.fresh_line(),
                Event::HardBreak => self.push_html("<br />\n"),
                Event::FootnoteReference(name) => {
                    let id = self.poll_note(name);
                    self.push_note(format_args!("<sup><a href=\"#{0}\">{0}</a></sup>", id));
                }
            }
        }
        self.fresh_buffer();
    }

    fn parse_text(&mut self) {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(_) => nest += 1,
                Event::End(_) if nest == 0 => break,
                Event::End(_) => nest -= 1,
                Event::Text(text) => self.push_html(&text),
                Event::Html(_) | Event::InlineHtml(_) => (),
                Event::SoftBreak | Event::HardBreak => self.push_html(" "),
                Event::FootnoteReference(name) => {
                    let id = self.poll_note(name);
                    self.push_note(format_args!("[{0}]", id));
                }
            }
        }
    }

    fn fresh_buffer(&mut self) {
        if !self.space_buffer.is_empty() {
            self.data.push_str(&self.space_buffer);
            self.space_buffer.clear();
        };
    }

    fn fresh_line(&mut self) {
        self.fresh_buffer();
        if !(self.data.is_empty() || self.data.ends_with('\n')) {
            self.data.push('\n');
        }
        self.space_state = Scripts::Unknown;
    }

    fn push_html(&mut self, text: &str) {
        self.fresh_buffer();
        self.data.push_str(text);
        self.space_state = Scripts::Unknown;
    }

    fn push_text(&mut self, text: &str) {
        let ws = self.space_state;
        let ns = text.chars().next().map_or(Scripts::Unknown, |x| x.into());

        if (ws == Scripts::Chinese && ns != Scripts::Unknown)
            || (ns == Scripts::Chinese && ws != Scripts::Unknown) && ws != ns
        {
            self.data.push('\u{2009}');
        };

        self.fresh_buffer();
        process(&mut self.data, text);
        self.space_state = text.chars().last().map_or(Scripts::Unknown, |x| x.into());
    }

    fn push_note(&mut self, args: Arguments) {
        self.fresh_buffer();
        self.data.write_fmt(args).unwrap();
        self.space_state = Scripts::Unknown;
    }

    fn poll_note(&mut self, name: Cow<'a, str>) -> usize {
        let id = self.reference.len() + 1;
        *self.reference.entry(name).or_insert(id)
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Paragraph => {
                self.fresh_line();
                self.data.push_str("<p>");
            }
            Tag::Rule => {
                self.fresh_line();
                self.data.push_str("<hr />\n")
            }
            Tag::Header(level) => {
                self.fresh_line();
                self.data.push_str("<h");
                self.data.push((b'0' + level as u8) as char);
                self.data.push('>');
            }
            Tag::Table(alignments) => {
                self.fresh_line();
                self.table_alignments = alignments;
                self.data.push_str("<table>");
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.space_buffer.push_str("<thead><tr>");
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.space_buffer.push_str("<tr>");
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => self.space_buffer.push_str("<th"),
                    TableState::Body => self.space_buffer.push_str("<td"),
                }
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => self.space_buffer.push_str(" align=\"left\""),
                    Some(&Alignment::Center) => self.space_buffer.push_str(" align=\"center\""),
                    Some(&Alignment::Right) => self.space_buffer.push_str(" align=\"right\""),
                    _ => (),
                }
                self.space_buffer.push_str(">");
            }
            Tag::BlockQuote => {
                self.fresh_line();
                self.data.push_str("<blockquote>\n");
            }
            Tag::CodeBlock(info) => {
                self.fresh_line();
                let lang = info.split(' ').next().unwrap();
                if lang.is_empty() {
                    self.data.push_str("<pre><code>");
                } else {
                    self.data.push_str("<pre><code class=\"language-");
                    process(&mut self.data, lang);
                    self.data.push_str("\">");
                }
            }
            Tag::List(Some(1)) => {
                self.fresh_line();
                self.data.push_str("<ol>\n");
            }
            Tag::List(Some(start)) => {
                self.fresh_line();
                write!(&mut self.data, "<ol start=\"{}\">\n", start).unwrap();
            }
            Tag::List(None) => {
                self.fresh_line();
                self.data.push_str("<ul>\n");
            }
            Tag::Item => {
                self.fresh_line();
                self.data.push_str("<li>");
            }
            Tag::Emphasis => self.space_buffer.push_str("<em>"),
            Tag::Strong => self.space_buffer.push_str("<strong>"),
            Tag::Code => self.space_buffer.push_str("<code>"),
            Tag::Link(dest, title) => {
                self.space_buffer.push_str("<a href=\"");
                self.space_buffer.push_str(&dest);
                if !title.is_empty() {
                    self.space_buffer.push_str("\" title=\"");
                    process(&mut self.space_buffer, title);
                }
                self.space_buffer.push_str("\" target=\"_blank\">");
            }
            Tag::Image(dest, title) => {
                self.space_buffer.push_str("<img src=\"");
                self.space_buffer.push_str(&dest);
                self.space_buffer.push_str("\" alt=\"");
                self.parse_text();
                if !title.is_empty() {
                    self.space_buffer.push_str("\" title=\"");
                    process(&mut self.space_buffer, title);
                }
                self.space_buffer.push_str("\" />")
            }
            Tag::FootnoteDefinition(name) => {
                let id = self.poll_note(name);
                self.push_note(format_args!("<aside id=\"{0}\"><sup>{0}</sup>", id));
            }
        }
    }

    fn end_tag(&mut self, tag: Tag) {
        self.fresh_buffer();
        match tag {
            Tag::Paragraph => self.data.push_str("</p>\n"),
            Tag::Rule => (),
            Tag::Header(level) => {
                self.data.push_str("</h");
                self.data.push((b'0' + level as u8) as char);
                self.data.push_str(">\n");
            }
            Tag::Table(_) => {
                self.data.push_str("</tbody></table>\n");
            }
            Tag::TableHead => {
                self.data.push_str("</tr></thead><tbody>\n");
                self.table_state = TableState::Body;
            }
            Tag::TableRow => {
                self.data.push_str("</tr>\n");
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => self.data.push_str("</th>"),
                    TableState::Body => self.data.push_str("</td>"),
                }
                self.table_cell_index += 1;
            }
            Tag::BlockQuote => self.data.push_str("</blockquote>\n"),
            Tag::CodeBlock(_) => self.data.push_str("</code></pre>\n"),
            Tag::List(Some(_)) => self.data.push_str("</ol>\n"),
            Tag::List(None) => self.data.push_str("</ul>\n"),
            Tag::Item => self.data.push_str("</li>\n"),
            Tag::Emphasis => self.data.push_str("</em>"),
            Tag::Strong => self.data.push_str("</strong>"),
            Tag::Code => self.data.push_str("</code>"),
            Tag::Link(_, _) => self.data.push_str("</a>"),
            Tag::Image(_, _) => (),
            Tag::FootnoteDefinition(_) => self.data.push_str("</aside>\n"),
        }
    }
}

impl<'a> Iterator for Blog<'a> {
    type Item = Post;

    fn next(&mut self) -> Option<Post> {
        if self.iter.get_offset() < self.size {
            self.clear();
            self.parse_meta();
            self.parse_body();
            Some(Post {
                title: self.title.clone(),
                released: self.released.clone(),
                modified: self.modified.clone(),
                category: self.category.clone(),
                pagename: self.pagename.clone(),
                data: self.data.clone(),
            })
        } else {
            None
        }
    }
}
