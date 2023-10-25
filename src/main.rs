mod gui;
use gui::GUI;
mod player;
use player::Player;
mod game_window;
use bevy::{asset::*, math::*, prelude::*, utils::Duration, window::WindowResized};
use game_window::{GameWindow, ResolutionSettings};

//player
const PLAYER_START_Y: f32 = 0.0;
const PLAYER_SIZE: Vec2 = Vec2::new(25.0, 200.0);
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PLAYER_SCORE: usize = 0;
const AI_SCORE: usize = 0;
const PLAYER_SPEED: f32 = 250.0;

//gui
pub const GUI_FONT_SIZE: f32 = 40.0;
pub const GUI_TEXT_PADDING: Val = Val::Px(5.0);
pub const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

//Main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
            ..Default::default()
        }))
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(GUI {
            player_score: PLAYER_SCORE,
            ai_score: AI_SCORE,
        })
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                GUI::update_gui,
                GameWindow::toggle_resolution,
            ),
        )
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, Player::movement)
        .run();
}

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct CollisionSound(Handle<AudioSource>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    //Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    //player
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(-600., PLAYER_START_Y, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            ..Default::default()
        },
        Player {
            speed: PLAYER_SPEED,
            is_bot: false,
        },
        Collider { size: PLAYER_SIZE },
    ));
    //AI
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(600., PLAYER_START_Y, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            ..Default::default()
        },
        Player {
            speed: PLAYER_SPEED,
            is_bot: true,
        },
        Collider { size: PLAYER_SIZE },
    ));

    //
    //Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font_size: GUI_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: GUI_FONT_SIZE,
                color: SCORE_COLOR,

                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: GUI_TEXT_PADDING,
            left: GUI_TEXT_PADDING,
            right: GUI_TEXT_PADDING * -1.0,
            ..default()
        }),
    );
}
