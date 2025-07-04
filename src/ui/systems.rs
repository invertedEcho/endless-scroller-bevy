use bevy::prelude::*;

use crate::{score::resources::ScoreResource, states::GameState};

use super::components::{DeadMenu, MainMenu, PlayButton, ScoreText};

pub fn spawn_main_menu(mut commands: Commands, game_state: Res<State<GameState>>) {
    let game_state = game_state.get();
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(Text::new("Endless Scroller"));
                });
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    Button,
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new("Play"), TextFont { ..default() }));
                });
        });
}

pub fn handle_play_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_game_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_entity_query: Query<Entity, With<MainMenu>>,
) {
    println!("Despawning main menu");
    let main_menu_entity = main_menu_entity_query
        .single()
        .expect("Exactly one main menu exists");
    commands.entity(main_menu_entity).despawn();
}

pub fn spawn_dead_menu(mut commands: Commands, score_resource: Res<ScoreResource>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.),
                ..default()
            },
            DeadMenu,
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("You died. Wanna play again?"));
            parent.spawn(Text::new(format!("Your score: {}", score_resource.count)));
            parent.spawn((Button, PlayButton, Text::new("Play again")));
        });
}

pub fn despawn_dead_menu(mut commands: Commands, dead_menu_query: Query<Entity, With<DeadMenu>>) {
    let dead_menu_entity = dead_menu_query
        .single()
        .expect("Exactly one dead menu entity exists");
    commands.entity(dead_menu_entity).despawn();
}

pub fn spawn_score_text(mut commands: Commands, score: Res<ScoreResource>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((Text::new(format!("Score: {}", score.count)), ScoreText));
        });
}

pub fn despawn_score_text(
    mut commands: Commands,
    score_text_query: Query<Entity, With<ScoreText>>,
) {
    let score_text_entity = score_text_query
        .single()
        .expect("Exactly one score text exists");
    commands.entity(score_text_entity).despawn();
}
