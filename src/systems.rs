use bevy::{prelude::*, window::WindowResized};

use crate::components::RelevantForDespawnOnResize;
use crate::physics::events::RedrawGroundColliderEvent;
use crate::player::events::RedrawKnightEvent;
use crate::scrolling_background::events::RedrawScrollingBackgroundEvent;

use crate::resources::WindowDimensions;

pub fn on_resize_system(
    mut commands: Commands,
    mut resizer_read: EventReader<WindowResized>,
    mut scrolling_background_redraw_event_writer: EventWriter<RedrawScrollingBackgroundEvent>,
    mut knight_redraw_event_writer: EventWriter<RedrawKnightEvent>,
    mut ground_collider_event_writer: EventWriter<RedrawGroundColliderEvent>,
    entities_to_despawn: Query<Entity, With<RelevantForDespawnOnResize>>,
    mut window_dimensions: ResMut<WindowDimensions>,
) {
    for window_resized_event in resizer_read.read() {
        for entity in entities_to_despawn {
            println!("Despawning relevant entity: {:?}", entity);
            commands.entity(entity).despawn();
        }

        window_dimensions.width = window_resized_event.width;
        window_dimensions.height = window_resized_event.height;

        // TODO: This will be too much at some point, figure out a better way to do this
        scrolling_background_redraw_event_writer.write(RedrawScrollingBackgroundEvent);
        knight_redraw_event_writer.write(RedrawKnightEvent);
        ground_collider_event_writer.write(RedrawGroundColliderEvent);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
