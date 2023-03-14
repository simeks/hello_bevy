use crate::ScoreEvent;

use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: i32,
}

pub fn setup_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Scoreboard { score: 0 });

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
            transform: Transform::from_translation(Vec3::new(40.0, 20.0, 0.0)),
            ..default()
        },
        ScoreText,
    ));
}

pub fn score_events(
    mut scoreboard: ResMut<Scoreboard>,
    mut score_events: EventReader<ScoreEvent>,
) {
    for _ in score_events.iter() {
        scoreboard.score += 1;
    }
}

pub fn update_score(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if scoreboard.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Score: {}", scoreboard.score);
        }
    }
}
