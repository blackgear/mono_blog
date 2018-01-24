use std::borrow::Cow;
use bincode::deserialize;
use inflate::inflate_bytes;
use std::cmp::max;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Scripts {
    Numbers,
    English,
    Chinese,
    Unknown,
}

#[derive(Deserialize)]
struct Tree {
    node: Vec<(u8, Tree)>,
    data: Option<Vec<(u8, u8)>>,
}

lazy_static! {
    static ref PATTERN: Tree = {
        deserialize(&inflate_bytes(include_bytes!("en.bincode")).unwrap()).unwrap()
    };
    static ref HYPHENS: Mutex<HashMap<String, Vec<u8>>> = Mutex::new({
        let mut m = HashMap::new();
        m.insert("associate".into()     , vec![0,1,0,1,0,0,0,0,0] );
        m.insert("associates".into()    , vec![0,1,0,1,0,0,0,0,0,0] );
        m.insert("declination".into()   , vec![0,0,1,0,1,0,1,0,0,0,0] );
        m.insert("obligatory".into()    , vec![0,0,0,0,1,1,0,0,0,0] );
        m.insert("philanthropic".into() , vec![0,0,0,1,0,1,0,0,0,0,0,0,0] );
        m.insert("present".into()       , vec![0,0,0,0,0,0,0] );
        m.insert("presents".into()      , vec![0,0,0,0,0,0,0,0] );
        m.insert("project".into()       , vec![0,0,0,0,0,0,0] );
        m.insert("projects".into()      , vec![0,0,0,0,0,0,0,0] );
        m.insert("reciprocity".into()   , vec![0,0,0,1,0,0,0,0,0,0,0] );
        m.insert("recognizance".into()  , vec![0,1,0,0,1,0,1,0,0,0,0,0] );
        m.insert("reformation".into()   , vec![0,0,1,0,1,0,1,0,0,0,0] );
        m.insert("retribution".into()   , vec![0,0,1,0,1,0,1,0,0,0,0] );
        m.insert("table".into()         , vec![0,1,0,0,0] );
        m
    });
}

fn parse(ch: char) -> Scripts {
    match ch {
        '\u{0030}'...'\u{0039}' => Scripts::Numbers,
        '\u{0041}'...'\u{005A}' |
        '\u{0061}'...'\u{007A}' => Scripts::English,
        '\u{3400}'...'\u{4DBF}' |
        '\u{4E00}'...'\u{9FFF}' |
        '\u{F900}'...'\u{FAFF}' |
        '\u{20000}'...'\u{2A6DF}' |
        '\u{2B740}'...'\u{2B81F}' |
        '\u{2B820}'...'\u{2CEAF}' |
        '\u{2CEB0}'...'\u{2EBE0}' |
        '\u{2F800}'...'\u{2FA1F}' => Scripts::Chinese,
        _ => Scripts::Unknown,
    }
}

fn points<'a, S: AsRef<str>>(content: S) -> Vec<u8> {
    let length = content.as_ref().len();

    if length < 5 {
        return vec![0; length];
    }

    let word = [".", &content.as_ref(), "."].concat().into_bytes();
    let mut points: Vec<u8> = vec![0; length];
    for prefix in 0..length {
        let mut tree = &*PATTERN;
        for chr in &word[prefix..] {
            match tree.node.binary_search_by_key(chr, |&(x, _)| x) {
                Ok(index) => {
                    tree = &tree.node[index].1;
                    if let Some(ref data) = tree.data {
                        for &(offset, point) in data.iter() {
                            let i = prefix + offset as usize;
                            if i > 1 && i <= length {
                                points[i - 2] = max(point, points[i - 2]);
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }
    points[0] = 0;
    points[length - 3] = 0;
    points[length - 2] = 0;
    points
}

fn hyphen<'a, S: AsRef<str>>(content: S) -> Cow<'a, str> {
    let word = content.as_ref().to_lowercase();
    let points = HYPHENS
        .lock()
        .unwrap()
        .entry(word.clone())
        .or_insert_with(|| points(&word))
        .clone();

    let mut result = String::with_capacity(content.as_ref().len() * 3 / 2);
    for (i, chr) in content.as_ref().chars().enumerate() {
        result.push(chr);
        if points[i] % 2 != 0 {
            result.push('\u{00AD}')
        }
    }
    Cow::Owned(result)
}

pub fn process<'a, S: AsRef<str>>(content: S) -> Cow<'a, str> {
    let mut ws = Scripts::Unknown;
    let mut chars = content.as_ref().chars();
    let mut result = String::with_capacity(content.as_ref().len() * 3 / 2);
    let mut buffer = String::with_capacity(20);

    while let Some(ch) = chars.next() {
        let ns = parse(ch);
        if ws != ns {
            if ws == Scripts::Chinese && ns != Scripts::Unknown {
                result.push('\u{2009}');
            }
            if ws == Scripts::English {
                result.push_str(hyphen(&buffer).as_ref());
                buffer.clear();
            }
            if ns == Scripts::Chinese && ws != Scripts::Unknown {
                result.push('\u{2009}');
            }
        }
        if ns == Scripts::English {
            buffer.push(ch);
        } else {
            result.push(ch);
        }
        ws = ns;
    }
    Cow::Owned(result)
}
