use emu_8080::State;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;
use std::fs;
use std::time::{Duration, Instant};

const FRAME_RATE: u64 = 60;
const SCREEN_WIDTH: u32 = 224;
const SCREEN_HEIGHT: u32 = 256;
const PIXEL_COMPONENTS: usize = 3;

type ScreenData = [[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

fn main() -> Result<(), String> {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut emulator_state = State::default();
    emulator_state.load_memory(file_bytes);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Space Invaders", SCREEN_WIDTH * 4, SCREEN_HEIGHT * 4)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut timer = Instant::now();
    let mut current_screen_line: u32 = 0;

    'running: loop {
        run_next_operation(&mut emulator_state);

        let duration = timer.elapsed();

        if duration > Duration::from_micros(1_000_000 / FRAME_RATE / SCREEN_HEIGHT as u64) {
            timer = Instant::now();
            current_screen_line += 1;
            generate_video_interrupts_if_needed(&mut emulator_state, current_screen_line);
        }

        if current_screen_line >= SCREEN_HEIGHT {
            current_screen_line = 0;
            let screen_data = get_screen_data(&emulator_state);
            let pixel_data = get_pixels_from_screen_data(&screen_data);
            texture
                .update(None, &pixel_data, SCREEN_WIDTH as usize * 3)
                .map_err(|e| e.to_string())?;

            render_next_frame(&mut canvas, &texture)?;
            let should_quit = handle_events(&mut event_pump);

            if should_quit {
                break 'running;
            }
        }

        // Crude assumption of each instruction taking 2 cycles on a 2MHz processor for the time being
        // std::thread::sleep(Duration::from_micros(1));
    }

    Ok(())
}

fn run_next_operation(state: &mut State) {
    let memory_value = state.get_memory_value_at_program_counter();
    let operation = emu_8080::disassembler::disassemble_op_code(memory_value);
    state.run_operation(operation);
}

fn raise_interrupt(state: &mut State, reset_index: u8) {
    if state.are_interrupts_enabled {
        println!("-- Raised interrupt with reset index of {} --", reset_index);
        emu_8080::branch_instructions::rst_instruction(state, reset_index);
    }
}

fn generate_video_interrupts_if_needed(state: &mut State, screen_line: u32) {
    // From http://computerarcheology.com/Arcade/SpaceInvaders/Hardware.html
    if screen_line == 96 {
        raise_interrupt(state, 1);
    } else if screen_line == SCREEN_HEIGHT {
        raise_interrupt(state, 2);
    }
}

fn get_screen_data(state: &State) -> ScreenData {
    const VIDEO_MEMORY_START: u16 = 0x2400;
    const NUM_BYTES_PER_COLUMN: u32 = SCREEN_HEIGHT / 8;
    let mut screen_data = [[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

    for screen_row_byte in 0..NUM_BYTES_PER_COLUMN {
        for screen_column in 0..SCREEN_WIDTH {
            let memory_address = VIDEO_MEMORY_START
                + (screen_column * NUM_BYTES_PER_COLUMN
                    + (NUM_BYTES_PER_COLUMN - screen_row_byte - 1)) as u16;
            let memory_value = state.get_value_at_memory_location(memory_address);

            if memory_value != 0b0000_0000 {
                for bit_index in 0_u8..=7_u8 {
                    let screen_row = screen_row_byte * 8 + (7 - bit_index) as u32;
                    let is_bit_set =
                        emu_8080::bit_operations::is_bit_set(memory_value as i8, bit_index);
                    screen_data[screen_row as usize][screen_column as usize] = is_bit_set;
                }
            }
        }
    }

    screen_data
}

fn get_pixels_from_screen_data(
    screen_data: &ScreenData,
) -> [u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize * PIXEL_COMPONENTS] {
    let mut pixel_data = [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize * PIXEL_COMPONENTS];

    for (y, row) in screen_data.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value {
                let index = (y * SCREEN_WIDTH as usize + x) * PIXEL_COMPONENTS;

                // From https://tcrf.net/File:SpaceInvadersArcColorUseTV.png
                let color = if (32..64).contains(&y) {
                    Color::RED
                } else if y >= 178 && (y < 240 || (24..136).contains(&x)) {
                    Color::GREEN
                } else {
                    Color::WHITE
                };

                pixel_data[index] = color.r;
                pixel_data[index + 1] = color.g;
                pixel_data[index + 2] = color.b;
            }
        }
    }

    pixel_data
}

fn render_next_frame(canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
    canvas.clear();
    canvas.copy(texture, None, None)?;
    canvas.present();
    Ok(())
}

fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return true;
            }
            _ => {}
        }
    }

    false
}
