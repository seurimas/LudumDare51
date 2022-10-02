use crate::{prelude::*, ten_seconds::field::FieldLocationContents};

use super::spawn_tower;

pub fn manage_towers(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut field: ResMut<Field>,
    mut wave_status: ResMut<WaveStatus>,
    input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut field_location_query: Query<&mut FieldLocationContents>,
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
                        let tower_type = wave_status.tower_type;
                        if valid_location && wave_status.buy(tower_type) {
                            spawn_tower(
                                &mut commands,
                                &sprites,
                                &mut field,
                                location,
                                tower_type,
                                field_location_query,
                            );
                        }
                    }
                }
            }
        }
    } else if input.just_pressed(MouseButton::Right) {
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
                        let contents = field.get_contents(&location);
                        if let FieldLocationContents::Tower(tower_entity, tower_type) = contents {
                            wave_status.sell(*tower_type);
                            commands.entity(*tower_entity).despawn_recursive();
                            if let Ok(mut field_location_contents) =
                                field_location_query.get_mut(*field.get_entity(&location))
                            {
                                *field_location_contents = FieldLocationContents::None;
                            }
                        }
                    }
                }
            }
        }
    }
}
