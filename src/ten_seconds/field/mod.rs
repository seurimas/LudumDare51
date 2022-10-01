use crate::prelude::*;

use self::highlighting::FieldLocationHighlight;

use super::{assets::Sprites, towers::TowerType};

mod constants;
pub mod highlighting;
use self::constants::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Component, Debug, Inspectable)]
pub struct FieldLocation(pub i32, pub i32);

#[derive(Component, Debug, Clone, Copy)]
pub enum FieldLocationContents {
    None,
    BlockingEnemy(Entity, EnemyType),
    Tower(Entity, TowerType),
    Spawner,
    Goal,
}

impl Inspectable for FieldLocationContents {
    type Attributes = ();
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        options: Self::Attributes,
        context: &mut bevy_inspector_egui::Context,
    ) -> bool {
        match self {
            FieldLocationContents::None => ui.label("None"),
            FieldLocationContents::BlockingEnemy(_, _) => ui.label("BlockingEnemy"),
            FieldLocationContents::Tower(_, _) => ui.label("Tower"),
            FieldLocationContents::Spawner => ui.label("Spawner"),
            FieldLocationContents::Goal => ui.label("Goal"),
        };
        false
    }
}

impl FieldLocationContents {
    pub fn is_empty(&self) -> bool {
        match self {
            FieldLocationContents::None => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Pathability {
    Pathable,
    Unpathable,
}

pub struct Field {
    pub width: i32,
    pub height: i32,
    pub tile_size: f32,
    pub offset: Vec2,
    pub source: (i32, i32),
    pub target: (i32, i32),
    pub field_locations: Vec<(Entity, FieldLocationContents, Pathability)>,
    pub enemies_in_tiles: Vec<Vec<(Entity, Vec2)>>,
}

impl Field {
    pub fn new(
        width: i32,
        height: i32,
        tile_size: f32,
        offset: Vec2,
        source: (i32, i32),
        target: (i32, i32),
        field_locations: Vec<(Entity, FieldLocationContents, Pathability)>,
    ) -> Self {
        let mut enemies_in_tiles = Vec::new();
        enemies_in_tiles.resize(field_locations.len(), Vec::new());
        Field {
            width,
            height,
            tile_size,
            offset,
            source,
            target,
            field_locations,
            enemies_in_tiles,
        }
    }

    pub fn clear_enemies_in_tiles(&mut self) {
        let mut enemies_in_tiles = Vec::new();
        enemies_in_tiles.resize(self.field_locations.len(), Vec::new());
        self.enemies_in_tiles = enemies_in_tiles;
    }

    pub fn add_enemy_in_tile(
        &mut self,
        location: &FieldLocation,
        enemy: Entity,
        enemy_location: Vec2,
    ) {
        self.enemies_in_tiles[(location.0 + location.1 * self.width) as usize]
            .push((enemy, enemy_location));
    }

    pub fn get_enemies_in_tile(&self, location: &FieldLocation) -> &Vec<(Entity, Vec2)> {
        &self.enemies_in_tiles[(location.0 + location.1 * self.width) as usize]
    }

    pub fn get_enemies_in_or_near_tile(&self, location: &FieldLocation) -> Vec<(Entity, Vec2)> {
        let mut in_or_near = Vec::new();
        in_or_near.extend(self.get_enemies_in_tile(location));
        for neighbor in self.get_neighbors(location) {
            in_or_near.extend(self.get_enemies_in_tile(&neighbor.0));
        }
        in_or_near
    }

    pub fn get_entity_contents_pathability(
        &self,
        location: &FieldLocation,
    ) -> &(Entity, FieldLocationContents, Pathability) {
        &self.field_locations[(location.0 + location.1 * self.width) as usize]
    }

    fn get_entity_contents_pathability_mut(
        &mut self,
        location: &FieldLocation,
    ) -> &mut (Entity, FieldLocationContents, Pathability) {
        &mut self.field_locations[(location.0 + location.1 * self.width) as usize]
    }

    pub fn get_entity(&self, location: &FieldLocation) -> &Entity {
        &self.get_entity_contents_pathability(location).0
    }

    pub fn get_contents(&self, location: &FieldLocation) -> &FieldLocationContents {
        &self.get_entity_contents_pathability(location).1
    }

    pub fn update_contents(&mut self, location: &FieldLocation, contents: &FieldLocationContents) {
        let entity_contents_pathability = self.get_entity_contents_pathability_mut(location);
        entity_contents_pathability.1 = contents.clone();
        match contents {
            FieldLocationContents::Tower(_, tower_type) => {
                if tower_type.is_blocking() {
                    entity_contents_pathability.2 = Pathability::Unpathable;
                } else {
                    entity_contents_pathability.2 = Pathability::Pathable;
                }
            }
            _ => {
                entity_contents_pathability.2 = Pathability::Pathable;
            }
        }
    }

    pub fn is_pathable(&self, location: &FieldLocation) -> bool {
        self.get_entity_contents_pathability(location).2 == Pathability::Pathable
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
        if location.1 < self.height - 1 {
            neighbors.push((FieldLocation(location.0, location.1 + 1), 1));
        }
        neighbors
    }

    pub fn get_pathable_neighbors(&self, location: &FieldLocation) -> Vec<(FieldLocation, i32)> {
        let mut neighbors = self.get_neighbors(location);
        neighbors.retain(|(neighbor, _cost)| self.is_pathable(neighbor));
        neighbors
    }

    pub fn estimate_distance_to_goal(&self, location: &FieldLocation) -> i32 {
        (location.0 - self.target.0).abs() + (location.1 - self.target.1).abs()
    }

    pub fn is_in_goal(&self, location: &FieldLocation) -> bool {
        self.estimate_distance_to_goal(location) == 0
    }

    pub fn get_spawn_transform(&self) -> Transform {
        let mut transform = Transform::default();
        transform.translation = Vec3::new(
            self.offset.x + (self.tile_size * (self.source.0 as f32 + 0.5)),
            self.offset.y + (self.tile_size * (self.source.1 as f32 + 0.5)),
            0.,
        );
        transform
    }
}

pub fn spawn_field(mut commands: Commands, sprites: Res<Sprites>) {
    let mut field_locations = Vec::new();
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            let mut transform = Transform::default();
            transform.translation = Vec3::new(
                (x as f32 * TILE_SIZE) + OFFSET.0 + TILE_SIZE / 2.,
                (y as f32 * TILE_SIZE) + OFFSET.1 + TILE_SIZE / 2.,
                0.0,
            );
            let contents = if x == SOURCE.0 && y == SOURCE.1 {
                FieldLocationContents::Spawner
            } else if x == TARGET.0 && y == TARGET.1 {
                FieldLocationContents::Goal
            } else {
                FieldLocationContents::None
            };
            let location_entity = commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: sprites.field.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(FieldLocation(x, y))
                .insert(FieldLocationHighlight::None)
                .insert(contents)
                .id();
            field_locations.push((location_entity, contents, Pathability::Pathable));
        }
    }
    let field = Field::new(
        FIELD_WIDTH,
        FIELD_HEIGHT,
        TILE_SIZE,
        OFFSET.into(),
        SOURCE,
        TARGET,
        field_locations,
    );
    commands.insert_resource(field);
}

pub fn update_contents(
    mut field: ResMut<Field>,
    query: Query<(&FieldLocation, &FieldLocationContents), Changed<FieldLocationContents>>,
) {
    for (location, new_contents) in query.iter() {
        field.update_contents(location, new_contents);
    }
}

pub fn update_enemies_in_tiles(
    mut field: ResMut<Field>,
    query: Query<(Entity, &EnemyType, &Transform), Changed<Transform>>,
) {
    field.clear_enemies_in_tiles();
    for (entity, _enemy, transform) in query.iter() {
        let location = Vec2::new(transform.translation.x, transform.translation.y);
        if let Some(tile) = get_tile_from_location(location, &field) {
            field.add_enemy_in_tile(&FieldLocation(tile.0, tile.1), entity, location);
        }
    }
}
