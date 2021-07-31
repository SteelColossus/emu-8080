extern crate sdl2;

use std::time::{Duration, Instant};
use std::{env, fs};

use log::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;

use emu_8080::{runner, State};

use crate::machine::Machine;

mod machine;

const FRAME_RATE: u64 = 60;
const ORIGINAL_SCREEN_WIDTH: u32 = 256;
const ORIGINAL_SCREEN_HEIGHT: u32 = 224;
const NUM_PIXEL_COMPONENTS: usize = 3;
const SCREEN_DATA_SIZE: usize =
    (ORIGINAL_SCREEN_WIDTH * ORIGINAL_SCREEN_HEIGHT) as usize * NUM_PIXEL_COMPONENTS;

fn main() -> Result<(), String> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let file_name = args
        .get(1)
        .expect("Must provide a filename argument for a game to play")
        .as_str();

    let expected_extension = ".bin";
    let mut machine: Box<dyn Machine> = match file_name {
        "invaders.bin" => Box::new(machine::SpaceInvadersMachine::default()),
        "boothill.bin" => Box::new(machine::BootHillMachine::default()),
        _ => {
            if &file_name[file_name.len() - expected_extension.len()..file_name.len()]
                == expected_extension
            {
                let machine_name = &file_name[..file_name.len() - expected_extension.len()];
                let orientation = match machine_name {
                    "lagunar" => 90,
                    _ => 0,
                };
                Box::new(machine::BlankMachine::with_name_and_orientation(
                    machine_name.to_string(),
                    orientation,
                ))
            } else {
                panic!("Can't play game with filename {}", file_name);
            }
        }
    };

    let file_bytes = fs::read(file_name)
        .unwrap_or_else(|_| panic!("Could not read a file with filename {}", file_name));
    machine.get_state_mut().load_memory(file_bytes);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Based on audio file bitrate of 88kbps
    mixer::open_audio(11_025, mixer::AUDIO_U8, 1, 1_024)?;

    let (screen_width, screen_height) = get_screen_dimensions(&*machine);
    let window = video_subsystem
        .window(machine.get_name(), screen_width * 4, screen_height * 4)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, screen_width, screen_height)
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut timer = Instant::now();
    let mut current_screen_line: u32 = 0;

    'running: loop {
        runner::run_next_operation(&mut machine.get_state_mut());

        let duration = timer.elapsed();

        if duration > Duration::from_micros(1_000_000 / FRAME_RATE / screen_height as u64) {
            timer = Instant::now();
            current_screen_line += 1;
            generate_video_interrupts_if_needed(
                &mut machine.get_state_mut(),
                current_screen_line,
                screen_height,
            );
        }

        if current_screen_line >= screen_height {
            current_screen_line = 0;
            let screen_pixel_data = get_screen_pixel_data(&*machine);
            texture
                .update(
                    None,
                    &screen_pixel_data,
                    screen_width as usize * NUM_PIXEL_COMPONENTS,
                )
                .map_err(|e| e.to_string())?;

            render_next_frame(&mut canvas, &texture)?;
            let should_quit = handle_events(&mut event_pump, &mut machine);

            if should_quit {
                break 'running;
            }

            machine.set_ports_based_on_inputs();
        }

        // Crude assumption of each instruction taking 2 cycles on a 2MHz processor for the time being
        // std::thread::sleep(Duration::from_micros(1));
    }

    Ok(())
}

fn get_screen_dimensions(machine: &dyn Machine) -> (u32, u32) {
    let screen_orientation = machine.get_orientation();
    let is_screen_rotated = (screen_orientation % 360 / 90) % 2 == 1;
    let screen_width = if is_screen_rotated {
        ORIGINAL_SCREEN_HEIGHT
    } else {
        ORIGINAL_SCREEN_WIDTH
    };
    let screen_height = if is_screen_rotated {
        ORIGINAL_SCREEN_WIDTH
    } else {
        ORIGINAL_SCREEN_HEIGHT
    };
    (screen_width, screen_height)
}

fn raise_interrupt(state: &mut State, reset_index: u8) {
    if state.are_interrupts_enabled {
        debug!("-- Raised interrupt with reset index of {} --", reset_index);
        emu_8080::branch_instructions::rst_instruction(state, reset_index);
    }
}

fn generate_video_interrupts_if_needed(state: &mut State, screen_line: u32, max_screen_line: u32) {
    // From http://computerarcheology.com/Arcade/SpaceInvaders/Hardware.html
    if screen_line == 96 {
        raise_interrupt(state, 1);
    } else if screen_line == max_screen_line {
        raise_interrupt(state, 2);
    }
}

fn get_screen_pixel_data(machine: &dyn Machine) -> [u8; SCREEN_DATA_SIZE] {
    const VIDEO_MEMORY_START: u16 = 0x2400;
    let num_bytes_per_original_row: u32 = ORIGINAL_SCREEN_WIDTH / 8;
    let mut screen_pixel_data = [0b0000_0000; SCREEN_DATA_SIZE];
    let state = machine.get_state();

    let (screen_width, screen_height) = get_screen_dimensions(machine);
    let screen_orientation = machine.get_orientation();

    for original_screen_row in 0..ORIGINAL_SCREEN_HEIGHT {
        for original_screen_column_byte in 0..num_bytes_per_original_row {
            let memory_address = VIDEO_MEMORY_START
                + (original_screen_row * num_bytes_per_original_row + original_screen_column_byte)
                    as u16;
            let memory_value = state.get_value_at_memory_location(memory_address);

            if memory_value != 0b0000_0000 {
                set_original_column_byte_pixels(
                    machine,
                    &mut screen_pixel_data,
                    screen_width,
                    screen_height,
                    screen_orientation,
                    original_screen_column_byte,
                    original_screen_row,
                    memory_value,
                )
            }
        }
    }

    screen_pixel_data
}

fn set_original_column_byte_pixels(
    machine: &dyn Machine,
    screen_pixel_data: &mut [u8; SCREEN_DATA_SIZE],
    screen_width: u32,
    screen_height: u32,
    screen_orientation: u32,
    original_screen_column_byte: u32,
    original_screen_row: u32,
    memory_value: u8,
) {
    let num_screen_turns = screen_orientation % 360 / 90;

    for bit_index in 0_u8..=7_u8 {
        let is_bit_set = emu_8080::bit_operations::is_bit_set(memory_value, bit_index);

        if is_bit_set {
            let original_screen_column = original_screen_column_byte * 8 + bit_index as u32;

            let (x, y) = match num_screen_turns {
                0 => (original_screen_column, original_screen_row),
                1 => (
                    (screen_width - 1) - original_screen_row,
                    original_screen_column,
                ),
                2 => (
                    (screen_width - 1) - original_screen_column,
                    (screen_height - 1) - original_screen_row,
                ),
                3 => (
                    original_screen_row,
                    (screen_height - 1) - original_screen_column,
                ),
                _ => unreachable!(),
            };

            let index = (y * screen_width + x) as usize * NUM_PIXEL_COMPONENTS;

            set_pixel(
                machine,
                &mut screen_pixel_data[index..(index + NUM_PIXEL_COMPONENTS)],
                x,
                y,
            );
        }
    }
}

fn set_pixel(machine: &dyn Machine, pixel_slice: &mut [u8], x: u32, y: u32) {
    let color = machine.get_pixel_color(x, y);
    pixel_slice[0] = color.r;
    pixel_slice[1] = color.g;
    pixel_slice[2] = color.b;
}

fn render_next_frame(canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
    canvas.clear();
    canvas.copy(texture, None, None)?;
    canvas.present();
    Ok(())
}

fn handle_events(event_pump: &mut EventPump, machine: &mut Box<dyn Machine>) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return true;
            }
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                machine.set_input_from_key(key, true);
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                machine.set_input_from_key(key, false);
            }
            _ => {}
        }
    }

    false
}
