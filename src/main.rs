use emu_8080::State;
use std::fs;

fn main() {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut emulator_state = State::default();
    emulator_state.load_memory(file_bytes);

    loop {
        let memory_value = emulator_state.get_memory_value_at_program_counter();
        let op_code = emu_8080::disassembler::disassemble_op_code(memory_value);
        emulator_state.run_operation(op_code);
    }
}
