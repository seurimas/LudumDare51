use std::ops::{Deref, DerefMut};

pub use crate::bt::*;
pub use crate::ten_seconds::assets::Sprites;
pub use crate::ten_seconds::enemies::EnemyType;
use crate::ten_seconds::field::FieldLocationContents;
pub use crate::ten_seconds::field::{Field, FieldLocation};
pub use crate::ten_seconds::towers::TowerType;
use bevy::app::AppLabel;
pub use bevy::prelude::*;
pub use bevy_inspector_egui::Inspectable;
use pathfinding::prelude::astar;

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

pub fn get_tile_from_screen_pick(
    window: &Window,
    position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    field: &impl Deref<Target = Field>,
) -> Option<(i32, i32)> {
    let mut location = screen_to_world(window, position, camera, camera_transform);
    location.x -= field.tile_size / 2.;
    location.y -= field.tile_size / 4.;
    get_tile_from_location(location, field)
}

pub fn get_tile_from_location(
    location: Vec2,
    field: &impl Deref<Target = Field>,
) -> Option<(i32, i32)> {
    let tile_x = (location.x / field.tile_size as f32).floor() as i32;
    let tile_y = (location.y / field.tile_size as f32).floor() as i32;
    if tile_x < 0 || tile_x >= field.width || tile_y < 0 || tile_y >= field.height {
        None
    } else {
        Some((tile_x, tile_y))
    }
}

pub fn can_path_from_spawn_if(
    field: &impl Deref<Target = Field>,
    mut newly_invalid: impl FnMut(FieldLocation) -> bool,
) -> bool {
    let path = astar(
        &FieldLocation(field.source.0, field.source.1),
        |n| {
            let mut neighbors = field.get_neighbors(n);
            neighbors.retain(|neighbor| !newly_invalid(neighbor.0));
            neighbors
        },
        |n| field.estimate_distance_to_goal(n),
        |n| field.is_in_goal(n),
    );
    path.is_some()
}

pub fn is_valid_tower_location(
    field_location_query: &Query<&mut FieldLocationContents>,
    field: &impl Deref<Target = Field>,
    location: FieldLocation,
) -> bool {
    let valid_location = if let Ok(field_location_contents) =
        field_location_query.get(*field.get_entity(&location))
    {
        field_location_contents.is_empty()
    } else {
        false
    };
    let valid_location = valid_location && can_path_from_spawn_if(field, |loc| loc == location);
    valid_location
}
