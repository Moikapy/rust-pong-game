use bevy::{prelude::*};
#[derive(Resource, Clone, Copy)]
pub struct Scoreboard {
    pub score: usize,
}
