use bevy::asset::LoadState;

use crate::prelude::*;

pub struct Sprites {
    pub field: Handle<TextureAtlas>,
    pub field_sprite: Handle<Image>,
}

pub fn loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprites: Option<Res<Sprites>>,
    mut app_state: ResMut<State<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Some(sprites) = sprites {
        let load_state = asset_server.get_group_load_state(vec![sprites.field_sprite.id]);
        if load_state == LoadState::Loaded {
            app_state.set(AppState::InGame).unwrap();
        } else {
            println!("load_state: {:?}", load_state);
        }
    } else {
        let field_sprite = asset_server.load("field.png");
        let texture_atlas =
            TextureAtlas::from_grid(field_sprite.clone(), Vec2::new(32.0, 32.0), 8, 8);
        let field = texture_atlases.add(texture_atlas);
        commands.insert_resource(Sprites {
            field,
            field_sprite,
        });
    }
}
