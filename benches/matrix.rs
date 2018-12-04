#![feature(test)]
extern crate rogue_tutorial;
extern crate test;

use test::Bencher;

use rogue_tutorial::data::structures::matrix::*;

#[bench]
fn big_matrix_creation(b: &mut Bencher) {
    let x = DimIndex::max_value();
    let y = DimIndex::max_value();

    b.iter(|| {
        let _: Matrix<bool> = Matrix::new(x, y);
    })
}

#[bench]
fn sane_matrix_creation(b: &mut Bencher) {
    let x = 100;
    let y = 100;

    b.iter(|| {
        let _: Matrix<bool> = Matrix::new(x, y);
    })
}
