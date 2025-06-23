use bevy::{prelude::*, window::WindowResized};

use crate::background_image::components::BackgroundImage;
use crate::background_image::events::RedrawScrollingBackgroundEvent;
use crate::components::{RelevantForMoveX, RelevantForMoveY};
use crate::events::RelevantForMoveYEvent;

use crate::resources::WindowDimensions;
use crate::utils::get_y_of_ground;

const SCROLLING_SPEED: f32 = 100.0;

pub fn on_resize_system(
    mut commands: Commands,
    background_image_query: Query<Entity, With<BackgroundImage>>,
    mut resizer_read: EventReader<WindowResized>,
    mut scrolling_background_redraw_event_writer: EventWriter<RedrawScrollingBackgroundEvent>,
    mut relevant_for_move_y_event_writer: EventWriter<RelevantForMoveYEvent>,
    mut window_dimensions: ResMut<WindowDimensions>,
) {
    for window_resized_event in resizer_read.read() {
        for background_image in background_image_query {
            println!("Despawning background image: {:?}", background_image);
            commands.entity(background_image).despawn();
        }

        window_dimensions.width = window_resized_event.width;
        window_dimensions.height = window_resized_event.height;

        scrolling_background_redraw_event_writer.write(RedrawScrollingBackgroundEvent);
        relevant_for_move_y_event_writer.write(RelevantForMoveYEvent);
    }
}

pub fn handle_relevant_for_move_y_event(
    mut event_reader: EventReader<RelevantForMoveYEvent>,
    query: Query<&mut Transform, With<RelevantForMoveY>>,
    window_dimensions: Res<WindowDimensions>,
) {
    if event_reader.read().next().is_some() {
        // TODO: I think we need to freeze player so it cant fall
        let y_of_ground = get_y_of_ground(window_dimensions.height);
        for mut entity in query {
            entity.translation.y = y_of_ground;
        }
    }
}

pub fn scrolling_system(query: Query<&mut Transform, With<RelevantForMoveX>>, time: Res<Time>) {
    for mut transform in query {
        let current_translation_x = transform.translation.x;

        let new_translation_x = current_translation_x + SCROLLING_SPEED * time.delta_secs();

        transform.translation.x = new_translation_x;
    }
}
