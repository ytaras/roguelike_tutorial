use super::structures::*;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct HasPos {
    pub x: u32,
    pub y: u32,
}

type Color = (u8, u8, u8, u8);
pub const RED: Color = (255, 0, 0, 255);

#[derive(Component, Debug)]
pub struct IsVisible {
    pub display_char: char,
    pub color: Color,
}

#[derive(Component, Debug)]
pub struct IsPlayer;

#[derive(Component, Debug)]
pub struct PlansExecuting(ActorCommand);
impl PlansExecuting {
    pub fn new(ac: ActorCommand) -> Self {
        PlansExecuting(ac)
    }
}
