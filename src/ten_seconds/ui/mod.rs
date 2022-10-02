use crate::prelude::*;

pub mod systems;

#[derive(Component)]
pub struct HealthCrystal(pub usize);

pub fn init_ui(mut commands: Commands, windows: Res<Windows>, sprites: Res<Sprites>) {
    let (width, height) = windows
        .get_primary()
        .map(|window| (window.width(), window.height()))
        .unwrap_or((960., 720.));
    // BACKDROP
    commands
        .spawn_bundle(ImageBundle {
            image: UiImage(sprites.gui.clone()),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(height - 80.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InGameOnly)
        .insert(Name::new("GUI_Backdrop"));

    // HEALTH
    let mut crystals = Vec::new();
    for crystal_index in 0..10 {
        let mut crystal = commands.spawn_bundle(ImageBundle {
            image: UiImage(sprites.crystal_full.clone()),
            ..Default::default()
        });
        crystal.insert(HealthCrystal(crystal_index));
        crystals.push(crystal.id());
    }
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: UiRect::new(
                    Val::Px(0.),
                    Val::Px(540.),
                    Val::Px(0.),
                    Val::Px(height - 80.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("CrystalsBox"))
        .insert(InGameOnly)
        .push_children(&crystals[..]);

    // COUNTDOWN
    let countdown = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "10",
                TextStyle {
                    color: Color::rgb(0.43, 1., 0.38),
                    font_size: 80.,
                    font: sprites.countdown_font.clone(),
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Countdown"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: UiRect::new(
                    Val::Px(420.),
                    Val::Px(420.),
                    Val::Px(0.),
                    Val::Px(height - 80.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("CountdownBox"))
        .insert(InGameOnly)
        .add_child(countdown);

    // MINERALS
    let minerals = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    color: Color::rgb(0.43, 1., 0.38),
                    font_size: 32.,
                    font: sprites.countdown_font.clone(),
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Minerals"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: UiRect::new(
                    Val::Px(540.),
                    Val::Px(360.),
                    Val::Px(0.),
                    Val::Px(height - 76.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("MineralsBox"))
        .insert(InGameOnly)
        .add_child(minerals);

    // DUST
    let minerals = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    color: Color::rgb(0.43, 1., 0.38),
                    font_size: 32.,
                    font: sprites.countdown_font.clone(),
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Dust"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: UiRect::new(
                    Val::Px(600.),
                    Val::Px(300.),
                    Val::Px(0.),
                    Val::Px(height - 76.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("DustBox"))
        .insert(InGameOnly)
        .add_child(minerals);

    // TECH
    let minerals = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    color: Color::rgb(0.43, 1., 0.38),
                    font_size: 32.,
                    font: sprites.countdown_font.clone(),
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Tech"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: UiRect::new(
                    Val::Px(660.),
                    Val::Px(240.),
                    Val::Px(0.),
                    Val::Px(height - 76.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("TechBox"))
        .insert(InGameOnly)
        .add_child(minerals);
    // INFO
    let mut transform = Transform::default();
    transform.translation = Vec3::new(width - 240. + 16., height - 16., 0.);
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.towers.clone(),
            sprite: TextureAtlasSprite {
                color: Color::rgb(0.43, 1., 0.38),
                ..TextureAtlasSprite::new(TowerClass::Attack.get_sprite_index())
            },
            ..Default::default()
        })
        .insert(Name::new("AttackHelper"));
    let mut transform = Transform::default();
    transform.translation = Vec3::new(width - 240. + 32. + 16., height - 16., 0.);
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.towers.clone(),
            sprite: TextureAtlasSprite::new(TowerClass::Silo.get_sprite_index()),
            ..Default::default()
        })
        .insert(Name::new("SiloHelper"));
    let mut transform = Transform::default();
    transform.translation = Vec3::new(width - 240. + 64. + 16., height - 16., 0.);
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.towers.clone(),
            sprite: TextureAtlasSprite::new(TowerClass::Triple.get_sprite_index()),
            ..Default::default()
        })
        .insert(Name::new("TripleHelper"));
    let mut transform = Transform::default();
    transform.translation = Vec3::new(width - 240. + 96. + 16., height - 16., 0.);
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.towers.clone(),
            sprite: TextureAtlasSprite::new(TowerClass::BigBomb.get_sprite_index()),
            ..Default::default()
        })
        .insert(Name::new("BigBombHelper"));
    let mut transform = Transform::default();
    transform.translation = Vec3::new(width - 240. + 128. + 16., height - 16., 0.);
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform,
            texture_atlas: sprites.towers.clone(),
            sprite: TextureAtlasSprite::new(TowerClass::Wall.get_sprite_index()),
            ..Default::default()
        })
        .insert(Name::new("WallHelper"));
    let info = commands
        .spawn_bundle(TextBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "Num keys switch towers.\nAmmo: 3 - Costs: ".to_string(),
                    style: TextStyle {
                        color: Color::WHITE,
                        font_size: 24.,
                        font: sprites.countdown_font.clone(),
                    },
                },
                TextSection {
                    value: "3".to_string(),
                    style: TextStyle {
                        color: Color::rgb(1., 0.384, 0.384),
                        font_size: 24.,
                        font: sprites.countdown_font.clone(),
                    },
                },
                TextSection {
                    value: "/".to_string(),
                    style: TextStyle {
                        color: Color::WHITE,
                        font_size: 24.,
                        font: sprites.countdown_font.clone(),
                    },
                },
                TextSection {
                    value: "1".to_string(),
                    style: TextStyle {
                        color: Color::rgb(1., 1., 0.477),
                        font_size: 24.,
                        font: sprites.countdown_font.clone(),
                    },
                },
                TextSection {
                    value: "/".to_string(),
                    style: TextStyle {
                        color: Color::WHITE,
                        font_size: 24.,
                        font: sprites.countdown_font.clone(),
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        color: Color::rgb(0.635, 0.592, 1.),
                        font_size: 24.,
                        font: sprites.countdown_font.clone(),
                    },
                },
            ]),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Info"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: UiRect::new(
                    Val::Px(720.),
                    Val::Px(0.),
                    Val::Px(0.),
                    Val::Px(height - 76.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("InfoBox"))
        .insert(InGameOnly)
        .add_child(info);
}

pub fn init_game_over(mut commands: Commands, wave_status: Res<WaveStatus>, sprites: Res<Sprites>) {
    let wave_status_label = commands
        .spawn_bundle(TextBundle {
            text: Text::from_sections([
                TextSection::new(
                    format!("You made it to wave {}!\n", wave_status.wave_id),
                    TextStyle {
                        color: Color::rgb(0.43, 1., 0.38),
                        font_size: 32.,
                        font: sprites.countdown_font.clone(),
                    },
                ),
                TextSection::new(
                    format!("Press any key to play again."),
                    TextStyle {
                        color: Color::rgb(0.43, 1., 0.38),
                        font_size: 32.,
                        font: sprites.countdown_font.clone(),
                    },
                ),
            ])
            .with_alignment(TextAlignment::CENTER),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("GameOverWaveStatus"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                flex_grow: 1.0,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("GameOverScreen"))
        .insert(GameOverCleanup)
        .add_child(wave_status_label);
}

pub fn init_main_menu(mut commands: Commands, sprites: Res<Sprites>) {
    let wave_status_label = commands
        .spawn_bundle(TextBundle {
            text: Text::from_sections([
                TextSection::new(
                    format!("10"),
                    TextStyle {
                        color: Color::rgb(0.43, 1., 0.38),
                        font_size: 80.,
                        font: sprites.countdown_font.clone(),
                    },
                ),
                TextSection::new(
                    format!(" Second Tower Defense\n"),
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 80.,
                        font: sprites.countdown_font.clone(),
                    },
                ),
                TextSection::new(
                    format!("Press any key to play"),
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 32.,
                        font: sprites.countdown_font.clone(),
                    },
                ),
            ])
            .with_alignment(TextAlignment::CENTER),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("MainMenuText"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                flex_grow: 1.0,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("MainMenuScreen"))
        .insert(MainMenuOnly)
        .add_child(wave_status_label);
}

pub fn init_tutorial(mut commands: Commands, sprites: Res<Sprites>) {
    let wave_status_label = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: sprites.countdown_font.clone(),
                    font_size: 36.,
                    color: Color::rgb(0.435, 1., 0.384),
                },
            )
            .with_alignment(TextAlignment::CENTER),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InGameOnly)
        .insert(Name::new("Tutorial"))
        .id();
    commands
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::rgba(0., 0., 0., 0.)),
            style: Style {
                flex_grow: 1.0,
                align_items: AlignItems::FlexEnd,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::new(
                    Val::Undefined,
                    Val::Undefined,
                    Val::Px(120.),
                    Val::Undefined,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("TutorialNode"))
        .insert(InGameOnly)
        .add_child(wave_status_label);
}
