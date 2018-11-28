use super::structures::*;
use specs::prelude::*;
use specs_derive::*;

impl<'a> std::ops::AddAssign<&'a Dir> for &'a mut Pos {
    fn add_assign(&mut self, other: &'a Dir) {
        // FIXME - this can have underflow
        self.x = (self.x as i8 + other.ew.to_int()) as DimIndex;
        self.y = (self.y as i8 + other.ns.to_int()) as DimIndex;
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

#[derive(Component, Debug, PartialEq)]
pub struct PlansExecuting(pub ActorCommand);
impl PlansExecuting {
    pub fn new(ac: ActorCommand) -> Self {
        PlansExecuting(ac)
    }
}
