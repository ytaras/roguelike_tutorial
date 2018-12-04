use specs::prelude::*;
use specs_derive::*;

use super::structures::*;

pub use self::builder::*;

mod builder;

impl<'a> std::ops::AddAssign<&'a Dir> for &'a mut Pos {
    fn add_assign(&mut self, other: &'a Dir) {
        let o = **self + *other;
        self.x = o.x;
        self.y = o.y;
    }
}

impl<'a> std::ops::Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, other: Dir) -> Pos {
        let x: DimIndex = other.ew.apply(self.x);
        let y: DimIndex = other.ns.apply(self.y);
        Pos { x, y }
    }
}

type Color = (u8, u8, u8, u8);

#[derive(Component, Debug)]
pub struct IsVisible {
    pub display_char: char,
    pub color: Color,
}

#[derive(Component, Debug, Default)]
pub struct IsPlayer;

#[derive(Component, Debug, Default)]
pub struct TakesWholeTile;

#[derive(Component, Debug, PartialEq)]
pub struct PlansExecuting(pub ActorCommand);
impl PlansExecuting {
    pub fn new(ac: ActorCommand) -> Self {
        PlansExecuting(ac)
    }
}
