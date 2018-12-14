use specs::prelude::*;
use specs_derive::*;

use crate::data::structures::matrix::Matrix;
use crate::systems::render::Color;

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

#[derive(Component, PartialEq, Eq, Hash, Debug)]
pub struct HasPos(pub Pos);

#[derive(Component)]
pub struct HasVision {
    pub radius: DimIndex,
    fov: Option<Matrix<bool>>,
    memory: Option<Matrix<bool>>,
}

#[derive(Component, Default)]
pub struct HasEffectStack {
    pub hp_damage: i32,
}

impl HasEffectStack {
    pub fn hp(hp_damage: i32) -> Self {
        HasEffectStack { hp_damage }
    }
}

impl HasVision {
    pub fn new(radius: DimIndex) -> Self {
        HasVision {
            radius,
            fov: None,
            memory: None,
        }
    }
    pub fn expire_fov(&mut self) {
        self.fov = None;
    }
    pub fn fov(&self) -> Option<&Matrix<bool>> {
        self.fov.as_ref()
    }

    pub fn set_fov(&mut self, m: Matrix<bool>) {
        if self.memory.is_none() {
            self.memory = Some(Matrix::new(m.width(), m.height()));
        }
        let mem = self.memory.as_mut().unwrap();
        for (p, v) in m.iter() {
            if *v {
                mem[p] = true;
            }
        }
        self.fov = Some(m);
    }

    pub fn memory(&self) -> Option<&Matrix<bool>> {
        self.memory.as_ref()
    }
}

#[derive(Component)]
pub struct HasBrain {}
