use crate::prelude::*;

use super::HealthCrystal;

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
