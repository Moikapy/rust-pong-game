pub mod Camera {
    use bevy::prelude::*;

    //2D
    pub fn camera_2d(mut commands: &Commands) {
        // Camera
        commands.spawn(Camera2dBundle::default());
    }
}
