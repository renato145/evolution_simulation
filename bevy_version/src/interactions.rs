use bevy::prelude::*;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition(None))
            .add_system(track_mouse);
    }
}

pub struct MousePosition(pub Option<Vec2>);

fn track_mouse(
    mut mouse_position: ResMut<MousePosition>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_left: EventReader<CursorLeft>,
) {
    if cursor_left.iter().last().is_some() {
        mouse_position.0 = None;
        info!("Mouse left")
    } else if let Some(cursor) = cursor_moved.iter().last() {
        mouse_position.0 = Some(cursor.position);
    }
}
