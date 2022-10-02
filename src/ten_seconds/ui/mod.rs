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
        .push_children(&crystals[..]);

    // COUNTDOWN
    let countdown = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "10",
                TextStyle {
                    color: Color::rgb(43. / 256., 100. / 256., 38. / 256.),
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
        .add_child(countdown);

    // MINERALS
    let minerals = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    color: Color::rgb(43. / 256., 100. / 256., 38. / 256.),
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
        .insert(Name::new("MineralsBox"))
        .add_child(minerals);

    // DUST
    let minerals = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    color: Color::rgb(43. / 256., 100. / 256., 38. / 256.),
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
                    Val::Px(720.),
                    Val::Px(180.),
                    Val::Px(0.),
                    Val::Px(height - 76.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("DustBox"))
        .add_child(minerals);

    // TECH
    let minerals = commands
        .spawn_bundle(TextBundle {
            text: Text::from_section(
                "0",
                TextStyle {
                    color: Color::rgb(43. / 256., 100. / 256., 38. / 256.),
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
                    Val::Px(840.),
                    Val::Px(60.),
                    Val::Px(0.),
                    Val::Px(height - 76.),
                ),
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("TechBox"))
        .add_child(minerals);
}
