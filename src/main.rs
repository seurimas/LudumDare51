use crate::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};
use ten_seconds::TenSecondTowersPlugin;

#[macro_use]
extern crate lazy_static;

mod bt;
mod prelude;
mod ten_seconds;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "10 Second Tower Defense".to_string(),
            width: 960.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Loading)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TenSecondTowersPlugin)
        .run();
}
