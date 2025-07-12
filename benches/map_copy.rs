//! Compare `insert` and `insert_unique_unchecked` operations performance.

#![feature(test)]

extern crate test;

use hashbrown::HashMap;
use test::Bencher;

const N: i32 = 65536;

#[bench]
fn merge_iter(b: &mut Bencher) {
    let keys: Vec<i32> = (0..((N / 4) * 3)).collect();
    let intersect: Vec<i32> = (((N / 4) * 3)..N).collect();
    let mut map = HashMap::new();
    for key in &keys {
        map.insert(*key, std::hint::black_box(N - *key));
    }
    let mut intersect_map = HashMap::new();
    for key in &intersect {
        intersect_map.insert(*key, std::hint::black_box(N - *key));
    }
    b.iter(|| {
        let mut base = map.clone();
        let intersect_map = intersect_map.clone();
        for (k, v) in intersect_map {
            *base.entry(k).or_default() += v;
        }
        let base = std::hint::black_box(base);
        assert_eq!(base.len(), N as _);
    });
}

#[bench]
fn merge_helper(b: &mut Bencher) {
    let keys: Vec<i32> = (0..((N / 4) * 3)).collect();
    let intersect: Vec<i32> = (((N / 4) * 3)..N).collect();
    let mut map = HashMap::new();
    for key in &keys {
        map.insert(*key, std::hint::black_box(N - *key));
    }
    let mut intersect_map = HashMap::new();
    for key in &intersect {
        intersect_map.insert(*key, std::hint::black_box(N - *key));
    }
    b.iter(|| {
        let mut base = map.clone();
        let mut intersect_map = intersect_map.clone();
        base.drain_from(&mut intersect_map, |left, right| *left += right);
        let base = std::hint::black_box(base);
        assert_eq!(base.len(), N as _);
    });
}
