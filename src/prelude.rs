pub use crate::bt::*;
pub use crate::ten_seconds::field::{Field, FieldLocation};
use bevy::app::AppLabel;
pub use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, AppLabel)]
pub enum AppState {
    Loading,
    InGame,
}

pub fn screen_to_world(
    wnd: &Window,
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    // get the size of the window
    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    world_pos.truncate()
}
