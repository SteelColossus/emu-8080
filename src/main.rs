use std::fs;

mod disassembler;

fn main() {
    let file_bytes = fs::read("invaders.h").unwrap();
    disassembler::disassemble(&file_bytes);
}
