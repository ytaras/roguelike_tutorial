#![feature(test)]
extern crate rogue_tutorial;
extern crate test;

use test::{Bencher, black_box};
use rogue_tutorial::*;

#[bench]
fn test_generate_string(b: &mut Bencher) {
    b.iter(|| {
        black_box(from_lib())
    });
}
