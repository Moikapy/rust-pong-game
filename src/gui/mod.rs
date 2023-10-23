use bevy::{prelude::*};

#[derive(Resource, Clone, Copy)]
pub struct GUI {
    pub health: usize,
}

impl GUI{
    pub fn update_gui(health: Res<GUI>, mut query: Query<&mut Text>) {
        let mut text = query.single_mut();
        text.sections[1].value = health.health.to_string();
    }
}