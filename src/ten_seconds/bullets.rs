use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BulletType {
    Basic {
        hits_enemies: bool,
        hits_towers: bool,
        sprite_index: usize,
    },
}

impl BulletType {
    pub fn get_sprite_index(&self) -> usize {
        match self {
            Self::Basic { sprite_index, .. } => *sprite_index,
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
        match self.bullet_type {
            BulletType::Basic { hits_enemies, .. } => hits_enemies,
        }
    }
    pub fn hits_towers(&self) -> bool {
        match self.bullet_type {
            BulletType::Basic { hits_towers, .. } => hits_towers,
        }
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
        });
}

pub fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    field: Res<Field>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();
    for (entity, mut bullet, mut transform) in bullets.iter_mut() {
        let distance = bullet.velocity * delta_seconds;
        transform.translation += Vec3::new(distance.x, distance.y, 0.);

        bullet.lifetime -= delta_seconds;
        if bullet.lifetime <= 0. {
            commands.entity(entity).despawn();
            continue;
        }

        let location = Vec2::new(transform.translation.x, transform.translation.y);
        if let Some(new_tile) = get_tile_from_location(location, &field) {
            if bullet.hits_enemies() {
                for (_enemy, center_point) in
                    field.get_enemies_in_or_near_tile(&FieldLocation(new_tile.0, new_tile.1))
                {
                    if location.distance_squared(center_point) < 64. {
                        println!("HIT!");
                    }
                }
            }
        }
    }
}
