mod art;
mod enemies;
mod player;
mod scoreboard;

use art::Sprites;
use bevy::{prelude::*, render::camera::ScalingMode};

const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;

#[derive(Resource)]
pub struct GameAssets {
    pub player: Handle<Image>,
    pub bullet: Handle<Image>,
}

pub struct BulletEvent {
    pub position: Vec3,
    pub velocity: Vec2,
}

pub struct ScoreEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Sprites>()
        .add_event::<BulletEvent>()
        .add_event::<ScoreEvent>()
        .add_startup_systems((
            setup,
            player::setup_player,
            enemies::setup_enemies,
            scoreboard::setup_scoreboard,
        ).chain())
        .add_systems((
            player::keyboard_input,
            player::spawn_bullets,
        ).chain())
        .add_system(player::bullet_movement)
        .add_systems((
            scoreboard::score_events,
            scoreboard::update_score,
        ).chain().after(player::bullet_movement))
        .add_system(enemies::enemy_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    mut sprites: ResMut<Sprites>,
) {
    *sprites = art::make_sprites(images);

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            viewport_origin: (0., 0.).into(),
            scaling_mode: ScalingMode::Fixed {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT
            },
            ..default()
        },
        ..default()
    });
}

