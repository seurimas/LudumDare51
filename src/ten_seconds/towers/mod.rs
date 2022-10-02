use crate::prelude::*;

pub mod ai;
pub mod management;
mod tree_nodes;
use self::ai::{TowerBehaviorTree, TowerImpulses};
use self::tree_nodes::*;

use super::enemies::waves::WaveEndEvent;
use super::field::FieldLocationContents;

#[derive(Debug, Clone, Copy, PartialEq, Inspectable)]
pub enum TowerClass {
    Attack,
    Silo,
    Burst,
    Triple,
    BigBomb,
    Wall,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Inspectable)]
pub struct TowerType {
    pub class: TowerClass,
    pub level: i32,
}

impl TowerType {
    pub fn get_cooldowns(&self) -> TowerCooldowns {
        self.class.get_cooldowns(self.level)
    }

    fn get_behavior_tree(&self) -> TowerBehaviorTree {
        self.class.get_behavior_tree(self.level)
    }

    pub fn get_mineral_deconstruct(&self) -> i32 {
        self.class.get_mineral_deconstruct()
    }

    pub fn get_dust_deconstruct(&self) -> i32 {
        self.class.get_dust_deconstruct()
    }

    pub fn get_tech_deconstruct(&self) -> i32 {
        self.class.get_tech_deconstruct()
    }
}

impl TowerClass {
    pub fn is_blocking(&self) -> bool {
        true
    }
    fn get_behavior_tree(&self, level: i32) -> TowerBehaviorTree {
        let tree_def = match self {
            Self::Attack => {
                BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(FireBulletNode {
                    name: "Attack".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 0,
                        damage: 1 + level,
                    },
                    fired: false,
                    speed: 512.,
                    cooldown: 0.3333,
                    lifetime: 0.25,
                })])
                .create_tree()
            }
            Self::Triple => {
                let mut steps = Vec::new();
                steps.push(BehaviorTreeDef::User(FireBulletNode {
                    name: "TripleFirst".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 1,
                        damage: 1,
                    },
                    fired: false,
                    speed: 512.,
                    cooldown: 1.,
                    lifetime: 0.25,
                }));
                for i in 0..(level + 2) {
                    steps.push(BehaviorTreeDef::User(FireBulletNode {
                        name: format!("Triple{}", i),
                        bullet_type: BulletType::Basic {
                            sprite_index: 0,
                            damage: 1,
                        },
                        fired: false,
                        speed: 512.,
                        cooldown: 0.05,
                        lifetime: 0.25,
                    }));
                }
                BehaviorTreeDef::Sequence(steps).create_tree()
            }
            Self::BigBomb => {
                BehaviorTreeDef::Sequence(vec![BehaviorTreeDef::User(FireBulletNode {
                    name: "Bomb".to_string(),
                    bullet_type: BulletType::Basic {
                        sprite_index: 1,
                        damage: 10,
                    },
                    fired: false,
                    speed: 256.,
                    cooldown: 1. / (level as f32 + 1.),
                    lifetime: 0.5,
                })])
                .create_tree()
            }
            Self::Silo | Self::Wall => {
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
    fn get_cooldowns(&self, level: i32) -> TowerCooldowns {
        let ammo_left = match self {
            Self::Attack | Self::Burst => 3 + level * level,
            Self::Silo => 5 + 5 * level,
            Self::Triple => (3 + level) * (3 + level),
            Self::BigBomb => 1,
            Self::Wall => 0,
        };
        let max_ammo = match self {
            Self::Wall => 1,
            _ => ammo_left,
        };
        TowerCooldowns {
            time_since_shot: 0.,
            time_since_hit: 0.,
            ammo_left,
            max_ammo,
        }
    }

    pub fn get_sprite_index(&self) -> usize {
        match self {
            Self::Attack => 0,
            Self::Silo => 8,
            Self::Burst => 16,
            Self::Triple => 1,
            Self::BigBomb => 2,
            Self::Wall => 3,
        }
    }

    pub fn get_mineral_cost(&self) -> i32 {
        match self {
            Self::Attack => 3,
            Self::Silo => 1,
            Self::Burst => 2,
            Self::Triple => 6,
            Self::BigBomb => 2,
            Self::Wall => 2,
        }
    }

    pub fn get_dust_cost(&self) -> i32 {
        match self {
            Self::Attack => 1,
            Self::Silo => 2,
            Self::Burst => 1,
            Self::Triple => 3,
            Self::BigBomb => 5,
            Self::Wall => 0,
        }
    }

    pub fn get_tech_cost(&self) -> i32 {
        match self {
            Self::Attack => 0,
            Self::Silo => 0,
            Self::Burst => 1,
            Self::Triple => 1,
            Self::BigBomb => 2,
            Self::Wall => 0,
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
    pub max_ammo: i32,
}

impl TowerCooldowns {
    pub fn pass_time(&mut self, delta_seconds: f32) {
        self.time_since_shot += delta_seconds;
        self.time_since_hit += delta_seconds;
    }

    pub fn has_ammo(&self) -> bool {
        self.ammo_left > 0
    }

    pub fn can_gain_ammo(&self) -> bool {
        self.ammo_left < self.max_ammo
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
            tower_behavior_tree: tower_type.class.get_behavior_tree(tower_type.level),
            tower_cooldowns: tower_type.class.get_cooldowns(tower_type.level),
        }
    }
}

pub fn tower_level_color(boosts: i32) -> Color {
    match boosts {
        0 => Color::WHITE,
        1 => Color::rgb(0.43, 1., 0.384),
        2 => Color::rgb(1., 0.384, 0.384),
        3 => Color::rgb(1., 1., 0.477),
        4 => Color::rgb(0.635, 0.592, 1.),
        _ => Color::rgb(1., 0., 0.477),
    }
}

pub fn upgrade_tower(
    entity: Entity,
    mut upgraded_tower_query: Query<(
        &mut TowerType,
        &mut TowerBehaviorTree,
        &mut TextureAtlasSprite,
    )>,
    field: &mut ResMut<Field>,
    field_location: FieldLocation,
    mut field_location_query: Query<&mut FieldLocationContents>,
) {
    if let Ok((mut tower_type, mut tower_behavior_tree, mut sprite)) =
        upgraded_tower_query.get_mut(entity)
    {
        tower_type.level += 1;
        *tower_behavior_tree = tower_type.get_behavior_tree();
        sprite.color = tower_level_color(tower_type.level);
        if let Ok(mut field_location_contents) =
            field_location_query.get_mut(*field.get_entity(&field_location))
        {
            *field_location_contents = FieldLocationContents::Tower(entity, *tower_type);
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
                sprite: TextureAtlasSprite {
                    color: tower_level_color(tower_type.level),
                    ..TextureAtlasSprite::new(tower_type.class.get_sprite_index())
                },
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
            cooldown.time_since_shot = rand::random();
        }
    }
}
