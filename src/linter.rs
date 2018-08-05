//! Linter for Chinese-English mixed content.
//!
//! Hyphenate, space and HTML-escape the content within O(n) time complexity, O(n) space complexity.
//!
//! This implementation of [Liang's hyphenation algorithm](https://tug.org/docs/liang/) is optimized
//! by a Double Array Trie based Aho–Corasick algorithm. 3x faster than
//! [hyphenation = "0.6.1"](https://crates.io/crates/hyphenation).
//!
//! # Liang's hyphenation algorithm
//!
//! ```plain
//!        . H y p h e n a t i o n .
//!
//! hy3ph => h y p h
//!         0,0,3,0,0
//!
//! he2n  =>       h e n
//!               0,0,2,0
//!
//!    ...   ...   ...   ...   ...   ...
//!
//! 1tio  =>               t i o
//!                       1,0,0,0
//!
//! o2n   =>                   o n
//!                           0,2,0
//!
//!        [0,0,3,0,0,2,5,4,2,0,2,0] <- This is Points
//!             ↓       ↓
//!          H y-p h e n-a t i o n
//! ```
//! Points represents hyphenation occasion in words, which is build by merge all points from matched
//! pattern. Larger Pattern point will cover/shadowing small point. Odd point allows hyphen at this
//! occasion, while even point disallows.
//!
//! This implementation stores Patterns in Double Array Trie based Aho–Corasick automaton, specially
//! optimised for data size and code speed.
//!
//! # Spacing algorithm
//!
//! Space will be insert between:
//!
//! ```plain
//! Scripts::Chinese & Scripts::English
//! Scripts::Chinese & Scripts::Numbers
//! Scripts::English & Scripts::Chinese
//! Scripts::Numbers & Scripts::Chinese
//! ```
//!
//! # HTML-escape algorithm
//!
//! ```plain
//! " => &#34;
//! & => &#38;
//! ' => &#39;
//! < => &lt;
//! > => &gt;
//! ```
use std::cmp::max;

/// Double Array Trie based Aho–Corasick algorithm transitions
static DFA: [u16; 33840] = include!("EN_dfa.in");
/// Pattern Points compressed by Shortest Common Supersequence
static RAW: [u8; 1964] = include!("EN_raw.in");

/// Convert a &str to Points. This function uses black magic codes to reduce cache miss(D1mr=0.44
/// DLmr=0.16) and improve speed.
/// DO NOT CHANGE WITHOUT BENCHMARK
fn detect(content: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; content.len() + 1];
    let mut cursor: usize = 184;
    let content = content
        .as_bytes()
        .iter()
        .map(|&x| match x {
            0x41...0x5A => (x as usize + 32) << 2,
            0x61...0x7A => (x as usize) << 2,
            _ => unreachable!(),
        })
        .chain([184].iter().cloned())
        .enumerate();
    for (idx, chr) in content {
        loop {
            let offset: usize = DFA[cursor] as usize + chr;
            if offset < 33840 && (DFA[offset + 1] as usize) == cursor {
                cursor = offset;
                break;
            }
            cursor = DFA[cursor + 2] as usize;
        }
        if DFA[cursor + 3] != 0 {
            let data_idx: usize = (DFA[cursor + 3] as usize) >> 4;
            let data_len: usize = (DFA[cursor + 3] as usize) & 0b1111;
            for i in 0..data_len {
                let p = RAW[data_idx + i];
                if p != 0 {
                    let g = idx + i + 2 - data_len;
                    result[g] = max(result[g], p);
                }
            }
        }
    }
    result
}

/// Push the hyphenated content into a ref mut String.
///
/// # Examples
///
/// ```rust
/// use linter::hyphen;
///
/// let mut result = String::new()
/// hyphen("Hyphenation", &mut result);
///
/// assert_eq!("Hy\u{00AD}phen\u{00AD}ation", result);
/// ```
///
/// # Panics
///
/// content should within [a-zA-Z] otherwise this will panic unreachable!().
///
/// ```rust,should_panic
/// use linter::hyphen;
///
/// let mut result = String::new()
/// hyphen("中文", &mut result);
/// ```
fn hyphen<'a>(result: &mut String, content: &'a str) {
    let length = content.len();
    if length < 5 {
        return result.push_str(content);
    }
    match content {
        "associate" => result.push_str("as\u{00AD}so\u{00AD}ciate"),
        "associates" => result.push_str("as\u{00AD}so\u{00AD}ciates"),
        "declination" => result.push_str("dec\u{00AD}li\u{00AD}na\u{00AD}tion"),
        "obligatory" => result.push_str("oblig\u{00AD}a\u{00AD}tory"),
        "philanthropic" => result.push_str("phil\u{00AD}an\u{00AD}thropic"),
        "present" => result.push_str("present"),
        "presents" => result.push_str("presents"),
        "project" => result.push_str("project"),
        "projects" => result.push_str("projects"),
        "reciprocity" => result.push_str("reci\u{00AD}procity"),
        "recognizance" => result.push_str("re\u{00AD}cog\u{00AD}ni\u{00AD}zance"),
        "reformation" => result.push_str("ref\u{00AD}or\u{00AD}ma\u{00AD}tion"),
        "retribution" => result.push_str("ret\u{00AD}ri\u{00AD}bu\u{00AD}tion"),
        "table" => result.push_str("ta\u{00AD}ble"),
        _ => {
            let points = detect(content);
            for (i, chr) in content.chars().enumerate() {
                result.push(chr);
                if i > 0 && i < length - 3 && points[i + 1] & 1 != 0 {
                    result.push('\u{00AD}')
                }
            }
        }
    }
}

/// Represents writing system of a char
///
/// ```rust
/// '\u{0030}'...'\u{0039}' => Scripts::Numbers,
///
/// '\u{0041}'...'\u{005A}'
/// '\u{0061}'...'\u{007A}' => Scripts::English,
///
/// '\u{3400}'...'\u{4DBF}'
/// '\u{4E00}'...'\u{9FFF}'
/// '\u{F900}'...'\u{FAFF}'
/// '\u{20000}'...'\u{2A6DF}'
/// '\u{2B740}'...'\u{2B81F}'
/// '\u{2B820}'...'\u{2CEAF}'
/// '\u{2CEB0}'...'\u{2EBE0}'
/// '\u{2F800}'...'\u{2FA1F}' => Scripts::Chinese,
///
/// _ => Scripts::Unknown,
/// ```
///
/// # Examples
///
/// ```rust
/// assert_eq!(Scripts::Numbers, Scripts::from('0'));
///
/// assert_eq!(Scripts::English, Scripts::from('a'));
///
/// assert_eq!(Scripts::Chinese, Scripts::from('中'));
/// ```
#[derive(PartialEq, Copy, Clone)]
pub enum Scripts {
    Numbers,
    English,
    Chinese,
    Unknown,
}

impl From<char> for Scripts {
    fn from(ch: char) -> Self {
        match ch {
            '\u{0030}'...'\u{0039}' => Scripts::Numbers,
            '\u{0041}'...'\u{005A}' | '\u{0061}'...'\u{007A}' => Scripts::English,
            '\u{3400}'...'\u{4DBF}'
            | '\u{4E00}'...'\u{9FFF}'
            | '\u{F900}'...'\u{FAFF}'
            | '\u{20000}'...'\u{2A6DF}'
            | '\u{2B740}'...'\u{2B81F}'
            | '\u{2B820}'...'\u{2CEAF}'
            | '\u{2CEB0}'...'\u{2EBE0}'
            | '\u{2F800}'...'\u{2FA1F}' => Scripts::Chinese,
            _ => Scripts::Unknown,
        }
    }
}

/// Returns the hyphenated spaced and HTML-escaped content.
///
/// # Examples
///
/// ```rust
/// use linter::Lintable;
///
/// let mut result = String::new();
/// result.push_txt(">这是Hyphenation的文字");
///
/// assert_eq!("&gt;这是\u{2009}Hy\u{00AD}phen\u{00AD}ation\u{2009}的文字", result);
/// ```
pub trait Linter {
    fn push_txt<S: AsRef<str>>(&mut self, text: S);
}

impl Linter for String {
    fn push_txt<S: AsRef<str>>(&mut self, text: S) {
        let mut ws = Scripts::Unknown;
        let mut buffer = String::with_capacity(20);

        for ch in text.as_ref().chars() {
            let ns = ch.into();
            if ws != ns {
                if ws == Scripts::Chinese && ns != Scripts::Unknown {
                    self.push('\u{2009}');
                }
                if ws == Scripts::English {
                    hyphen(self, &buffer);
                    buffer.clear();
                }
                if ns == Scripts::Chinese && ws != Scripts::Unknown {
                    self.push('\u{2009}');
                }
            }
            if ns == Scripts::English {
                buffer.push(ch);
            } else {
                match ch {
                    '\u{0022}' => self.push_str("&#34;"),
                    '\u{0026}' => self.push_str("&#38;"),
                    '\u{0027}' => self.push_str("&#39;"),
                    '\u{003C}' => self.push_str("&lt;"),
                    '\u{003E}' => self.push_str("&gt;"),
                    _ => self.push(ch),
                }
            }
            ws = ns;
        }
        hyphen(self, &buffer);
    }
}
