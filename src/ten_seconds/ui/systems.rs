use crate::prelude::*;

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
