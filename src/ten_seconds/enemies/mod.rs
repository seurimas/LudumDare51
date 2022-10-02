use crate::prelude::*;

use self::{
    ai::{EnemyBehaviorTree, EnemyImpulses},
    tree_nodes::PathfindNode,
};

pub mod ai;
pub mod damaged;
pub mod tree_nodes;
pub mod waves;

#[derive(PartialEq, Component, Debug, Clone, Copy, Inspectable)]
pub enum EnemyType {
    Basic,
    Seeker,
    Fast,
    Buster,
}

impl EnemyType {
    fn get_behavior_tree(&self) -> EnemyBehaviorTree {
        let tree_def = match self {
            Self::Basic | Self::Fast | Self::Seeker | Self::Buster => {
                BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(PathfindNode {
                    name: "BasicPath".to_string(),
                })])
                .create_tree()
            }
        };
        EnemyBehaviorTree(tree_def)
    }

    fn get_health(&self) -> Health {
        let health = match self {
            Self::Basic => 4,
            Self::Seeker => 6,
            Self::Fast => 4,
            Self::Buster => 4,
        };
        Health {
            max_health: health,
            health,
            dead: false,
        }
    }

    pub fn get_sprite(&self) -> usize {
        match self {
            Self::Basic => 0,
            Self::Seeker => 1,
            Self::Fast => 2,
            Self::Buster => 3,
        }
    }

    pub fn get_speed(&self) -> f32 {
        match self {
            Self::Basic => 128.0,
            Self::Seeker => 128.0,
            Self::Fast => 196.0,
            Self::Buster => 96.0,
        }
    }

    pub fn get_death_tile_cost(&self) -> i32 {
        match self {
            Self::Basic | Self::Fast => 0,
            Self::Seeker => 50,
            Self::Buster => 100,
        }
    }

    pub fn get_mineral_loot(&self) -> i32 {
        match self {
            Self::Basic | Self::Fast => 1,
            Self::Seeker => 2,
            Self::Buster => 3,
        }
    }

    pub fn get_dust_loot(&self) -> i32 {
        1
    }

    pub fn get_tech_loot(&self) -> i32 {
        match self {
            Self::Basic => 0,
            Self::Seeker | Self::Fast | Self::Buster => 1,
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy_type: EnemyType,
    enemy_behavior_tree: EnemyBehaviorTree,
    enemy_impulses: EnemyImpulses,
    health: Health,
}

impl EnemyBundle {
    pub fn new(enemy_type: EnemyType) -> Self {
        EnemyBundle {
            enemy_type,
            enemy_impulses: Default::default(),
            enemy_behavior_tree: enemy_type.get_behavior_tree(),
            health: enemy_type.get_health(),
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    sprites: &Res<Sprites>,
    transform: Transform,
    enemy_type: EnemyType,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.enemies.clone(),
            sprite: TextureAtlasSprite::new(enemy_type.get_sprite()),
            ..Default::default()
        })
        .insert_bundle(EnemyBundle::new(enemy_type))
        .insert(GameOverCleanup);
}
