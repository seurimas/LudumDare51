use crate::{prelude::*, ten_seconds::field::TILE_SIZE};

use super::FieldLocationContents;

#[derive(PartialEq, Component, Debug)]
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
        &FieldLocationContents,
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
            let tile =
                get_tile_from_screen_pick(window, position, camera, camera_transform, &field);
            if let Some((tile_x, tile_y)) = tile {
                for (location, contents, mut highlight, mut sprite, mut visibility) in
                    query.iter_mut()
                {
                    let new_highlight = match (tile_x - location.0, tile_y - location.1) {
                        (0, 0) => FieldLocationHighlight::Available,
                        (1, 0) | (-1, 0) | (0, 1) | (0, -1) => FieldLocationHighlight::Nearby,
                        (1, 1) | (-1, 1) | (1, -1) | (-1, -1) => FieldLocationHighlight::Nearby,
                        _ => FieldLocationHighlight::None,
                    };
                    let new_highlight = match (new_highlight, contents) {
                        (FieldLocationHighlight::None, _) => FieldLocationHighlight::None,
                        (FieldLocationHighlight::Available, FieldLocationContents::None) => {
                            FieldLocationHighlight::Available
                        }
                        (FieldLocationHighlight::Available, _) => {
                            FieldLocationHighlight::Unavailable
                        }
                        (n_highlight, _) => n_highlight,
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
            } else {
                for (_location, _contents, _highlight, _sprite, mut visibility) in query.iter_mut()
                {
                    visibility.is_visible = false;
                }
            }
        }
    }
}
