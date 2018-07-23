use std::cmp::max;
use std::borrow::Cow;

static DFA: [u16; 33840] = include!("EN_dfa.in");
static RAW: [u8; 1964] = include!("EN_raw.in");

pub fn detect(content: &str) -> Vec<u8> {
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

pub fn hyphen<'a, S: AsRef<str>>(content: S) -> Cow<'a, str> {
    let length = content.as_ref().len();
    if length < 5 {
        return Cow::Owned(content.as_ref().to_string())
    }
    let points = detect(content.as_ref().to_lowercase());

    let mut result = String::with_capacity(length * 3 / 2);

    for (i, chr) in content.as_ref().chars().enumerate() {
        result.push(chr);
        if i > 0 && i < length - 3 && points[i+2] & 1 != 0 {
            result.push('\u{00AD}')
        }
    }
    Cow::Owned(result)
}
