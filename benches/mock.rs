#![feature(test)]
extern crate rogue_tutorial;
extern crate test;

use rogue_tutorial::*;
use test::{black_box, Bencher};

#[bench]
fn test_generate_string(b: &mut Bencher) {
    b.iter(|| black_box(from_lib()));
}
