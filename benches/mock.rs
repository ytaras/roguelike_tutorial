#![feature(test)]
extern crate rogue_tutorial;
extern crate specs;
extern crate test;
#[macro_use]
extern crate specs_derive;

use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::systems::logic::*;
use rogue_tutorial::*;
use specs::prelude::*;
use specs::RunNow;
use specs_derive::*;
use test::Bencher;

const ENTITIES: u32 = 1_000_000;

#[bench]
fn bench_unique_assertion(b: &mut Bencher) {
    let mut w = World::new();
    w.register::<IsPlayer>();
    w.create_entity().with(IsPlayer).build();
    for _ in 0..ENTITIES {
        w.create_entity().build();
    }
    let mut system: AssertUnique<IsPlayer> = Default::default();
    b.iter(|| {
        system.run_now(&w.res);
    });
}

#[bench]
fn bench_unique_query(b: &mut Bencher) {
    let mut w = World::new();
    w.register::<IsPlayer>();
    w.create_entity().with(IsPlayer).build();
    for _ in 0..ENTITIES {
        w.create_entity().with(IsPlayer).build();
    }

    let storage = &w.read_storage::<IsPlayer>();
    b.iter(|| {
        let _ = common::query::unique(storage);
    });
}

#[bench]
fn bench_small_hash(b: &mut Bencher) {
    let mut w = World::new();
    w.register::<Pos>();
    w.register::<Data>();
    for i in 0..100 {
        w.create_entity()
            .with(Pos { x: i, y: i })
            .with(Data(i))
            .build();
    }
    let s1 = &w.read_storage::<Pos>();
    let s2 = &w.read_storage::<Data>();

    b.iter(|| {
        let _ = common::query::hash(s1, s2);
    });
}

#[bench]
fn bench_large_hash(b: &mut Bencher) {
    let mut w = World::new();
    w.register::<Pos>();
    w.register::<Data>();
    for i in 0..u16::max_value() {
        w.create_entity()
            .with(Pos { x: i, y: i })
            .with(Data(i))
            .build();
    }
    let s1 = &w.read_storage::<Pos>();
    let s2 = &w.read_storage::<Data>();

    b.iter(|| {
        let _ = common::query::hash(s1, s2);
    });
}

#[bench]
fn bench_matrix_creation(b: &mut Bencher) {
    let h = 1000;
    let w = 1000;

    b.iter(|| {
        let _: data::structures::matrix::Matrix<bool> = data::structures::matrix::Matrix::new(h, w);
    });
}
#[derive(Component)]
struct Data(u16);
