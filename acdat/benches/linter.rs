use std::borrow::Cow;
use std::cmp::max;

static DFA: [u16; 33840] = include!("EN_dfa.in");
static RAW: [u8; 1964] = include!("EN_raw.in");

pub fn detect(content: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; content.len() + 1];
    let mut cursor: usize = 184;
    let content = content
        .as_bytes()
        .iter()
        .map(|&x| {
            if x < 0x61 {
                (x as usize + 32) << 2
            } else {
                (x as usize) << 2
            }
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

pub fn hyphen<'a>(content: &'a str) -> Cow<'a, str> {
    let length = content.len();
    if length < 5 {
        return Cow::Borrowed(content);
    }
    let points = match content {
        "associate" => vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
        "associates" => vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0],
        "declination" => vec![0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0],
        "obligatory" => vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        "philanthropic" => vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        "present" => vec![0, 0, 0, 0, 0, 0, 0, 0],
        "presents" => vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        "project" => vec![0, 0, 0, 0, 0, 0, 0, 0],
        "projects" => vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        "reciprocity" => vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        "recognizance" => vec![0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
        "reformation" => vec![0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0],
        "retribution" => vec![0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0],
        "table" => vec![0, 0, 1, 0, 0, 0],
        _ => detect(content),
    };

    let mut result = String::with_capacity(length * 3 / 2);

    for (i, chr) in content.chars().enumerate() {
        result.push(chr);
        if i > 0 && i < length - 3 && points[i + 1] & 1 != 0 {
            result.push('\u{00AD}')
        }
    }
    Cow::Owned(result)
}
