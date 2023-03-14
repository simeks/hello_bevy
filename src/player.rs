use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, BulletEvent, ScoreEvent};
use crate::art::Sprites;
use crate::enemies::Enemy;

use bevy::prelude::*;

const VELOCITY : f32 = 100.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec2,
}

pub fn setup_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                ..default()
            },
            texture: sprites.player.clone(),
            transform: Transform::from_translation(Vec3::new(0.5*SCREEN_WIDTH, 20., 0.))
                .with_scale(Vec3::splat(5.0)),
            ..default()
        },
        Player
    ));
}

pub fn keyboard_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    mut ev_bullet: EventWriter<BulletEvent>,
) {
    for (mut transform, _) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= VELOCITY * time.delta_seconds();
            transform.translation.x = transform.translation.x.max(10.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += VELOCITY * time.delta_seconds();
            transform.translation.x = transform.translation.x.min(SCREEN_WIDTH - 10.0);
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            ev_bullet.send(crate::BulletEvent {
                position: transform.translation,
                velocity: Vec2 { x: 0.0, y: 100.0 },
            });
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut events: EventReader<BulletEvent>,
) {
    for event in events.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 0.0, 1.0),
                    ..default()
                },
                texture: sprites.bullet.clone(),
                transform: Transform::from_translation(
                    event.position + Vec3::new(0.0, 20.0, 0.0)
                    )
                        .with_scale(Vec3::splat(5.0)),
                ..default()
            },
            Bullet {
                velocity: event.velocity,
            }
        ));
    }
}

pub fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Bullet), With<Bullet>>,
    query_enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Bullet>)>,
    mut ev_score: EventWriter<ScoreEvent>,
) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        transform.translation += Vec3::new(bullet.velocity.x, bullet.velocity.y, 0.0) * time.delta_seconds();

        if transform.translation.y > SCREEN_HEIGHT {
            commands.entity(entity).despawn();
        }

        for (enemey_entity, enemy_transform) in query_enemies.iter() {
            if transform.translation.distance(enemy_transform.translation) < 20.0 {
                commands.entity(entity).despawn();
                commands.entity(enemey_entity).despawn();

                ev_score.send(ScoreEvent {});
            }
        }
    }
}
