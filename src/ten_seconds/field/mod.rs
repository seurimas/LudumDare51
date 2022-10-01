use crate::prelude::*;

use self::highlighting::FieldLocationHighlight;

use super::assets::Sprites;

pub mod highlighting;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Component)]
pub struct FieldLocation(i32, i32);

const FIELD_WIDTH: i32 = 31;
const FIELD_HEIGHT: i32 = 23;
const TILE_SIZE: f32 = 32.0;
const OFFSET: (f32, f32) = (16.0, 16.0);
const SOURCE: (i32, i32) = (0, 15);
const TARGET: (i32, i32) = (30, 15);

pub struct Field {
    pub width: i32,
    pub height: i32,
    pub tile_size: f32,
    pub offset: Vec2,
    pub source: (i32, i32),
    pub target: (i32, i32),
}

impl Field {
    pub fn new(
        width: i32,
        height: i32,
        tile_size: f32,
        offset: Vec2,
        source: (i32, i32),
        target: (i32, i32),
    ) -> Self {
        Field {
            width,
            height,
            tile_size,
            offset,
            source,
            target,
        }
    }

    pub fn get_neighbors(&self, location: &FieldLocation) -> Vec<(FieldLocation, i32)> {
        let mut neighbors = Vec::new();
        if location.0 > 0 {
            neighbors.push((FieldLocation(location.0 - 1, location.1), 1));
        }
        if location.1 > 0 {
            neighbors.push((FieldLocation(location.0, location.1 - 1), 1));
        }
        if location.0 < self.width - 1 {
            neighbors.push((FieldLocation(location.0 + 1, location.1), 1));
        }
        if location.0 < self.height - 1 {
            neighbors.push((FieldLocation(location.0, location.1 + 1), 1));
        }
        neighbors
    }

    pub fn estimate_distance_to_goal(&self, location: &FieldLocation) -> i32 {
        (location.0 - self.target.0).abs() + (location.1 - self.target.1).abs()
    }
}

pub fn add_camera(mut commands: Commands) {
    let mut transform = Transform::default();
    transform.translation = Vec3::new(
        FIELD_WIDTH as f32 * TILE_SIZE / 2.0 + OFFSET.0,
        FIELD_HEIGHT as f32 * TILE_SIZE / 2.0 + OFFSET.1,
        1.0,
    );
    commands.spawn_bundle(Camera2dBundle {
        transform,
        ..Default::default()
    });
}

pub fn spawn_field(mut commands: Commands, sprites: Res<Sprites>) {
    let field = Field::new(
        FIELD_WIDTH,
        FIELD_HEIGHT,
        TILE_SIZE,
        OFFSET.into(),
        SOURCE,
        TARGET,
    );
    commands.insert_resource(field);
    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            let mut transform = Transform::default();
            transform.translation = Vec3::new(
                (x as f32 * TILE_SIZE) + OFFSET.0,
                (y as f32 * TILE_SIZE) + OFFSET.1,
                0.0,
            );
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprites.field.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(FieldLocation(x, y))
                .insert(FieldLocationHighlight::None);
        }
    }
}
