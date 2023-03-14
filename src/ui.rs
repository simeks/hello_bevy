use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, ScoreEvent, WinEvent};

use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct Score {
    pub score: i32,
}

pub fn setup_score_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Score { score: 0 });

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            )
                .with_alignment(TextAlignment::Right),
            transform: Transform::from_translation(Vec3::new(40.0, 20.0, 1.0)),
            ..default()
        },
        ScoreText,
    ));
}

pub fn score_events(
    mut scoreboard: ResMut<Score>,
    mut score_events: EventReader<ScoreEvent>,
) {
    for _ in score_events.iter() {
        scoreboard.score += 1;
    }
}

pub fn update_score(
    scoreboard: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if scoreboard.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Score: {}", scoreboard.score);
        }
    }
}

pub fn win_event(
    mut commands: Commands,
    ev_win: EventReader<WinEvent>,
    asset_server: Res<AssetServer>,
) {
    if !ev_win.is_empty() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                "You Win!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(
                Vec3::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0, 1.0)
            ),
            ..default()
        });
    }
}
