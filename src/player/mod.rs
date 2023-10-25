use bevy::{math::*, prelude::*};

#[derive(Resource, Clone, Copy, Component)]
pub struct Player {
    pub speed: f32,
    pub is_bot: bool,
}

impl Player {
    pub fn movement(
        input: Res<Input<KeyCode>>,
        time_step: Res<FixedTime>,
        mut query: Query<(&Player, &mut Transform)>,
        player: Query<&Player>,
    ) {
        for (player, mut player_transform) in &mut query.iter_mut() {
            //        let mut direction_x = 0.0;
            let mut direction_y = 0.0;
            if !player.is_bot {
                if input.pressed(KeyCode::W) {
                    direction_y += 1.0;
                }
                if input.pressed(KeyCode::S) {
                    direction_y -= 1.0;
                }

                let new_y = player_transform.translation.y
                    + direction_y * player.speed * time_step.period.as_secs_f32();

                //        player_transform.translation.x = new_x;
                player_transform.translation.y = new_y;
            } else {
            }
        }
    }
}
