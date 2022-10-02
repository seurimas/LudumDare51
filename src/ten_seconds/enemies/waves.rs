use bevy::ecs::entity::Entities;

use crate::prelude::*;

use super::spawn_enemy;

#[derive(Debug)]
pub struct WaveStatus {
    time_left: f32,
    spawned: Vec<EnemyType>,
    spawns: Vec<EnemyType>,
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
            minerals: 2,
            dust: 1,
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
        match self.wave_id {
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

            id => {
                if id % 3 == 0 {
                    "stings/TinyRobotEb.ogg"
                } else {
                    "stings/TinyRobot.ogg"
                }
            }
        }
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
            self.spawned.clear();
            self.spawns
                .resize(self.wave_id.try_into().unwrap(), EnemyType::Basic);
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
    if let Some(enemy_type) = wave_status.drain_timed_spawn() {
        spawn_enemy(
            &mut commands,
            &sprites,
            field.get_spawn_transform(),
            enemy_type,
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
