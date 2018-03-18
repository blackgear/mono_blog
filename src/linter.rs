use std::cmp::max;
use std::borrow::Cow;

static DFA: [u16; 33840] = include!("EN_dfa.in");
static RAW: [u8; 1964] = include!("EN_raw.in");

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
            let offset: usize = DFA[cursor + 0] as usize + chr;
            if offset < 33840 && (DFA[offset + 1] as usize) == cursor {
                cursor = offset;
                break;
            }
            cursor = DFA[cursor + 2] as usize;
        }
        if DFA[cursor + 3] != 0 {
            let data_idx: usize = (DFA[cursor + 3] as usize) >> 4;
            let data_len: usize = (DFA[cursor + 3] as usize) % (1 << 4);
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

fn hyphen<'a>(content: &'a str, result: &mut String) {
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

#[derive(PartialEq)]
enum Scripts {
    Numbers,
    English,
    Chinese,
    Unknown,
}

fn parse(ch: char) -> Scripts {
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
                hyphen(&buffer, &mut result);
                buffer.clear();
            }
            if ns == Scripts::Chinese && ws != Scripts::Unknown {
                result.push('\u{2009}');
            }
        }
        if ns == Scripts::English {
            buffer.push(ch);
        } else {
            match ch {
                '\u{0022}' => result.push_str("&#34;"),
                '\u{0026}' => result.push_str("&#38;"),
                '\u{0027}' => result.push_str("&#39;"),
                '\u{003C}' => result.push_str("&lt;"),
                '\u{003E}' => result.push_str("&gt;"),
                _ => result.push(ch),
            }
        }
        ws = ns;
    }
    hyphen(&buffer, &mut result);
    Cow::Owned(result)
}
