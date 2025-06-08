//! Compare `insert` and `insert_unique_unchecked` operations performance.

#![feature(test)]

extern crate test;

use hashbrown::HashMap;
use test::Bencher;

const N: usize = 65536;

#[bench]
fn insert_large(b: &mut Bencher) {
    let keys: Vec<String> = (0..N).map(|i| format!("xxxx{}yyyy", i)).collect();
    b.iter(|| {
        let mut m = HashMap::with_capacity(N);
        for k in &keys {
            m.insert(k, k);
        }
        m
    });
}

#[bench]
fn lookup_all_large(b: &mut Bencher) {
    let keys: Vec<String> = (0..N).map(|i| format!("xxxx{}yyyy", i)).collect();
    let mut m = HashMap::with_capacity(N);
    for k in &keys {
        m.insert(k, k);
    }
    b.iter(|| {
        for k in &keys {
            m.get(k);
        }
    });
}
