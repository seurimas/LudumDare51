use crate::{prelude::*, ten_seconds::field::FieldLocationContents};

use super::spawn_tower;

pub fn manage_towers(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut field: ResMut<Field>,
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    field_location_query: Query<&mut FieldLocationContents>,
) {
    if input.just_pressed(MouseButton::Left) {
        if let Ok((camera, camera_transform)) = q_camera.get_single() {
            if let Some(window) = windows.get_primary() {
                if let Some(position) = window.cursor_position() {
                    let tower_loc = get_tile_from_screen_pick(
                        window,
                        position,
                        camera,
                        camera_transform,
                        &field,
                    );
                    if let Some((tile_x, tile_y)) = tower_loc {
                        let location = FieldLocation(tile_x, tile_y);
                        let valid_location =
                            is_valid_tower_location(&field_location_query, &field, location);
                        if valid_location {
                            spawn_tower(
                                &mut commands,
                                &sprites,
                                &mut field,
                                location,
                                TowerType::Attack,
                                field_location_query,
                            );
                        }
                    }
                }
            }
        }
    }
}
