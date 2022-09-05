use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health {
    pub(crate) current: i32,
    pub(crate) max: i32,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Name(pub String);
