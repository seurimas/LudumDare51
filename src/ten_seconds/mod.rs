use bevy_inspector_egui::RegisterInspectable;

use crate::prelude::*;

use self::{
    assets::{loading_system, Sprites},
    bullets::{update_bullets, Bullet},
    enemies::{
        ai::{move_enemies, think_for_enemies, EnemyImpulses},
        damaged::die_enemies,
        waves::{goal_system, wave_system, WaveEndEvent, WaveStatus},
    },
    field::{
        highlighting::highlight_field_location_by_mouse, spawn_field, update_contents,
        update_enemies_in_tiles, FieldLocationContents,
    },
    health::apply_basic_hits,
    towers::{
        ai::{shoot_for_towers, think_for_towers, turn_for_towers},
        loot_corpses,
        management::manage_towers,
        refresh_towers, spawn_tower,
    },
    ui::{
        init_game_over, init_ui,
        systems::{fade_in_game_over, update_countdown, update_health, update_resources},
    },
};

pub mod assets;
pub mod bullets;
pub mod enemies;
pub mod field;
pub mod health;
pub mod towers;
pub mod ui;
pub struct TenSecondTowersPlugin;

impl Plugin for TenSecondTowersPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<FieldLocationContents>()
            .register_inspectable::<FieldLocation>()
            .register_inspectable::<TowerType>()
            .register_inspectable::<EnemyType>()
            .register_inspectable::<Bullet>()
            .register_inspectable::<EnemyImpulses>()
            .register_inspectable::<Health>()
            .add_event::<BulletHitEvent>()
            .add_event::<DeathEvent>()
            .add_event::<WaveEndEvent>()
            .insert_resource(WaveStatus::default())
            .add_startup_system(watch_for_changes)
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_system))
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(spawn_field)
                    .with_system(add_camera)
                    .with_system(init_ui),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(highlight_field_location_by_mouse)
                    .with_system(update_contents)
                    .with_system(update_enemies_in_tiles)
                    .with_system(think_for_enemies)
                    .with_system(move_enemies)
                    .with_system(think_for_towers)
                    .with_system(shoot_for_towers)
                    .with_system(turn_for_towers)
                    .with_system(update_bullets)
                    .with_system(apply_basic_hits)
                    .with_system(wave_system)
                    .with_system(die_enemies)
                    .with_system(goal_system)
                    .with_system(update_countdown)
                    .with_system(update_resources)
                    .with_system(update_health)
                    .with_system(loot_corpses)
                    .with_system(refresh_towers)
                    .with_system(manage_towers),
            )
            .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(init_game_over))
            .add_system_set(
                SystemSet::on_update(AppState::GameOver)
                    .with_system(think_for_enemies)
                    .with_system(move_enemies)
                    .with_system(fade_in_game_over),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(cleanup_system::<InGameOnly>),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::GameOver)
                    .with_system(cleanup_system::<GameOverCleanup>),
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
    commands
        .spawn_bundle(Camera2dBundle {
            transform,
            ..Default::default()
        })
        .insert(GameOverCleanup);
}
