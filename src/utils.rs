use bevy::prelude::*;

pub fn get_cursor_pos(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    ) -> Option<Vec2> {
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return None;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return None;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return None;
    };
    
    Some(point)
}