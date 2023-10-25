use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct GUI {
    pub player_score: usize,
    pub ai_score: usize,
}

impl GUI {
    pub fn update_gui(score: Res<GUI>, mut query: Query<&mut Text>) {
        let mut text = query.single_mut();
        text.sections[1].value =
            score.player_score.to_string() + " - " + score.ai_score.to_string().as_str();
    }
}
