use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum TowerType {
    Attack,
    Barrier,
    Ping,
}
pub fn spawn_tower(
    commands: &mut Commands,
    sprites: &Res<Sprites>,
    field: &mut ResMut<Field>,
    field_location: FieldLocation,
    tower_type: TowerType,
) {
    let mut transform = Transform::default();
    transform.translation = Vec3::new(
        field.offset.x + (field_location.0 as f32 + 0.5) * field.tile_size,
        field.offset.y + (field_location.1 as f32 + 0.5) * field.tile_size,
        1.0,
    );
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.towers.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(tower_type);
}
