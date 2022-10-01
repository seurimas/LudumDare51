use crate::prelude::*;

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
            .insert(tower_type)
            .id();
        *field_location_contents = FieldLocationContents::Tower(tower_entity, tower_type);
    }
}
