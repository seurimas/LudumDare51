use crate::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use ten_seconds::TenSecondTowersPlugin;

#[macro_use]
extern crate lazy_static;

mod bt;
mod prelude;
mod ten_seconds;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Loading)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TenSecondTowersPlugin)
        .run();
}
