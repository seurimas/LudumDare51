use crate::prelude::*;

#[derive(PartialEq, Component)]
pub enum FieldLocationHighlight {
    None,
    Nearby,
    Available,
    Unavailable,
    Filled,
}

pub fn highlight_field_location_by_mouse(
    field: Res<Field>,
    mut query: Query<(
        &FieldLocation,
        &mut FieldLocationHighlight,
        &mut TextureAtlasSprite,
        &mut Visibility,
    )>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    if let Some(window) = windows.get_primary() {
        if let Some(position) = window.cursor_position() {
            let real_loc = screen_to_world(window, position, camera, camera_transform);
            let tile_x = (real_loc.x / field.tile_size as f32).floor() as i32;
            let tile_y = (real_loc.y / field.tile_size as f32).floor() as i32;
            println!(
                "{:?} {:?} {:?}",
                position,
                (real_loc.x, real_loc.y),
                (tile_x, tile_y)
            );
            if tile_x < 0 || tile_x >= field.width || tile_y < 0 || tile_y >= field.height {
                for (_location, _highlight, _sprite, mut visibility) in query.iter_mut() {
                    visibility.is_visible = false;
                }
                return;
            }
            for (location, mut highlight, mut sprite, mut visibility) in query.iter_mut() {
                let new_highlight = match (tile_x - location.0, tile_y - location.1) {
                    (0, 0) => FieldLocationHighlight::Available,
                    (1, 0) | (-1, 0) | (0, 1) | (0, -1) => FieldLocationHighlight::Nearby,
                    (1, 1) | (-1, 1) | (1, -1) | (-1, -1) => FieldLocationHighlight::Nearby,
                    _ => FieldLocationHighlight::None,
                };
                sprite.index = match new_highlight {
                    FieldLocationHighlight::Available => 3,
                    FieldLocationHighlight::Unavailable => 4,
                    FieldLocationHighlight::Nearby => 5,
                    _ => 5, // Doesn't matter, will be hidden.
                };
                visibility.is_visible = match new_highlight {
                    FieldLocationHighlight::Filled | FieldLocationHighlight::None => false,
                    _ => true,
                };
                *highlight = new_highlight;
            }
        }
    }
}
