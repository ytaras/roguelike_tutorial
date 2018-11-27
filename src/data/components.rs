use super::structures::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug, PartialEq)]
pub struct HasPos {
    pub x: u32,
    pub y: u32,
}

impl<'a> std::ops::AddAssign<&'a Dir> for &'a mut HasPos {
    fn add_assign(&mut self, other: &'a Dir) {
        // FIXME - this can have underflow
        self.x = (self.x as i8 + other.ew.to_int()) as u32;
        self.y = (self.y as i8 + other.ns.to_int()) as u32;
    }
}

type Color = (u8, u8, u8, u8);
pub const RED: Color = (255, 0, 0, 255);

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
