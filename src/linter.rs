use std::borrow::Cow;
use bincode::deserialize;
use inflate::inflate_bytes;
use std::cmp::max;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Scripts {
    Numbers,
    Western,
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
        '\u{0030}'...'\u{0039}' |
        '\u{10140}'...'\u{1018F}' |
        '\u{10100}'...'\u{1013F}' => Scripts::Numbers,
        '\u{0041}'...'\u{005A}' |
        '\u{0061}'...'\u{007A}' |
        '\u{00C0}'...'\u{00FF}' |
        '\u{0100}'...'\u{017F}' |
        '\u{0180}'...'\u{024F}' |
        '\u{0250}'...'\u{02AF}' |
        '\u{02B0}'...'\u{02FF}' |
        '\u{0300}'...'\u{036F}' |
        '\u{0370}'...'\u{03FF}' |
        '\u{0400}'...'\u{04FF}' |
        '\u{0500}'...'\u{052F}' |
        '\u{0530}'...'\u{058F}' |
        '\u{10A0}'...'\u{10FF}' |
        '\u{1680}'...'\u{169F}' |
        '\u{16A0}'...'\u{16FF}' |
        '\u{1AB0}'...'\u{1AFF}' |
        '\u{1C80}'...'\u{1C8F}' |
        '\u{1D00}'...'\u{1D7F}' |
        '\u{1D80}'...'\u{1DBF}' |
        '\u{1DC0}'...'\u{1DFF}' |
        '\u{1E00}'...'\u{1EFF}' |
        '\u{1F00}'...'\u{1FFF}' |
        '\u{20D0}'...'\u{20FF}' |
        '\u{2C00}'...'\u{2C5F}' |
        '\u{2C60}'...'\u{2C7F}' |
        '\u{2D00}'...'\u{2D2F}' |
        '\u{2DE0}'...'\u{2DFF}' |
        '\u{A640}'...'\u{A69F}' |
        '\u{A700}'...'\u{A71F}' |
        '\u{A720}'...'\u{A7FF}' |
        '\u{AB30}'...'\u{AB6F}' |
        '\u{FB00}'...'\u{FB4F}' |
        '\u{10000}'...'\u{1007F}' |
        '\u{10080}'...'\u{100FF}' |
        '\u{101D0}'...'\u{101FF}' |
        '\u{10280}'...'\u{1029F}' |
        '\u{102A0}'...'\u{102DF}' |
        '\u{10300}'...'\u{1032F}' |
        '\u{10330}'...'\u{1034F}' |
        '\u{10350}'...'\u{1037F}' |
        '\u{10450}'...'\u{1047F}' |
        '\u{10500}'...'\u{1052F}' |
        '\u{10530}'...'\u{1056F}' |
        '\u{10600}'...'\u{1077F}' |
        '\u{10800}'...'\u{1083F}' |
        '\u{10920}'...'\u{1093F}' |
        '\u{10C80}'...'\u{10CFF}' |
        '\u{1E000}'...'\u{1E02F}' => Scripts::Western,
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
        match (ws, ns) {
            (Scripts::Chinese, Scripts::Western) |
            (Scripts::Chinese, Scripts::Numbers) |
            (Scripts::Numbers, Scripts::Chinese) => {
                result.push('\u{2009}');
            }
            (Scripts::Western, Scripts::Chinese) => {
                if !buffer.is_empty() {
                    result.push_str(hyphen(&buffer).as_ref());
                    buffer.clear();
                }
                result.push('\u{2009}');
            }
            _ => {}
        }
        if ns == Scripts::Western {
            buffer.push(ch);
        } else {
            result.push(ch);
        }
        ws = ns;
    }
    Cow::Owned(result)
}
