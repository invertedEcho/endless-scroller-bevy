use bevy::{prelude::*, window::WindowResized};

use crate::background_image::events::RedrawBackgroundEvent;
use crate::player::events::RedrawKnightEvent;

use crate::{components::RelevantForDespawnOnResize, resources::WindowDimensions};

pub fn on_resize_system(
    mut commands: Commands,
    mut resizer_read: EventReader<WindowResized>,
    mut background_redraw_event_writer: EventWriter<RedrawBackgroundEvent>,
    mut knight_redraw_event_writer: EventWriter<RedrawKnightEvent>,
    entities_to_despawn: Query<Entity, With<RelevantForDespawnOnResize>>,
    mut window_dimensions: ResMut<WindowDimensions>,
) {
    for window_resized_event in resizer_read.read() {
        for entity in entities_to_despawn {
            println!("Despawning entity: {:?}", entity);
            commands.entity(entity).despawn();
        }

        window_dimensions.width = window_resized_event.width;
        window_dimensions.height = window_resized_event.height;

        // TODO: This will be too much at some point, figure out a better way to do this
        background_redraw_event_writer.write(RedrawBackgroundEvent);
        knight_redraw_event_writer.write(RedrawKnightEvent);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
