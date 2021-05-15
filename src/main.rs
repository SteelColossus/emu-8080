use std::fs;
use emu_8080::bit_operations;

mod disassembler;

fn main() {
    let file_bytes = fs::read("invaders.bin").unwrap();
    let mut temp_pc = 0;

    while temp_pc < file_bytes.len() {
        let file_byte = file_bytes[temp_pc];
        let op_code = disassembler::disassemble_op_code(file_byte);
        let op_code_pc = temp_pc;
        temp_pc += 1;

        let mut additional_byte_1 = None;
        let mut additional_byte_2 = None;
        let num_additional_bytes = op_code.num_additional_bytes();

        if num_additional_bytes >= 1 {
            additional_byte_1 = Some(file_bytes[temp_pc]);
            temp_pc += 1;
        }

        if num_additional_bytes >= 2 {
            additional_byte_2 = Some(file_bytes[temp_pc]);
            temp_pc += 1;
        }

        if let (Some(byte_1), Some(byte_2)) = (additional_byte_1, additional_byte_2) {
            println!("{:04X?} {:?} {:04X?}", op_code_pc, op_code, bit_operations::concat_low_high_bytes(byte_1, byte_2));
        } else if let Some(byte_1) = additional_byte_1 {
            println!("{:04X?} {:?} {:02X?}", op_code_pc, op_code, byte_1);
        } else {
            println!("{:04X?} {:?}", op_code_pc, op_code);
        }
    }
}
