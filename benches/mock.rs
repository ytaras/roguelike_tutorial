#![feature(test)]
extern crate rogue_tutorial;
extern crate specs;
extern crate test;

use rogue_tutorial::data::components::*;
use rogue_tutorial::systems::logic::*;
use specs::prelude::*;
use specs::RunNow;
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
