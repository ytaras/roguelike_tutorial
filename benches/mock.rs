#![feature(test)]
extern crate rogue_tutorial;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate test;

use test::Bencher;

use specs::prelude::*;
use specs::RunNow;
use specs_derive::*;

use rogue_tutorial::data::components::*;
use rogue_tutorial::data::structures::*;
use rogue_tutorial::systems::logic::*;
use rogue_tutorial::*;

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
    w.register::<HasPos>();
    w.register::<Data>();
    for i in 0..100 {
        w.create_entity()
            .with(HasPos(Pos { x: i, y: i }))
            .with(Data(i))
            .build();
    }
    let s1 = &w.read_storage::<HasPos>();
    let s2 = &w.read_storage::<Data>();

    b.iter(|| {
        let _ = common::query::hash(s1, s2);
    });
}

#[bench]
fn bench_large_hash(b: &mut Bencher) {
    let mut w = World::new();
    w.register::<HasPos>();
    w.register::<Data>();
    for i in 0..DimIndex::max_value() {
        w.create_entity()
            .with(HasPos(Pos { x: i, y: i }))
            .with(Data(i))
            .build();
    }
    let s1 = &w.read_storage::<HasPos>();
    let s2 = &w.read_storage::<Data>();

    b.iter(|| {
        let _ = common::query::hash(s1, s2);
    });
}

#[derive(Component)]
struct Data(DimIndex);
