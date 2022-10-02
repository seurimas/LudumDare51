use crate::prelude::*;

pub mod ai;
pub mod management;
mod tree_nodes;
use self::ai::{TowerBehaviorTree, TowerImpulses};
use self::tree_nodes::*;

use super::enemies::waves::WaveEndEvent;
use super::field::FieldLocationContents;

#[derive(Component, Debug, Clone, Copy, Inspectable)]
pub enum TowerType {
    Attack,
    Silo,
    Burst,
    Triple,
    BigBomb,
}

impl TowerType {
    pub fn is_blocking(&self) -> bool {
        true
    }
    fn get_behavior_tree(&self) -> TowerBehaviorTree {
        let tree_def = match self {
            Self::Attack => {
                BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(FireBulletNode {
                    name: "Attack".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 0,
                        damage: 1,
                    },
                    fired: false,
                    speed: 512.,
                    cooldown: 0.3333,
                    lifetime: 0.25,
                })])
                .create_tree()
            }
            Self::Triple => BehaviorTreeDef::Sequence(vec![
                BehaviorTreeDef::User(FireBulletNode {
                    name: "TripleOne".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 1,
                        damage: 1,
                    },
                    fired: false,
                    speed: 512.,
                    cooldown: 1.,
                    lifetime: 0.25,
                }),
                BehaviorTreeDef::User(FireBulletNode {
                    name: "TripleTwo".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 0,
                        damage: 1,
                    },
                    fired: false,
                    speed: 512.,
                    cooldown: 0.05,
                    lifetime: 0.25,
                }),
                BehaviorTreeDef::User(FireBulletNode {
                    name: "TripleTwo".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 0,
                        damage: 1,
                    },
                    fired: false,
                    speed: 512.,
                    cooldown: 0.05,
                    lifetime: 0.25,
                }),
            ])
            .create_tree(),
            Self::BigBomb => {
                BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(FireBulletNode {
                    name: "Bomb".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 1,
                        damage: 10,
                    },
                    fired: false,
                    speed: 256.,
                    cooldown: 1.,
                    lifetime: 0.5,
                })])
                .create_tree()
            }
            Self::Silo => {
                BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(RotatingAssistNode {
                    name: "Reload".to_string(),
                    idx: 0,
                })])
                .create_tree()
            }
            _ => todo!("Other towers"),
        };
        TowerBehaviorTree(tree_def)
    }
    fn get_cooldowns(&self) -> TowerCooldowns {
        let ammo_left = match self {
            Self::Attack | Self::Burst => 5,
            Self::Silo => 10,
            Self::Triple => 9,
            Self::BigBomb => 1,
        };
        TowerCooldowns {
            time_since_shot: 0.,
            time_since_hit: 0.,
            ammo_left,
        }
    }

    pub fn get_sprite_index(&self) -> usize {
        match self {
            Self::Attack => 0,
            Self::Silo => 8,
            Self::Burst => 16,
            Self::Triple => 1,
            Self::BigBomb => 2,
        }
    }

    pub fn get_mineral_cost(&self) -> i32 {
        match self {
            Self::Attack => 2,
            Self::Silo => 1,
            Self::Burst => 2,
            Self::Triple => 4,
            Self::BigBomb => 2,
        }
    }

    pub fn get_dust_cost(&self) -> i32 {
        match self {
            Self::Attack => 1,
            Self::Silo => 1,
            Self::Burst => 1,
            Self::Triple => 3,
            Self::BigBomb => 3,
        }
    }

    pub fn get_tech_cost(&self) -> i32 {
        match self {
            Self::Attack => 0,
            Self::Silo => 0,
            Self::Burst => 1,
            Self::Triple => 2,
            Self::BigBomb => 3,
        }
    }

    pub fn get_mineral_deconstruct(&self) -> i32 {
        i32::max(0, self.get_mineral_cost() - 1)
    }

    pub fn get_dust_deconstruct(&self) -> i32 {
        self.get_mineral_cost()
    }

    pub fn get_tech_deconstruct(&self) -> i32 {
        i32::max(0, self.get_tech_cost() - 2)
    }
}

#[derive(Debug, Copy, Clone, Component)]
pub struct TowerCooldowns {
    pub time_since_shot: f32,
    pub time_since_hit: f32,
    pub ammo_left: i32,
}

impl TowerCooldowns {
    pub fn pass_time(&mut self, delta_seconds: f32) {
        self.time_since_shot += delta_seconds;
        self.time_since_hit += delta_seconds;
    }

    pub fn has_ammo(&self) -> bool {
        self.ammo_left > 0
    }

    pub fn use_ammo(&mut self) -> bool {
        if self.ammo_left > 0 {
            self.ammo_left -= 1;
            true
        } else {
            false
        }
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
            tower_cooldowns: tower_type.get_cooldowns(),
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
                sprite: TextureAtlasSprite::new(tower_type.get_sprite_index()),
                ..Default::default()
            })
            .insert_bundle(TowerBundle::new(tower_type))
            .insert(InGameOnly)
            .id();
        *field_location_contents = FieldLocationContents::Tower(tower_entity, tower_type);
    }
}

pub fn refresh_towers(
    mut ev_wave_end: EventReader<WaveEndEvent>,
    mut cooldowns: Query<(&TowerType, &mut TowerCooldowns)>,
) {
    for _wave_end in ev_wave_end.iter() {
        for (tower_type, mut cooldown) in cooldowns.iter_mut() {
            *cooldown = tower_type.get_cooldowns();
        }
    }
}
