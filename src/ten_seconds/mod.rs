use crate::prelude::*;

use self::{
    assets::{loading_system, Sprites},
    field::{add_camera, highlighting::highlight_field_location_by_mouse, spawn_field},
};

pub mod assets;
pub mod field;
pub struct TenSecondTowersPlugin;

impl Plugin for TenSecondTowersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "10 Second Tower Defense".to_string(),
            width: 960.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .add_startup_system(watch_for_changes)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_system))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_field)
                .with_system(add_camera)
                .with_system(spawn_debug),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(highlight_field_location_by_mouse),
        );
    }
}

fn watch_for_changes(asset_server: ResMut<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
}

fn spawn_debug(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: sprites.field.clone(),
        sprite: TextureAtlasSprite::new(9),
        ..Default::default()
    });
}
