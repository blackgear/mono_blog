use rayon::prelude::*;
use std::fmt::Debug;
use fnv::FnvHashSet;

fn overlap<T: PartialEq + Clone>(a: &Vec<T>, b: &Vec<T>) -> usize {
    let mut i = 0;
    let index = loop {
        if b.starts_with(&a[i..]) {
            break i;
        } else {
            i += 1;
        }
    };
    a.len() - index
}

fn contains<T: PartialEq + Clone>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a.len() < b.len() {
        return false;
    }
    a.windows(b.len()).position(|i| i == b.as_slice()).is_some()
}

fn dedup<T: PartialEq + Clone>(data: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    'm: for i in 0..data.len() {
        for j in (0..result.len()).rev() {
            if contains(&result[j], &data[i]) {
                continue 'm;
            }
            if contains(&data[i], &result[j]) {
                result.remove(j);
            }
        }
        result.push(data[i].clone());
    }
    result
}

fn permutation(length: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for i in 0..length {
        for j in 0..length {
            if i == j {
                continue;
            };
            result.push((i, j));
        }
    }
    result
}

fn solve<T: PartialEq + Clone + Sync>(mut data: Vec<Vec<T>>) -> Vec<T> {
    data = dedup(data);
    let size = data.len();
    while data.len() > 1 {
        println!(
            "Progress {:5.2}%",
            100.0 - data.len() as f64 / size as f64 * 100.0
        );
        let mut result: Vec<(usize, usize, usize)> = permutation(data.len())
            .into_par_iter()
            .map(|(i, j): (usize, usize)| (i, j, overlap(&data[i], &data[j])))
            .collect();
        result.sort_unstable_by_key(|x| 1000 - x.2);
        let target = result[0].2;
        let length = result.iter().filter(|&x| x.2 == target).count();
        let mut processed: FnvHashSet<usize> = FnvHashSet::default();
        for i in 0..length {
            let target = result[i];
            if processed.contains(&target.0) || processed.contains(&target.1) {
                continue;
            }
            let suffix = &data[target.1][target.2..].to_vec();
            data[target.0].extend_from_slice(suffix);
            processed.insert(target.0);
            processed.insert(target.1);
        }
        data = dedup(data);
    }
    data[0].clone()
}

pub fn process<T: PartialEq + Clone + Sync + Send + Debug>(data: Vec<Vec<T>>) -> Vec<T> {
    let result = solve(data.clone());
    if data.iter().all(|x| contains(&result, x)) {
        println!("Verified");
    }
    result
}
