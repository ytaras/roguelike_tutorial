#![feature(test)]
extern crate rogue_tutorial;
extern crate test;

use rogue_tutorial::levels::level_1;
use test::Bencher;

#[bench]
fn level_1_gen(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    b.iter(|| level_1(&mut rng))
}
