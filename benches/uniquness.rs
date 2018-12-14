#![feature(test)]
extern crate itertools;
extern crate rogue_tutorial;
extern crate test;

use test::Bencher;

use itertools::*;

use rogue_tutorial::data::structures::matrix::PosCollection;
use rogue_tutorial::data::structures::*;

#[bench]
fn uniqueness_testing_pos(b: &mut Bencher) {
    let from = Pos { x: 0, y: 0 };
    let to = Pos {
        x: DimIndex::max_value(),
        y: DimIndex::max_value(),
    };
    let range = from..to;

    b.iter(|| {
        let one = range.iter_pos().count();
        let two = range.iter_pos().unique().count();
        assert_eq!(one, two);
    })
}
