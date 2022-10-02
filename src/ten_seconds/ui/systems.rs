use crate::prelude::*;

use super::HealthCrystal;

const FADE_IN_SECONDS: f32 = 3.;
const GAME_OVER_OPACITY: f32 = 0.95;

pub fn fade_in_game_over(
    time: Res<Time>,
    mut state: ResMut<State<AppState>>,
    mut wave_status: ResMut<WaveStatus>,
    mut game_over_query: Query<(&mut UiColor, &Name)>,
    input: Res<Input<KeyCode>>,
) {
    let delta_seconds = time.delta_seconds();
    let mut game_is_over = false;
    for (mut color, name) in game_over_query.iter_mut() {
        if name.eq_ignore_ascii_case("GameOverScreen") {
            let old_a = color.0.a();
            color.0.set_a(f32::min(
                old_a + (delta_seconds * GAME_OVER_OPACITY / FADE_IN_SECONDS),
                GAME_OVER_OPACITY,
            ));
            game_is_over = old_a >= GAME_OVER_OPACITY;
        }
    }
    if game_is_over && input.get_just_pressed().len() > 0 {
        *wave_status = WaveStatus::default();
        state.set(AppState::InGame);
    }
}

pub fn update_countdown(
    mut countdown_query: Query<(&mut Text, &Name)>,
    wave_status: Res<WaveStatus>,
) {
    for (mut text, name) in countdown_query.iter_mut() {
        if name.eq_ignore_ascii_case("Countdown") {
            text.sections[0].value = wave_status.get_countdown_value();
        }
    }
}

pub fn update_health(
    sprites: Res<Sprites>,
    mut health_crystal_query: Query<(&mut UiImage, &HealthCrystal, &mut Visibility)>,
    wave_status: Res<WaveStatus>,
) {
    let full = wave_status.health / 2;
    let has_half = wave_status.health % 2 == 1;
    for (mut image, crystal, mut visibility) in health_crystal_query.iter_mut() {
        if (crystal.0 as i32) < full {
            image.0 = sprites.crystal_full.clone();
            visibility.is_visible = true;
        } else if crystal.0 as i32 == full && has_half {
            image.0 = sprites.crystal_half.clone();
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }
}
