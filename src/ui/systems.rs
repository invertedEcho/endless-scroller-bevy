use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

pub fn spawn_main_menu(mut commands: Commands) {
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
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    Button,
                    BorderColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Play"));
                });
        });
}
