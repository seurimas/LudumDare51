use bevy_inspector_egui::RegisterInspectable;

use crate::prelude::*;

use self::{
    assets::{loading_system, Sprites},
    enemies::{
        ai::{move_enemies, think_for_enemies, EnemyImpulses},
        spawn_enemy,
        waves::{wave_system, WaveStatus},
    },
    field::{
        highlighting::highlight_field_location_by_mouse, spawn_field, update_contents,
        FieldLocationContents,
    },
    towers::spawn_tower,
};

pub mod assets;
pub mod enemies;
pub mod field;
pub mod towers;
pub struct TenSecondTowersPlugin;

impl Plugin for TenSecondTowersPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<FieldLocationContents>()
            .register_inspectable::<FieldLocation>()
            .register_inspectable::<TowerType>()
            .register_inspectable::<EnemyType>()
            .register_inspectable::<EnemyImpulses>()
            .insert_resource(WaveStatus::default())
            .add_startup_system(watch_for_changes)
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_system))
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(spawn_field)
                    .with_system(add_camera)
                    .with_system(spawn_debug),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(highlight_field_location_by_mouse)
                    .with_system(update_contents)
                    .with_system(think_for_enemies)
                    .with_system(move_enemies)
                    .with_system(wave_system)
                    .with_system(spawn_debug_tower),
            );
    }
}

fn watch_for_changes(asset_server: ResMut<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}

pub fn add_camera(mut commands: Commands, windows: Res<Windows>) {
    let mut transform = Transform::default();
    if let Some(window) = windows.get_primary() {
        transform.translation = Vec3::new(window.width() / 2., window.height() / 2., 10.);
    }
    commands.spawn_bundle(Camera2dBundle {
        transform,
        ..Default::default()
    });
}

fn spawn_debug(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: sprites.field.clone(),
        sprite: TextureAtlasSprite::new(9),
        ..Default::default()
    });
    let mut transform = Transform::default();
    transform.translation = Vec3::new(100.0, 100.0, 1.0);
}

fn spawn_debug_tower(
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
