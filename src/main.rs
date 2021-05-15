use emu_8080::State;
use std::fs;
use std::time::{Duration, Instant};
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

const FRAME_RATE: u64 = 60;
const SCREEN_WIDTH: u16 = 256;
const SCREEN_HEIGHT: u16 = 224;

fn main() {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut emulator_state = State::default();
    emulator_state.load_memory(file_bytes);

    let mut timer = Instant::now();
    let mut frame_count = 0;
    let mut screen_frame = 1;

    loop {
        let memory_value = emulator_state.get_memory_value_at_program_counter();
        let operation = emu_8080::disassembler::disassemble_op_code(memory_value);
        emulator_state.run_operation(operation);

        let duration = timer.elapsed();

        if duration > Duration::from_millis(1000 / FRAME_RATE) {
            timer = Instant::now();
            frame_count += 1;
            let was_interrupt_generated = generate_video_interrupts_if_needed(&mut emulator_state, frame_count % FRAME_RATE);

            if was_interrupt_generated {
                print_screen(&emulator_state, screen_frame);
                screen_frame += 1;
            }
        }
    }
}

fn raise_interrupt(state: &mut State, reset_index: u8) {
    if state.are_interrupts_enabled {
        println!("-- Raised interrupt with reset index of {} --", reset_index);
        emu_8080::branch_instructions::rst_instruction(state, reset_index);
    }
}

fn generate_video_interrupts_if_needed(state: &mut State, frame_count: u64) -> bool {
    let screen_line = frame_count * (SCREEN_HEIGHT as u64 / FRAME_RATE);

    if screen_line == 93 {
        raise_interrupt(state, 1);
        return true;
    } else if screen_line == 0 {
        raise_interrupt(state, 2);
        return true;
    }

    false
}

fn print_screen(state: &State, screen_frame: u64) {
    const VIDEO_MEMORY_START: u16 = 0x2400;
    let mut screen_as_numbers = String::new();

    for screen_row in 0..SCREEN_HEIGHT {
        for screen_height_byte in 0..SCREEN_WIDTH / 8 {
            let memory_address = VIDEO_MEMORY_START + screen_row * SCREEN_WIDTH / 8 + screen_height_byte;
            let memory_value = state.get_value_at_memory_location(memory_address);
            write!(&mut screen_as_numbers, "{:08b}", memory_value);
        }

        write!(&mut screen_as_numbers, "\n");
    }

    let filepath = format!("screens/screen_frame_{}.txt", screen_frame);
    // fs::write(filepath, screen_as_numbers);
}
