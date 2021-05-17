use emu_8080::State;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;
use std::fs;
use std::time::{Duration, Instant};

const FRAME_RATE: u64 = 60;
const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 224;
const SCREEN_DATA_SIZE: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

fn main() -> Result<(), String> {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut emulator_state = State::default();
    emulator_state.load_memory(file_bytes);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Space Invaders", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB332, SCREEN_WIDTH, SCREEN_HEIGHT)
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
            texture
                .update(None, &screen_data, SCREEN_WIDTH as usize)
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

fn get_screen_data(state: &State) -> [u8; SCREEN_DATA_SIZE] {
    const VIDEO_MEMORY_START: u32 = 0x2400;
    const BYTES_PER_ROW: u32 = SCREEN_WIDTH / 8;
    let mut screen_data = [0; SCREEN_DATA_SIZE];

    for screen_row in 0..SCREEN_HEIGHT {
        for screen_height_byte in 0..BYTES_PER_ROW {
            let memory_address =
                (VIDEO_MEMORY_START + screen_row * BYTES_PER_ROW + screen_height_byte) as u16;
            let memory_value = state.get_value_at_memory_location(memory_address);

            if memory_value != 0 {
                for bit_index in 0..7 {
                    if emu_8080::bit_operations::is_bit_set(memory_value as i8, bit_index) {
                        let screen_pixel_x = screen_height_byte * 8 + bit_index as u32;
                        let screen_data_index =
                            (screen_row * SCREEN_WIDTH + screen_pixel_x) as usize;
                        screen_data[screen_data_index] = 0b11111111;
                    }
                }
            }
        }
    }

    screen_data
}

fn render_next_frame(canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
    canvas.clear();
    canvas.copy_ex(texture, None, None, -90.0, None, false, false)?;
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
