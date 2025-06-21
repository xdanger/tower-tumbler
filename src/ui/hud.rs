use crate::GameScore;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
}

pub fn update_score_display(score: Res<GameScore>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Score: {}", score.current);
        }
    }
}
