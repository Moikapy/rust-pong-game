use bevy::{prelude::*, window::*};

#[derive(Resource, Clone, Copy)]
pub struct GameWindow {
    width: usize,
    height: usize,
}

/// Stores the various window-resolutions we can select between.
#[derive(Resource)]
pub struct ResolutionSettings {
    pub large: Vec2,
    pub medium: Vec2,
    pub small: Vec2,
}

impl GameWindow {
    pub fn setup_window(mut windows: Query<&mut Window>, resolution: Res<ResolutionSettings>) {
        let mut window = windows.single_mut();
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    /// This system shows how to request the window to a new resolution
    pub fn toggle_resolution(
        keys: Res<Input<KeyCode>>,
        mut windows: Query<&mut Window>,
        resolution: Res<ResolutionSettings>,
    ) {
        let mut window = windows.single_mut();

        if keys.just_pressed(KeyCode::Key1) {
            let res = resolution.small;
            window.resolution.set(res.x, res.y);
        }
        if keys.just_pressed(KeyCode::Key2) {
            let res = resolution.medium;
            window.resolution.set(res.x, res.y);
        }
        if keys.just_pressed(KeyCode::Key3) {
            let res = resolution.large;
            window.resolution.set(res.x, res.y);
        }
    }
    //    fn update_paddle_scale(
    //        mut window_events: EventReader<WindowResized>,
    //        mut window_resolution: Query<&mut Window>,
    //        mut paddle_transforms: Query<&mut Transform, With<Sprite>>,
    //    ) {
    //        for event in window_events.iter() {
    //            // Get the new window resolution.
    //            let new_resolution = event.resolution;
    //
    //            // Update the window resolution.
    //            window_resolution.set(new_resolution.width, new_resolution.height);
    //
    //            // Update the paddle's scale.
    //            for mut paddle_transform in paddle_transforms.iter_mut() {
    //                let paddle_size = paddle_transform.scale;
    //
    //                // Calculate the new scale factor.
    //                let scale_factor = window_resolution.get().width / paddle_size.x;
    //
    //                // Set the paddle's scale.
    //                paddle_transform.scale = Vec3::new(scale_factor, 1.0, 1.0);
    //            }
    //        }
    //    }
}
