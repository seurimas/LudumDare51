use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BulletType {
    Basic { sprite_index: usize, damage: i32 },
}

impl BulletType {
    pub fn get_sprite_index(&self) -> usize {
        match self {
            Self::Basic { sprite_index, .. } => *sprite_index,
        }
    }
    pub fn damage(&self) -> i32 {
        match self {
            Self::Basic { damage, .. } => *damage,
        }
    }
}

impl Inspectable for BulletType {
    type Attributes = ();

    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _options: Self::Attributes,
        _context: &mut bevy_inspector_egui::Context,
    ) -> bool {
        ui.label("Basic");
        false
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct Bullet {
    pub bullet_type: BulletType,
    pub velocity: Vec2,
    pub lifetime: f32,
}

impl Bullet {
    pub fn hits_enemies(&self) -> bool {
        true
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    sprites: &Res<Sprites>,
    transform: Transform,
    bullet_type: BulletType,
    velocity: Vec2,
    lifetime: f32,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.bullets.clone(),
            sprite: TextureAtlasSprite::new(bullet_type.get_sprite_index()),
            ..Default::default()
        })
        .insert(Bullet {
            bullet_type,
            velocity,
            lifetime,
        })
        .insert(InGameOnly);
}

#[derive(Debug)]
pub struct BulletHitEvent {
    pub bullet_entity: Entity,
    pub target_entity: Entity,
    pub bullet_type: BulletType,
}

pub fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    field: Res<Field>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform)>,
    mut ev_bullet_hit: EventWriter<BulletHitEvent>,
) {
    let delta_seconds = time.delta_seconds();
    for (bullet_entity, mut bullet, mut transform) in bullets.iter_mut() {
        let distance = bullet.velocity * delta_seconds;
        transform.translation += Vec3::new(distance.x, distance.y, 0.);

        bullet.lifetime -= delta_seconds;
        if bullet.lifetime <= 0. {
            commands.entity(bullet_entity).despawn();
            continue;
        }

        let location = Vec2::new(transform.translation.x, transform.translation.y);
        if let Some(new_tile) = get_tile_from_location(location, &field) {
            if bullet.hits_enemies() {
                for (target_entity, center_point) in
                    field.get_enemies_in_or_near_tile(&FieldLocation(new_tile.0, new_tile.1))
                {
                    if location.distance_squared(center_point) < 64. {
                        ev_bullet_hit.send(BulletHitEvent {
                            bullet_entity,
                            target_entity,
                            bullet_type: bullet.bullet_type,
                        });
                        break;
                    }
                }
            }
        }
    }
}
