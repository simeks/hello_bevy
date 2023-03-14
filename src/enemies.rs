use crate::art::Sprites;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use bevy::prelude::*;

const NUM_ENEMIES: usize = 15;
const VELOCITY: f32 = 20.0;

#[derive(Component)]
pub struct Enemy;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Resource)]
pub struct EnemyMovement {
    direction: Direction,
    bounds: [f32; 2],
    timer: Timer,
}

pub fn setup_enemies(
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    let mut bounds = [0.0; 2];
    for row in 0..4 {
        for i in 0..(NUM_ENEMIES-row%2) {
            let x = 40.0 * (i as f32) + 20.0 + (row % 2) as f32 * 20.0;
            bounds[1] = x.max(bounds[1]);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 1.0),
                        ..default()
                    },
                    texture: sprites.enemy.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x,
                        SCREEN_HEIGHT - 20.0 - (row as f32) * 40.0,
                        0.0,
                    ))
                        .with_scale(Vec3::splat(5.0)),
                    ..default()
                },
                Enemy,
            ));
        }
    }

    commands.insert_resource(EnemyMovement {
        direction: Direction::Right,
        bounds,
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}

pub fn enemy_movement(
    time: Res<Time>,
    mut movement: ResMut<EnemyMovement>,
    mut query : Query<&mut Transform, With<Enemy>>,
) {
    movement.timer.tick(time.delta());

    if movement.timer.just_finished() {
        if movement.bounds[0] < 20.0 {
            movement.direction = Direction::Right;
        } else if movement.bounds[1] > SCREEN_WIDTH - 40.0 {
            movement.direction = Direction::Left;
        }

        if movement.direction == Direction::Right {
            movement.bounds[0] += VELOCITY;
            movement.bounds[1] += VELOCITY;
        } else {
            movement.bounds[0] -= VELOCITY;
            movement.bounds[1] -= VELOCITY;
        }

        for mut transform in query.iter_mut() {
            if movement.direction == Direction::Right {
                transform.translation.x += VELOCITY;
                if transform.translation.x > movement.bounds[1] {
                    movement.direction = Direction::Left;
                }
            } else {
                transform.translation.x -= VELOCITY;
                if transform.translation.x < movement.bounds[0] {
                    movement.direction = Direction::Right;
                }
            }
        }
    }
}