use crate::{prelude::*, ten_seconds::field::FieldLocationContents};

use super::spawn_tower;

fn set_helper_text(tower_type: TowerType, mut helper_text_query: Query<(&mut Text, &Name)>) {
    let flavor = match tower_type {
        TowerType::Attack => "Simple gun tower.",
        TowerType::Silo => "Resupplies neighbors.",
        TowerType::Burst => "Fires in four directions.",
        TowerType::Triple => "Fires 3-shot bursts.",
        TowerType::BigBomb => "Fires really big shots.",
    };
    let value = format!(
        "{}\nAmmo: {} - Costs: ",
        flavor,
        tower_type.get_cooldowns().ammo_left
    );
    for (mut text, name) in helper_text_query.iter_mut() {
        if name.eq_ignore_ascii_case("Info") {
            text.sections[0].value = value;
            text.sections[1].value = format!("{}", tower_type.get_mineral_cost());
            text.sections[3].value = format!("{}", tower_type.get_dust_cost());
            text.sections[5].value = format!("{}", tower_type.get_tech_cost());
            return;
        }
    }
}

fn highlight_icon(
    old_tower_type: TowerType,
    tower_type: TowerType,
    mut icon_query: Query<(&mut TextureAtlasSprite, &Name)>,
) {
    let tower_helper_name = get_helper_name(tower_type);
    let old_tower_helper_name = get_helper_name(old_tower_type);
    for (mut sprite, name) in icon_query.iter_mut() {
        if name.eq_ignore_ascii_case(tower_helper_name) {
            sprite.color = Color::rgb(0.435, 1., 0.384);
        } else if name.eq_ignore_ascii_case(old_tower_helper_name) {
            sprite.color = Color::WHITE;
        }
    }
}

fn get_helper_name(tower_type: TowerType) -> &'static str {
    let tower_helper_name = match tower_type {
        TowerType::Attack => "AttackHelper",
        TowerType::Silo => "SiloHelper",
        TowerType::Burst => "BurstHelper",
        TowerType::Triple => "TripleHelper",
        TowerType::BigBomb => "BigBombHelper",
    };
    tower_helper_name
}

pub fn switch_tower_types(
    mut wave_status: ResMut<WaveStatus>,
    input: Res<Input<KeyCode>>,
    helper_text_query: Query<(&mut Text, &Name)>,
    icon_query: Query<(&mut TextureAtlasSprite, &Name)>,
) {
    let old_tower_type = wave_status.tower_type;
    if input.just_pressed(KeyCode::Key1) {
        wave_status.tower_type = TowerType::Attack;
    } else if input.just_pressed(KeyCode::Key2) {
        wave_status.tower_type = TowerType::Silo;
    } else if input.just_pressed(KeyCode::Key3) {
        wave_status.tower_type = TowerType::Triple;
    } else if input.just_pressed(KeyCode::Key4) {
        wave_status.tower_type = TowerType::BigBomb;
    } else {
        return;
    }
    set_helper_text(wave_status.tower_type, helper_text_query);
    highlight_icon(old_tower_type, wave_status.tower_type, icon_query);
}

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
