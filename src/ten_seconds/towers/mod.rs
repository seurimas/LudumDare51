use crate::prelude::*;

pub mod ai;
mod tree_nodes;
use self::ai::{TowerBehaviorTree, TowerImpulses};
use self::tree_nodes::*;

use super::field::FieldLocationContents;

#[derive(Component, Debug, Clone, Copy, Inspectable)]
pub enum TowerType {
    Attack,
    Barrier,
    Ping,
}

impl TowerType {
    pub fn is_blocking(&self) -> bool {
        match self {
            TowerType::Attack | TowerType::Ping => true,
            _ => false,
        }
    }
    fn get_behavior_tree(&self) -> TowerBehaviorTree {
        let tree_def = match self {
            _ => BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(FireBulletNode {
                name: "Attack".to_string(),
                bullet_type: BulletType::Basic {
                    hits_enemies: true,
                    hits_towers: false,
                    sprite_index: 0,
                },
                speed: 512.,
                cooldown: 2.0,
                lifetime: 100.0,
            })]),
        };
        TowerBehaviorTree(tree_def.create_tree())
    }
}

#[derive(Debug, Copy, Clone, Component, Default)]
pub struct TowerCooldowns {
    pub time_since_shot: f32,
    pub time_since_hit: f32,
}

impl TowerCooldowns {
    pub fn pass_time(&mut self, delta_seconds: f32) {
        self.time_since_shot += delta_seconds;
        self.time_since_hit += delta_seconds;
    }
}

#[derive(Bundle)]
struct TowerBundle {
    tower_type: TowerType,
    tower_behavior_tree: TowerBehaviorTree,
    tower_impulses: TowerImpulses,
    tower_cooldowns: TowerCooldowns,
}

impl TowerBundle {
    fn new(tower_type: TowerType) -> Self {
        TowerBundle {
            tower_type,
            tower_impulses: Default::default(),
            tower_behavior_tree: tower_type.get_behavior_tree(),
            tower_cooldowns: Default::default(),
        }
    }
}

pub fn spawn_tower(
    commands: &mut Commands,
    sprites: &Res<Sprites>,
    field: &mut ResMut<Field>,
    field_location: FieldLocation,
    tower_type: TowerType,
    mut field_location_query: Query<&mut FieldLocationContents>,
) {
    let mut transform = Transform::default();
    transform.translation = Vec3::new(
        field.offset.x + (field_location.0 as f32 + 0.5) * field.tile_size,
        field.offset.y + (field_location.1 as f32 + 0.5) * field.tile_size,
        1.0,
    );
    if let Ok(mut field_location_contents) =
        field_location_query.get_mut(*field.get_entity(&field_location))
    {
        let tower_entity = commands
            .spawn_bundle(SpriteSheetBundle {
                transform,
                texture_atlas: sprites.towers.clone(),
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            })
            .insert_bundle(TowerBundle::new(tower_type))
            .id();
        *field_location_contents = FieldLocationContents::Tower(tower_entity, tower_type);
    }
}
