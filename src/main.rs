use emu_8080::State;
use std::fs;
use std::time::{Instant, Duration};

const FRAME_RATE: u64 = 60;

fn main() {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut emulator_state = State::default();
    emulator_state.load_memory(file_bytes);

    let mut timer = Instant::now();
    let mut frame_count = 0;

    loop {
        let memory_value = emulator_state.get_memory_value_at_program_counter();
        let operation = emu_8080::disassembler::disassemble_op_code(memory_value);
        emulator_state.run_operation(operation);

        let duration = timer.elapsed();

        if duration > Duration::from_millis(1000 / FRAME_RATE) {
            timer = Instant::now();
            frame_count += 1;
            generate_video_interrupts_if_needed(&mut emulator_state, frame_count % FRAME_RATE);
        }
    }
}

fn raise_interrupt(state: &mut State, reset_index: u8) {
    if state.are_interrupts_enabled {
        println!("-- Raised interrupt with reset index of {} --", reset_index);
        emu_8080::branch_instructions::rst_instruction(state, reset_index);
    }
}

fn generate_video_interrupts_if_needed(state: &mut State, frame_count: u64) {
    let screen_line = frame_count * (224 / FRAME_RATE);

    if screen_line == 93 {
        raise_interrupt(state, 1);
    } else if screen_line == 0 {
        raise_interrupt(state, 2);
    }
}
