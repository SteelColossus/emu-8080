use emu_8080::{bit_operations, Ports, State};
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
const NUM_PIXEL_COMPONENTS: usize = 3;
const SCREEN_DATA_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize * NUM_PIXEL_COMPONENTS;

#[derive(Default)]
struct SpaceInvadersPorts {
    shift_data: u16,
    shift_amount: u8,
}

impl Ports for SpaceInvadersPorts {
    fn get_in_port(&self, port_number: u8) -> i8 {
        match port_number {
            1 => 0b0000_1000,
            2 => 0b0000_0000,
            3 => {
                ((self.shift_data & (0b_1111_1111_0000_0000 >> self.shift_amount as u16))
                    >> (8 - self.shift_amount)) as i8
            }
            _ => panic!("Can't handle Port {}", port_number),
        }
    }

    fn set_out_port(&mut self, port_number: u8, value: i8) {
        match port_number {
            3 | 5 | 6 => (),
            2 => self.shift_amount = value as u8 & 0b0000_0111,
            4 => {
                let (_, high_shift_data) = bit_operations::split_to_low_high_bytes(self.shift_data);
                self.shift_data =
                    bit_operations::concat_low_high_bytes(high_shift_data, value as u8);
            }
            _ => panic!("Can't handle Port {}", port_number),
        };
    }
}

fn main() -> Result<(), String> {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut emulator_state = State::default();
    emulator_state.load_memory(file_bytes);
    emulator_state.ports = Box::new(SpaceInvadersPorts::default());

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
            let screen_pixel_data = get_screen_pixel_data(&emulator_state);
            texture
                .update(
                    None,
                    &screen_pixel_data,
                    SCREEN_WIDTH as usize * NUM_PIXEL_COMPONENTS,
                )
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

fn get_screen_pixel_data(state: &State) -> [u8; SCREEN_DATA_SIZE] {
    const VIDEO_MEMORY_START: u16 = 0x2400;
    const NUM_BYTES_PER_COLUMN: u32 = SCREEN_HEIGHT / 8;
    let mut screen_pixel_data = [0b0000_0000; SCREEN_DATA_SIZE];

    for screen_row_byte in 0..NUM_BYTES_PER_COLUMN {
        for screen_column in 0..SCREEN_WIDTH {
            let memory_address = VIDEO_MEMORY_START
                + (screen_column * NUM_BYTES_PER_COLUMN
                    + (NUM_BYTES_PER_COLUMN - screen_row_byte - 1)) as u16;
            let memory_value = state.get_value_at_memory_location(memory_address);

            if memory_value != 0b0000_0000 {
                set_row_byte_pixels(
                    &mut screen_pixel_data,
                    screen_row_byte,
                    screen_column,
                    memory_value,
                )
            }
        }
    }

    screen_pixel_data
}

fn set_row_byte_pixels(
    screen_pixel_data: &mut [u8; SCREEN_DATA_SIZE],
    screen_row_byte: u32,
    screen_column: u32,
    memory_value: u8,
) {
    for bit_index in 0_u8..=7_u8 {
        let is_bit_set = emu_8080::bit_operations::is_bit_set(memory_value as i8, bit_index);

        if is_bit_set {
            let screen_row = screen_row_byte * 8 + (7 - bit_index) as u32;
            let index = (screen_row * SCREEN_WIDTH + screen_column) as usize * NUM_PIXEL_COMPONENTS;
            set_pixel(
                &mut screen_pixel_data[index..(index + NUM_PIXEL_COMPONENTS)],
                screen_column,
                screen_row,
            );
        }
    }
}

fn set_pixel(pixel_slice: &mut [u8], x: u32, y: u32) {
    // From https://tcrf.net/File:SpaceInvadersArcColorUseTV.png
    let color = if (32..64).contains(&y) {
        Color::RED
    } else if y >= 178 && (y < 240 || (24..136).contains(&x)) {
        Color::GREEN
    } else {
        Color::WHITE
    };

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
