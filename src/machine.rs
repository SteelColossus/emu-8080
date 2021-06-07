use std::collections::HashMap;

use emu_8080::{bit_operations, Ports, State};
use log::debug;
use maplit::hashmap;
use sdl2::keyboard::Keycode;
use sdl2::mixer;
use sdl2::mixer::{Channel, Chunk};

pub trait Machine {
    fn set_input_from_key(&mut self, key: Keycode, key_down: bool);
    fn set_ports_based_on_inputs(&mut self);
}

pub struct SpaceInvadersMachine {
    pub state: State,
    inputs: SpaceInvadersInputs,
    dip_switches: SpaceInvadersDipSwitches,
}

impl Default for SpaceInvadersMachine {
    fn default() -> Self {
        SpaceInvadersMachine {
            state: {
                let mut state = State::default();
                let ports = SpaceInvadersPorts::with_sound_map(hashmap! {
                    SoundName::Shoot => AUDIO_FOLDER_PATH.to_owned() + "shoot.wav",
                    SoundName::PlayerKilled => AUDIO_FOLDER_PATH.to_owned() + "explosion.wav",
                    SoundName::InvaderKilled => AUDIO_FOLDER_PATH.to_owned() + "invaderkilled.wav",
                    SoundName::UfoFly => AUDIO_FOLDER_PATH.to_owned() + "ufo_lowpitch.wav",
                    SoundName::UfoKilled => AUDIO_FOLDER_PATH.to_owned() + "ufo_highpitch.wav",
                    SoundName::InvaderMovement1 => AUDIO_FOLDER_PATH.to_owned() + "fastinvader1.wav",
                    SoundName::InvaderMovement2 => AUDIO_FOLDER_PATH.to_owned() + "fastinvader2.wav",
                    SoundName::InvaderMovement3 => AUDIO_FOLDER_PATH.to_owned() + "fastinvader3.wav",
                    SoundName::InvaderMovement4 => AUDIO_FOLDER_PATH.to_owned() + "fastinvader4.wav",
                });
                state.ports = Box::new(ports);
                state
            },
            inputs: SpaceInvadersInputs::default(),
            dip_switches: SpaceInvadersDipSwitches::default(),
        }
    }
}

const AUDIO_FOLDER_PATH: &str = "audio/";

impl Machine for SpaceInvadersMachine {
    fn set_input_from_key(&mut self, key: Keycode, key_down: bool) {
        match key {
            Keycode::RShift => self.inputs.credit = key_down,
            Keycode::Backquote => self.inputs.tilt = key_down,
            Keycode::Return => self.inputs.p1_start = key_down,
            Keycode::Backspace => self.inputs.p2_start = key_down,
            Keycode::Space => {
                self.inputs.p1_shoot = key_down;
                self.inputs.p2_shoot = key_down;
            }
            Keycode::Left => {
                self.inputs.p1_left = key_down;
                self.inputs.p2_left = key_down;
            }
            Keycode::Right => {
                self.inputs.p1_right = key_down;
                self.inputs.p2_right = key_down;
            }
            _ => {}
        }
    }

    fn set_ports_based_on_inputs(&mut self) {
        let mut port_0 = self.state.ports.get_in_port_static_value(0).unwrap();
        bit_operations::set_bit_in_value(
            &mut port_0,
            4,
            self.inputs.p1_shoot || self.inputs.p2_shoot,
        );
        bit_operations::set_bit_in_value(
            &mut port_0,
            5,
            self.inputs.p1_left || self.inputs.p2_left,
        );
        bit_operations::set_bit_in_value(
            &mut port_0,
            6,
            self.inputs.p1_right || self.inputs.p2_right,
        );
        self.state.ports.set_in_port_static_value(0, port_0);

        let mut port_1 = self.state.ports.get_in_port_static_value(1).unwrap();
        bit_operations::set_bit_in_value(&mut port_1, 0, self.inputs.credit);
        bit_operations::set_bit_in_value(&mut port_1, 1, self.inputs.p2_start);
        bit_operations::set_bit_in_value(&mut port_1, 2, self.inputs.p1_start);
        bit_operations::set_bit_in_value(&mut port_1, 4, self.inputs.p1_shoot);
        bit_operations::set_bit_in_value(&mut port_1, 5, self.inputs.p1_left);
        bit_operations::set_bit_in_value(&mut port_1, 6, self.inputs.p1_right);
        self.state.ports.set_in_port_static_value(1, port_1);

        let mut port_2 = self.state.ports.get_in_port_static_value(2).unwrap();
        bit_operations::set_bit_in_value(&mut port_2, 2, self.inputs.tilt);
        bit_operations::set_bit_in_value(&mut port_2, 4, self.inputs.p2_shoot);
        bit_operations::set_bit_in_value(&mut port_2, 5, self.inputs.p2_left);
        bit_operations::set_bit_in_value(&mut port_2, 6, self.inputs.p2_right);

        bit_operations::set_bit_in_value(&mut port_2, 0, self.dip_switches.num_ships_low);
        bit_operations::set_bit_in_value(&mut port_2, 1, self.dip_switches.num_ships_high);
        bit_operations::set_bit_in_value(
            &mut port_2,
            3,
            self.dip_switches.extra_ship_at_lower_score,
        );
        bit_operations::set_bit_in_value(&mut port_2, 7, self.dip_switches.coin_info_off);
        self.state.ports.set_in_port_static_value(2, port_2);
    }
}

#[derive(Eq, PartialEq, Hash)]
enum SoundName {
    Shoot,
    PlayerKilled,
    InvaderKilled,
    UfoFly,
    UfoKilled,
    InvaderMovement1,
    InvaderMovement2,
    InvaderMovement3,
    InvaderMovement4,
}

struct SpaceInvadersPorts {
    shift_data: u16,
    shift_amount: u8,
    in_port_0: u8,
    in_port_1: u8,
    in_port_2: u8,
    out_port_3: u8,
    out_port_5: u8,
    watchdog: u8,
    sound_map: HashMap<SoundName, Chunk>,
}

impl Default for SpaceInvadersPorts {
    fn default() -> Self {
        SpaceInvadersPorts {
            shift_data: 0b0000_0000_0000_0000,
            shift_amount: 0b0000_0000,
            in_port_0: 0b0000_1110,
            in_port_1: 0b0000_1000,
            in_port_2: 0b0000_0000,
            out_port_3: 0b0000_0000,
            out_port_5: 0b0000_0000,
            watchdog: 0b0000_0000,
            sound_map: HashMap::new(),
        }
    }
}

impl Ports for SpaceInvadersPorts {
    fn read_in_port(&self, port_number: u8) -> u8 {
        match port_number {
            0 => self.in_port_0,
            1 => self.in_port_1,
            2 => self.in_port_2,
            3 => {
                ((self.shift_data & (0b_1111_1111_0000_0000 >> self.shift_amount as u16))
                    >> (8 - self.shift_amount)) as u8
            }
            _ => panic!("Can't handle input Port {}", port_number),
        }
    }

    fn write_out_port(&mut self, port_number: u8, value: u8) {
        match port_number {
            3 => {
                self.play_sounds_if_needed(
                    self.out_port_3,
                    value,
                    hashmap! {
                        0 => &SoundName::UfoFly,
                        1 => &SoundName::Shoot,
                        2 => &SoundName::PlayerKilled,
                        3 => &SoundName::InvaderKilled,
                    },
                );
                self.out_port_3 = value;
            }
            5 => {
                self.play_sounds_if_needed(
                    self.out_port_5,
                    value,
                    hashmap! {
                        0 => &SoundName::InvaderMovement1,
                        1 => &SoundName::InvaderMovement2,
                        2 => &SoundName::InvaderMovement3,
                        3 => &SoundName::InvaderMovement4,
                        4 => &SoundName::UfoKilled,
                    },
                );
                self.out_port_5 = value;
            }
            2 => self.shift_amount = value & 0b0000_0111,
            4 => {
                let (_, high_shift_data) = bit_operations::split_to_low_high_bytes(self.shift_data);
                self.shift_data = bit_operations::concat_low_high_bytes(high_shift_data, value);
            }
            6 => {
                self.watchdog = value;
                debug!("Watchdog: {}", self.watchdog);
            }
            _ => panic!("Can't handle output Port {}", port_number),
        };
    }

    fn get_in_port_static_value(&self, port_number: u8) -> Option<u8> {
        match port_number {
            0 => Some(self.in_port_0),
            1 => Some(self.in_port_1),
            2 => Some(self.in_port_2),
            _ => None,
        }
    }

    fn set_in_port_static_value(&mut self, port_number: u8, value: u8) {
        match port_number {
            0 => self.in_port_0 = value,
            1 => self.in_port_1 = value,
            2 => self.in_port_2 = value,
            _ => {}
        }
    }
}

impl SpaceInvadersPorts {
    fn with_sound_map(sound_map: HashMap<SoundName, String>) -> Self {
        let mut ports = SpaceInvadersPorts::default();

        for (sound_name, file_path) in sound_map {
            let mut sound_chunk = Chunk::from_file(file_path).unwrap();
            sound_chunk.set_volume(mixer::MAX_VOLUME / 2);
            ports.sound_map.insert(sound_name, sound_chunk);
        }

        ports
    }

    fn play_sounds_if_needed(
        &self,
        port_value: u8,
        new_value: u8,
        bit_index_to_sound_name_map: HashMap<u8, &SoundName>,
    ) {
        for (bit_index, sound_name) in bit_index_to_sound_name_map {
            if bit_operations::is_bit_set(new_value, bit_index)
                && !bit_operations::is_bit_set(port_value, bit_index)
            {
                self.play_sound(sound_name);
            }
        }
    }

    fn play_sound(&self, sound_name: &SoundName) {
        let sound_chunk_result = self.sound_map.get(sound_name);

        if let Some(sound_chunk) = sound_chunk_result {
            let _ = Channel::all().play(&sound_chunk, 0);
        }
    }
}

#[derive(Default)]
struct SpaceInvadersInputs {
    pub credit: bool,
    pub tilt: bool,
    pub p1_start: bool,
    pub p1_shoot: bool,
    pub p1_left: bool,
    pub p1_right: bool,
    pub p2_start: bool,
    pub p2_shoot: bool,
    pub p2_left: bool,
    pub p2_right: bool,
}

#[derive(Default)]
struct SpaceInvadersDipSwitches {
    pub num_ships_low: bool,
    pub num_ships_high: bool,
    pub extra_ship_at_lower_score: bool,
    pub coin_info_off: bool,
}
