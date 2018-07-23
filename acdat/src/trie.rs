use fnv::{FnvHashMap, FnvHashSet};
use std::cmp::max;
use std::fmt;

#[derive(Debug)]
pub struct Trie {
    code: u8,
    depth: u16,
    index: u16,
    route: Vec<u8>,
    data: Option<Vec<u8>>,
    child: FnvHashMap<u8, Trie>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            code: 0,
            depth: 0,
            index: 0,
            data: None,
            child: FnvHashMap::default(),
            route: vec![],
        }
    }

    fn add(&mut self, codes: &[u8], data: Vec<u8>) {
        let t = self.child.entry(codes[0]).or_insert(Trie::new());
        t.code = codes[0];
        t.depth = self.depth + 1;
        t.route = self.route.clone();
        t.route.push(codes[0]);
        if codes.len() > 1 {
            t.add(&codes[1..], data);
        } else {
            t.data = Some(data)
        }
    }

    pub fn insert(&mut self, text: &str, data: Vec<u8>) {
        self.add(&text.as_bytes(), data);
    }
}

pub struct DATrie {
    base: Vec<u16>,
    mark: Vec<u16>,
    fail: Vec<u16>,
    used: Vec<bool>,
    data: Vec<Option<Vec<u8>>>,
}

impl DATrie {
    pub fn new() -> DATrie {
        DATrie {
            base: vec![0],
            mark: vec![0],
            fail: vec![0],
            used: vec![true],
            data: vec![],
        }
    }

    pub fn convert(&mut self, trie: &mut Trie) {
        if let Some(ref data) = trie.data {
            self.data[trie.index as usize] = Some(data.clone());
        }
        let offset = self.alloc(&mut trie.child);

        self.base[trie.index as usize] = offset;
        for (&code, mut node) in &mut trie.child {
            let index = offset as usize + code as usize;
            self.resize(index);
            self.mark[index] = trie.index;
            self.used[index] = true;
            node.index = index as u16;
        }

        for (_, mut node) in &mut trie.child {
            self.convert(&mut node);
        }
    }

    fn alloc(&mut self, k: &mut FnvHashMap<u8, Trie>) -> u16 {
        let mut offset: u16 = 0;
        loop {
            if k.keys().any(|&code| {
                let index = offset as usize + code as usize;
                *self.used.get(index).unwrap_or(&false)
            }) {
                offset += 1;
                continue;
            };
            self.resize(offset as usize);
            break offset;
        }
    }

    fn resize(&mut self, offset: usize) {
        if offset >= self.base.len() {
            self.base.resize(offset + 1, 0);
            self.mark.resize(offset + 1, u16::max_value());
            self.fail.resize(offset + 1, 0);
            self.used.resize(offset + 1, false);
            self.data.resize(offset + 1, None);
        }
    }

    pub fn prepare(&mut self, trie: &Trie) {
        for i in 1..trie.route.len() {
            if let Some(index) = self.fetch(&trie.route[i..]) {
                if self.fail[trie.index as usize] == 0 {
                    self.fail[trie.index as usize] = index;
                }
                if let Some(that) = self.data[index as usize].clone() {
                    let mut this: Vec<u8> = self.data[trie.index as usize]
                        .clone()
                        .unwrap_or(vec![0; that.len()]);
                    let delta = this.len() - that.len();
                    for i in 0..that.len() {
                        this[i + delta] = max(this[i + delta], that[i]);
                    }
                    self.data[trie.index as usize] = Some(this);
                }
            }
        }
        for (_, node) in &trie.child {
            self.prepare(&node);
        }
    }

    fn fetch(&self, text: &[u8]) -> Option<u16> {
        let mut cursor = 0;
        for &chr in text {
            let mut offset = self.base[cursor as usize] + chr as u16;
            if self.mark[offset as usize] != cursor {
                return None;
            }
            cursor = offset;
        }
        Some(cursor)
    }

    pub fn fillrate(&self) -> f64 {
        self.used.iter().filter(|&x| *x).count() as f64 * 100.0 / self.used.len() as f64
    }

    pub fn datalist(&self) -> Vec<Vec<u8>> {
        let mut points = FnvHashSet::default();
        for i in &self.data {
            if let &Some(ref d) = i {
                points.insert(d.clone());
            }
        }
        points.into_iter().collect()
    }
}

fn show<T: Iterator<Item = G>, G: fmt::Display>(f: &mut fmt::Formatter, name: &str, list: T) {
    write!(f, "{}", name).unwrap();
    for i in list {
        write!(f, "|{:3}", &i).unwrap();
    }
    write!(f, "|\n").unwrap();
}

impl fmt::Debug for DATrie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        show(f, " idx", 0..self.base.len());
        show(f, "base", self.base.iter());
        show(f, "mark", self.mark.iter());
        show(f, "fail", self.fail.iter());
        show(f, "used", self.used.iter().map(|&x| x as u16));
        Ok(())
    }
}

#[derive(Debug)]
pub struct State {
    base: u16,
    mark: u16,
    fail: u16,
    data: u16,
}

impl State {
    fn new(base: u16, mark: u16, fail: u16, data: Option<Vec<u8>>, raw: &[u8]) -> State {
        let mut result: u16 = 0;
        if let Some(data) = data {
            if let Some(idx) = raw.windows(data.len()).position(|i| i == data.as_slice()) {
                result = ((idx as u16) << 4) + data.len() as u16;
            }
        }
        State {
            base: base,
            mark: mark,
            fail: fail,
            data: result,
        }
    }

    fn pack(&self) -> [u16; 4] {
        if self.mark == u16::max_value() {
            [self.base * 4, self.mark, self.fail * 4, self.data]
        } else {
            [self.base * 4, self.mark * 4, self.fail * 4, self.data]
        }
    }
}

#[derive(Debug)]
pub struct ACdat {
    dfa: Vec<State>,
    raw: Vec<u8>,
}

impl ACdat {
    pub fn new(trie: DATrie, raw: &[u8]) -> ACdat {
        let mut result = ACdat {
            dfa: vec![],
            raw: raw.to_vec(),
        };
        for i in 0..trie.base.len() {
            result.dfa.push(State::new(
                trie.base[i],
                trie.mark[i],
                trie.fail[i],
                trie.data[i].clone(),
                &raw,
            ))
        }
        result
    }

    pub fn pack(&self) -> Vec<u16> {
        let mut r = vec![];
        for i in &self.dfa {
            r.extend_from_slice(&i.pack());
        }
        r
    }
}
