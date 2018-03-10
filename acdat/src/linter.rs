use std::cmp::max;
use std::borrow::Cow;

static DFA: [u16; 33840] = include!("EN_dfa.in");
static RAW: [u8; 1964] = include!("EN_raw.in");

pub fn detect<'a, S: AsRef<str>>(content: S) -> Vec<u8> {
    let opcode = [".", &content.as_ref(), "."].concat().into_bytes();
    let length = opcode.len();
    let mut result: Vec<u8> = vec![0; length+1];
    let mut cursor: usize = 0;

    for index in 0..length {
        loop {
            let offset: usize = DFA[cursor+0] as usize + ((opcode[index] as usize) << 2);
            if offset >= 33840 || (DFA[offset+1] as usize) != cursor {
                cursor = DFA[cursor+2] as usize;
            } else {
                cursor = offset;
                if DFA[cursor+3] != 0 {
                    let data_idx: usize = (DFA[cursor+3] as usize) >> 4;
                    let data_len: usize = (DFA[cursor+3] as usize) % (1 << 4);
                    for i in 0..data_len {
                        let idx = index+i+2-data_len;
                        result[idx] = max(result[idx], RAW[data_idx+i])
                    }
                }
                break;
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
