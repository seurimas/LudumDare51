use bevy::ecs::entity::Entities;

use crate::prelude::*;

use super::spawn_enemy;

#[derive(Debug)]
pub struct WaveStatus {
    time_left: f32,
    spawned: Vec<(EnemyType, i32)>,
    spawns: Vec<(EnemyType, i32)>,
    game_over: bool,
    pub wave_id: i32,
    pub health: i32,
    pub minerals: i32,
    pub dust: i32,
    pub tech: i32,
    pub tower_type: TowerType,
}

impl Default for WaveStatus {
    fn default() -> Self {
        WaveStatus {
            time_left: 10.,
            spawned: vec![],
            spawns: vec![],
            game_over: false,
            wave_id: 0,
            health: 20,
            minerals: 6,
            dust: 2,
            tech: 0,
            tower_type: TowerType::Attack,
        }
    }
}

impl WaveStatus {
    pub fn get_countdown_value(&self) -> String {
        format!("{}", self.time_left.floor())
    }

    fn get_total_spawns(&self) -> usize {
        self.spawned.len() + self.spawns.len()
    }

    fn get_sting(&self) -> &'static str {
        self.get_sting_for(self.wave_id)
    }

    fn get_sting_for(&self, wave_id: i32) -> &'static str {
        match wave_id {
            1 | 2 => "stings/ChoirSaprano.ogg",
            3 => "stings/ChoirSapranoEb.ogg",

            4 | 5 => "stings/ChoirTenor.ogg",
            6 => "stings/ChoirTenorEb.ogg",

            7 | 8 => "stings/ChoirBass.ogg",
            9 => "stings/ChoirBassEb.ogg",

            10 | 11 => "stings/FmBass.ogg",
            12 => "stings/FmBassEb.ogg",

            13 | 14 => "stings/ElectricGrand.ogg",
            15 => "stings/ElectricGrandEb.ogg",

            16 | 17 => "stings/SaxSynth.ogg",
            18 => "stings/SaxSynthEb.ogg",

            19 | 20 => "stings/Harmonica.ogg",
            21 => "stings/HarmonicaEb.ogg",

            22 | 23 => "stings/Sawtooth.ogg",
            24 => "stings/SawtoothEb.ogg",

            25 | 26 => "stings/SquareWave.ogg",
            27 => "stings/SquareWaveEb.ogg",

            28 | 29 => "stings/SquareWave.ogg",
            30 => "stings/SquareWaveEb.ogg",

            id => self.get_sting_for(id - 30),
        }
    }

    fn drain_timed_spawn(&mut self) -> Option<(EnemyType, i32)> {
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
            self.spawned.clear();
            self.spawns = get_spawns(self.wave_id);
            true
        } else {
            false
        }
    }

    fn drain_game_over(&mut self) -> bool {
        if self.health <= 0 && !self.game_over {
            self.game_over = true;
            true
        } else {
            false
        }
    }

    pub fn loot(&mut self, enemy_type: &EnemyType) {
        self.minerals += enemy_type.get_mineral_loot();
        self.dust += enemy_type.get_dust_loot();
        self.tech += enemy_type.get_tech_loot();
    }

    pub fn buy(&mut self, tower_type: TowerType) -> bool {
        let minerals = tower_type.get_mineral_cost();
        let dust = tower_type.get_dust_cost();
        let tech = tower_type.get_tech_cost();
        if self.minerals >= minerals && self.dust >= dust && self.tech >= tech {
            self.minerals -= minerals;
            self.dust -= dust;
            self.tech -= tech;
            true
        } else {
            false
        }
    }

    pub fn sell(&mut self, tower_type: TowerType) {
        self.minerals += tower_type.get_mineral_deconstruct();
        self.dust += tower_type.get_dust_deconstruct();
        self.tech += tower_type.get_tech_deconstruct();
    }

    pub fn get_tutorial(&self) -> Option<&'static str> {
        match self.wave_id {
            0 => Some("Place a tower near the center line by left-clicking."),
            1 => Some("Towers have ammo that refreshes every 10 seconds.\nPlace more towers."),
            2 => Some("You can also place ammo silos next to towers.\nPress '2' to switch to silos."),
            3 => Some("Some enemies will place markers when they die.\nOther enemies will avoid those markers."),
            4 => Some("You can desconstruct a tower by right-clicking.\nYou'll lose some minerals and tech."),
            5 => Some("Thieves will steal ammo from nearby towers to heal themselves."),
            7 => Some("Enemies may spawn with a different color\n and boosted health."),
            8 => Some("Tunnel Busters cannot die in tunnels.\nThey also have more health."),
            _ => None,
        }
    }
}

pub struct WaveEndEvent(pub i32);

pub fn wave_system(
    mut commands: Commands,
    time: Res<Time>,
    sprites: Res<Sprites>,
    field: Res<Field>,
    mut ev_wave_end: EventWriter<WaveEndEvent>,
    mut wave_status: ResMut<WaveStatus>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    wave_status.time_left -= time.delta_seconds();
    if let Some((enemy_type, boosts)) = wave_status.drain_timed_spawn() {
        spawn_enemy(
            &mut commands,
            &sprites,
            field.get_spawn_transform(),
            enemy_type,
            boosts,
        );
    }
    if wave_status.drain_wave_end() {
        ev_wave_end.send(WaveEndEvent(wave_status.wave_id - 1));
        audio.play(asset_server.load(wave_status.get_sting()));
    }
}

pub fn goal_system(
    mut commands: Commands,
    field: Res<Field>,
    sounds: Res<Sounds>,
    mut wave_status: ResMut<WaveStatus>,
    mut state: ResMut<State<AppState>>,
    entities: &Entities,
    audio: Res<Audio>,
) {
    for (entity, _location) in field.get_enemies_in_tile(&field.get_goal()) {
        if entities.contains(*entity) {
            commands.entity(*entity).despawn();
            wave_status.health -= 1;
            if wave_status.drain_game_over() {
                state.set(AppState::GameOver).unwrap();
                audio.play_with_settings(
                    sounds.game_over.clone(),
                    PlaybackSettings {
                        repeat: false,
                        volume: 1.,
                        speed: 1.,
                    },
                );
            } else {
                audio.play_with_settings(
                    sounds.goal_hit.clone(),
                    PlaybackSettings {
                        repeat: false,
                        volume: 1.,
                        speed: 1.,
                    },
                );
            }
        }
    }
}

fn get_spawns(wave_id: i32) -> Vec<(EnemyType, i32)> {
    match wave_id {
        1 => vec![(EnemyType::Basic, 0)],
        2 => vec![(EnemyType::Basic, 0), (EnemyType::Basic, 0)],
        3 => vec![
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Seeker, 0),
        ],
        4 => vec![
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Seeker, 0),
        ],
        5 => vec![
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Seeker, 0),
            (EnemyType::Seeker, 0),
            (EnemyType::Seeker, 0),
        ],
        6 => vec![
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Basic, 0),
            (EnemyType::Thief, 0),
            (EnemyType::Thief, 0),
        ],
        7 => vec![
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Fast, 1),
            (EnemyType::Fast, 0),
            (EnemyType::Fast, 0),
            (EnemyType::Thief, 0),
            (EnemyType::Thief, 0),
        ],
        8 => vec![
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Gnat, 0),
            (EnemyType::Gnat, 0),
            (EnemyType::Fast, 0),
            (EnemyType::Fast, 0),
            (EnemyType::Fast, 0),
            (EnemyType::Fast, 0),
            (EnemyType::Seeker, 1),
            (EnemyType::Seeker, 1),
        ],
        9 => vec![
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 1),
            (EnemyType::Basic, 2),
            (EnemyType::Basic, 2),
            (EnemyType::Buster, 0),
            (EnemyType::Buster, 0),
            (EnemyType::Buster, 0),
        ],
        simple_count => {
            let mut wave = Vec::new();
            let mut wave_cost = simple_count * 2;
            while wave_cost > 0 {
                match (rand::random::<f32>() * 10 as f32).floor() as i32 {
                    9 | 8 => {
                        if wave_cost > 3 {
                            wave.push((EnemyType::Buster, 0));
                            wave_cost -= 3;
                        } else {
                            wave.push((EnemyType::Basic, 0));
                            wave_cost -= 1;
                        }
                    }
                    7 | 6 => {
                        wave.push((EnemyType::Seeker, 0));
                        wave_cost -= 1;
                    }
                    5 | 4 => {
                        if wave_cost > 2 {
                            wave.push((EnemyType::Fast, 0));
                            wave_cost -= 2;
                        } else {
                            wave.push((EnemyType::Basic, 0));
                            wave_cost -= 1;
                        }
                    }
                    3 => {
                        if wave_cost > 2 {
                            wave.push((EnemyType::Thief, 0));
                            wave_cost -= 2;
                        } else {
                            wave.push((EnemyType::Basic, 0));
                            wave_cost -= 1;
                        }
                    }
                    2 | 1 => {
                        wave.push((EnemyType::Gnat, 0));
                        wave.push((EnemyType::Gnat, 0));
                        wave.push((EnemyType::Gnat, 0));
                        wave_cost -= 1;
                    }
                    _ => {
                        wave.push((EnemyType::Basic, 0));
                        wave_cost -= 1;
                    }
                }
                if wave_cost > 2 {
                    let boosted = (rand::random::<f32>() * wave.len() as f32).floor() as usize;
                    wave[boosted].1 += 1;
                    wave_cost -= 1;
                }
            }
            wave
        }
    }
}
