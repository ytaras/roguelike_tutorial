#![feature(test)]
extern crate rogue_tutorial;
extern crate test;

use rand::prelude::*;
use rogue_tutorial::common::fov::*;
use rogue_tutorial::data::structures::matrix::Matrix;
use rogue_tutorial::data::structures::*;
use test::Bencher;

fn random_level(d: Dim) -> LevelInfo {
    let mut rng = thread_rng();
    let m = Matrix::tabulate(d, |_| {
        [TileType::Wall, TileType::RoomWall, TileType::Ground]
            .choose(&mut rng)
            .unwrap()
            .to_owned()
    });
    LevelInfo::from_matrix(m)
}

fn bench_fov(ld: &LevelInfo, radius: DimIndex, b: &mut Bencher) {
    let pos = Pos {
        x: ld.width() / 2,
        y: ld.height() / 2,
    };
    b.iter(|| {
        calculate_fov(ld, pos, radius);
    });
}

#[bench]
fn fov_max(b: &mut Bencher) {
    let ld = random_level(Dim {
        width: DimIndex::max_value(),
        height: DimIndex::max_value(),
    });
    bench_fov(&ld, 30, b);
}

#[bench]
fn fov_small(b: &mut Bencher) {
    let ld = random_level(Dim {
        width: 100,
        height: 100,
    });

    bench_fov(&ld, 8, b);
}

#[bench]
fn fov_small_large_radius(b: &mut Bencher) {
    let ld = random_level(Dim {
        width: 100,
        height: 100,
    });

    bench_fov(&ld, 100, b);
}

#[bench]
fn fov_large_small_radius(b: &mut Bencher) {
    let ld = random_level(Dim {
        width: DimIndex::max_value(),
        height: DimIndex::max_value(),
    });

    bench_fov(&ld, 8, b);
}
