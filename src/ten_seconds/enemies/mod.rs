use crate::prelude::*;

use self::{
    ai::{EnemyBehaviorTree, EnemyImpulses},
    tree_nodes::{AttackNode, EnemyNode, PathfindNode},
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
    Gnat,
    Buster,
    Thief,
}

impl EnemyType {
    fn get_behavior_tree(&self) -> EnemyBehaviorTree {
        let tree_def = match self {
            Self::Basic | Self::Fast | Self::Seeker | Self::Buster | Self::Gnat => {
                BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(PathfindNode {
                    name: "BasicPath".to_string(),
                })])
                .create_tree()
            }
            Self::Thief => BehaviorTreeDef::Sequence(vec![
                BehaviorTreeDef::User(EnemyNode::Pathfind(PathfindNode {
                    name: "BasicPath".to_string(),
                })),
                BehaviorTreeDef::User(EnemyNode::Attack(AttackNode {
                    name: "BasicPath".to_string(),
                    idx: 0,
                })),
            ])
            .create_tree(),
        };
        EnemyBehaviorTree(tree_def)
    }

    fn get_health(&self, boosts: i32) -> Health {
        let health = match self {
            Self::Basic => 3 + boosts * 2,
            Self::Seeker => 4 + boosts * 3,
            Self::Gnat => 2 + boosts * 1,
            Self::Fast => 5 + boosts * 2,
            Self::Buster => 8 + boosts * 5,
            Self::Thief => 5 + boosts * 2,
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
            Self::Thief => 4,
            Self::Gnat => 5,
        }
    }

    pub fn get_speed(&self) -> f32 {
        match self {
            Self::Basic => 128.0,
            Self::Seeker => 128.0,
            Self::Fast | Self::Thief | Self::Gnat => 196.0,
            Self::Buster => 96.0,
        }
    }

    pub fn get_death_tile_cost(&self) -> i32 {
        match self {
            Self::Basic | Self::Fast | Self::Thief => 0,
            Self::Gnat => 25,
            Self::Seeker => 50,
            Self::Buster => 100,
        }
    }

    pub fn get_mineral_loot(&self) -> i32 {
        match self {
            Self::Basic | Self::Fast | Self::Gnat => 1,
            Self::Seeker => 2,
            Self::Buster | Self::Thief => 3,
        }
    }

    pub fn get_dust_loot(&self) -> i32 {
        1
    }

    pub fn get_tech_loot(&self) -> i32 {
        match self {
            Self::Basic | Self::Gnat => 0,
            Self::Seeker | Self::Fast | Self::Buster | Self::Thief => 1,
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
    pub fn new(enemy_type: EnemyType, boosts: i32) -> Self {
        EnemyBundle {
            enemy_type,
            enemy_impulses: Default::default(),
            enemy_behavior_tree: enemy_type.get_behavior_tree(),
            health: enemy_type.get_health(boosts),
        }
    }
}

fn boost_color(boosts: i32) -> Color {
    match boosts {
        0 => Color::WHITE,
        1 => Color::rgb(0.43, 1., 0.384),
        2 => Color::rgb(0.43, 1., 0.384),
        3 => Color::rgb(1., 0.384, 0.384),
        4 => Color::rgb(1., 1., 0.477),
        5 => Color::rgb(0.635, 0.592, 1.),
        _ => Color::rgb(1., 0., 0.477),
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    sprites: &Res<Sprites>,
    transform: Transform,
    enemy_type: EnemyType,
    boosts: i32,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.enemies.clone(),
            sprite: TextureAtlasSprite {
                color: boost_color(boosts),
                ..TextureAtlasSprite::new(enemy_type.get_sprite())
            },
            ..Default::default()
        })
        .insert_bundle(EnemyBundle::new(enemy_type, boosts))
        .insert(GameOverCleanup);
}
