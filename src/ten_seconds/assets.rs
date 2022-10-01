use bevy::asset::LoadState;

use crate::prelude::*;

pub struct Sprites {
    pub field: Handle<TextureAtlas>,
    pub field_sprite: Handle<Image>,
    pub enemies: Handle<TextureAtlas>,
    pub enemies_sprite: Handle<Image>,
    pub towers: Handle<TextureAtlas>,
    pub towers_sprite: Handle<Image>,
}

pub fn loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprites: Option<Res<Sprites>>,
    mut app_state: ResMut<State<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Some(sprites) = sprites {
        let load_state = asset_server
            .get_group_load_state(vec![sprites.field_sprite.id, sprites.enemies_sprite.id]);
        if load_state == LoadState::Loaded {
            app_state.set(AppState::InGame).unwrap();
        } else {
            println!("load_state: {:?}", load_state);
        }
    } else {
        let field_sprite = asset_server.load("field.png");
        let field_atlas =
            TextureAtlas::from_grid(field_sprite.clone(), Vec2::new(32.0, 32.0), 8, 8);
        let field = texture_atlases.add(field_atlas);

        let enemies_sprite = asset_server.load("enemies.png");
        let enemies_atlas =
            TextureAtlas::from_grid(enemies_sprite.clone(), Vec2::new(32.0, 32.0), 8, 8);
        let enemies = texture_atlases.add(enemies_atlas);

        let towers_sprite = asset_server.load("towers.png");
        let towers_atlas =
            TextureAtlas::from_grid(towers_sprite.clone(), Vec2::new(32.0, 32.0), 8, 8);
        let towers = texture_atlases.add(towers_atlas);

        commands.insert_resource(Sprites {
            field,
            field_sprite,
            enemies,
            enemies_sprite,
            towers,
            towers_sprite,
        });
    }
}
