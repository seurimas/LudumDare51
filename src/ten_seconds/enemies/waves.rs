use crate::prelude::*;

use super::spawn_enemy;

#[derive(Debug)]
pub struct WaveStatus {
    time_left: f32,
    spawned: Vec<EnemyType>,
    spawns: Vec<EnemyType>,
    wave_id: i32,
}

impl Default for WaveStatus {
    fn default() -> Self {
        WaveStatus {
            time_left: 10.,
            spawned: vec![],
            spawns: vec![EnemyType::Basic, EnemyType::Basic, EnemyType::Basic],
            wave_id: 1,
        }
    }
}

impl WaveStatus {
    fn get_total_spawns(&self) -> usize {
        self.spawned.len() + self.spawns.len()
    }

    fn drain_timed_spawn(&mut self) -> Option<EnemyType> {
        if self.spawns.len() == 0 {
            return None;
        }
        let delay = 3.0 / self.get_total_spawns() as f32;
        let progress = 10. - self.time_left;
        let spawn_index = (progress / delay) as usize;
        if spawn_index >= self.spawned.len() {
            let new_enemy = self.spawns.pop().unwrap();
            self.spawned.push(new_enemy);
            Some(new_enemy)
        } else {
            None
        }
    }

    fn drain_wave_end(&mut self) -> bool {
        if self.time_left <= 0. {
            self.time_left += 10.;
            self.wave_id += 1;
            self.spawns.extend(self.spawned.drain(..));
            true
        } else {
            false
        }
    }
}

pub fn wave_system(
    mut commands: Commands,
    time: Res<Time>,
    sprites: Res<Sprites>,
    field: Res<Field>,
    mut wave_status: ResMut<WaveStatus>,
) {
    wave_status.time_left -= time.delta_seconds();
    if let Some(enemy_type) = wave_status.drain_timed_spawn() {
        spawn_enemy(
            &mut commands,
            &sprites,
            field.get_spawn_transform(),
            enemy_type,
        );
    }
    if wave_status.drain_wave_end() {
        println!("Wave {:?}", wave_status.wave_id);
    }
}
