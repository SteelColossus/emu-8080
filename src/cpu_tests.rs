use crate::{disassembler, Operation, Register, RegisterPair, State};

fn run_next_operation(state: &mut State) -> bool {
    let memory_value = state.get_memory_value_at_program_counter();
    let operation = disassembler::disassemble_op_code(memory_value);

    if operation == Operation::Out {
        let port_number = state.get_value_at_memory_location(state.program_counter + 1);
        match port_number {
            0 => return true,
            1 => print_test_output(state),
            _ => {}
        };
    }

    state.run_operation(operation);
    false
}

fn print_test_output(state: &State) {
    let register_c = state.get_register_value(Register::C);

    match register_c {
        2 => {
            let register_e = state.get_register_value(Register::E);
            print!("{}", register_e as char);
        }
        9 => {
            let mut memory_address = RegisterPair::DE.get_full_value(state);
            let mut memory_character: char;

            loop {
                memory_character = state.get_value_at_memory_location(memory_address) as char;
                if memory_character == '$' {
                    break;
                } else {
                    print!("{}", memory_character);
                    memory_address += 1;
                }
            }
        }
        _ => panic!("Unexpected Register C value: {}", register_c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateBuilder;
    use std::fs;

    #[test]
    fn tst8080_test() {
        let pc_start = 0x0100;
        let mut file_bytes = fs::read("cpu_tests/TST8080.COM").unwrap();

        for memory_index in (0..pc_start).rev() {
            let memory_value = match memory_index {
                0x0000 => 0xD3,
                0x0001 => 0x00,
                0x0005 => 0xD3,
                0x0006 => 0x01,
                0x0007 => 0xC9,
                _ => 0b0000_0000,
            };

            file_bytes.insert(0, memory_value);
        }

        let mut state = StateBuilder::default().program_counter(pc_start).build();
        state.load_memory(file_bytes);

        'running: loop {
            let should_quit = run_next_operation(&mut state);
            if should_quit {
                break 'running;
            }
        }
    }
}
