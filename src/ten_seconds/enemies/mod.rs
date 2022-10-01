use crate::prelude::*;

use self::{
    ai::{EnemyBehaviorTree, EnemyImpulses},
    tree_nodes::PathfindNode,
};

pub mod ai;
pub mod tree_nodes;
pub mod waves;

#[derive(Component, Debug, Clone, Copy, Inspectable)]
pub enum EnemyType {
    Basic,
}

impl EnemyType {
    fn get_behavior_tree(&self) -> EnemyBehaviorTree {
        let tree_def = match self {
            Self::Basic => BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(PathfindNode {
                name: "BasicPath".to_string(),
            })]),
        };
        EnemyBehaviorTree(tree_def.create_tree())
    }

    fn get_speed(&self) -> f32 {
        match self {
            Self::Basic => 256.0,
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy_type: EnemyType,
    enemy_behavior_tree: EnemyBehaviorTree,
    enemy_impulses: EnemyImpulses,
}

impl EnemyBundle {
    pub fn new(enemy_type: EnemyType) -> Self {
        EnemyBundle {
            enemy_type,
            enemy_impulses: Default::default(),
            enemy_behavior_tree: enemy_type.get_behavior_tree(),
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
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert_bundle(EnemyBundle::new(enemy_type));
}
