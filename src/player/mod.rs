use bevy::{math::*, prelude::*};

#[derive(Resource, Clone, Copy,Component)]
pub struct Player{
    pub health:  usize,
    pub speed: f32,
}

impl Player{
    pub fn movement(           
        input: Res<Input<KeyCode>>,
        time_step: Res<FixedTime>,
        mut query: Query<&mut Transform,With<Player>>,
        player: Query<&Player>
        ) {
        let mut player_transform = query.single_mut();
        let  player = player.single();
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;
        if input.pressed(KeyCode::W) {
            direction_y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction_y -= 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction_x -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction_x += 1.0;
        }
      
        let new_x =
        player_transform.translation.x + direction_x * player.speed * time_step.period.as_secs_f32();
        let new_y =
        player_transform.translation.y + direction_y * player.speed * time_step.period.as_secs_f32();

        player_transform.translation.x = new_x;
        player_transform.translation.y = new_y;
    }
}