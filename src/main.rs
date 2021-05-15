use emu_8080::State;
use std::fs;

fn main() {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut emulator_state = State::default();
    emulator_state.load_memory(file_bytes);

    let known_good_op_codes: Vec<u8> = vec![
        0x00, 0x01, 0x05, 0x06, 0x09, 0x0d, 0x0e, 0x0f, 0x11, 0x13, 0x19, 0x1a, 0x21, 0x23, 0x26,
        0x29, 0x31, 0x32, 0x36, 0x3a, 0x3e, 0x56, 0x5e, 0x66, 0x6f, 0x77, 0x7a, 0x7b, 0x7c, 0x7e,
        0xa7, 0xaf, 0xc1, 0xc2, 0xc3, 0xc5, 0xc6, 0xc9, 0xcd, 0xd1, 0xd3, 0xd5, 0xe1, 0xe5, 0xe6,
        0xeb, 0xf1, 0xf5, 0xfb, 0xfe,
    ];

    loop {
        let memory_value = emulator_state.get_memory_value_at_program_counter();
        assert!(known_good_op_codes.contains(&memory_value));
        let operation = emu_8080::disassembler::disassemble_op_code(memory_value);
        emulator_state.run_operation(operation);
    }
}
