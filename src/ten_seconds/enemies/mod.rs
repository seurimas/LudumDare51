use crate::prelude::*;

pub mod ai;

#[derive(Component, Debug, Clone, Copy)]
pub enum EnemyType {
    Basic,
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
        .insert(enemy_type);
}
