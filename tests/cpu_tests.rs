use emu_8080::{disassembler, Operation, Register, RegisterPair, State, StateBuilder};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn run_next_operation(state: &mut State) -> bool {
    let memory_value = state.memory_value_at_pc();
    let operation = disassembler::disassemble_op_code(memory_value);
    let is_out_operation = operation == Operation::Out;

    state.run_operation(operation);

    if is_out_operation {
        let port_number = state.memory[(state.program_counter - 1) as usize];
        match port_number {
            0 => return true,
            1 => print_test_output(state),
            _ => {}
        };
    }

    false
}

fn print_test_output(state: &State) {
    let register_c = state.registers[Register::C];

    match register_c {
        2 => {
            let register_e = state.registers[Register::E];
            print!("{}", register_e as char);
        }
        9 => {
            let mut memory_address = state.full_rp_value(RegisterPair::DE);
            let mut memory_character: char;

            loop {
                memory_character = state.memory[memory_address as usize] as char;
                if memory_character == '$' {
                    break;
                } else {
                    print!("{}", memory_character);
                    memory_address += 1;
                }
            }
        }
        _ => panic!("Unexpected Register C value: {}", register_c),
    };
}

fn read_test_file(test_filename: &str) -> State {
    let pc_start = 0x0100;
    let test_path: PathBuf = ["cpu_tests", test_filename].iter().collect();
    let mut file_bytes = fs::read(test_path).expect("File not found!");

    for memory_index in (0..pc_start).rev() {
        let memory_value = match memory_index {
            0x0000 => 0xD3, // OUT
            0x0001 => 0x00, // 00
            0x0005 => 0xD3, // OUT
            0x0006 => 0x01, // 01
            0x0007 => 0xC9, // RET
            _ => 0b0000_0000,
        };

        file_bytes.insert(0, memory_value);
    }

    let mut state = StateBuilder::default().program_counter(pc_start).build();
    state.load_memory(file_bytes);
    state
}

fn init() {
    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .is_test(true)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .try_init();
}

fn assert_cpu_cycles_are_as_expected(state: &State, expected_cpu_cycles: usize) {
    let actual_cpu_cycles = state.cpu_total_state_count();
    assert_eq!(
        actual_cpu_cycles, expected_cpu_cycles,
        "Expected test to take {} cycles, but it actually took {} cycles",
        expected_cpu_cycles, actual_cpu_cycles
    );
}

fn run_cpu_test(test_filename: &str, expected_cpu_cycles: usize) {
    init();
    let mut state = read_test_file(test_filename);

    'running: loop {
        let should_quit = run_next_operation(&mut state);
        if should_quit {
            break 'running;
        }
    }

    assert_cpu_cycles_are_as_expected(&state, expected_cpu_cycles);
}

#[test]
fn cpu_test_tst8080() {
    run_cpu_test("TST8080.COM", 4924);
}

#[test]
fn cpu_test_cputest() {
    run_cpu_test("CPUTEST.COM", 255653383);
}

#[test]
fn cpu_test_8080pre() {
    run_cpu_test("8080PRE.COM", 7817);
}

#[test]
fn cpu_test_8080exm() {
    run_cpu_test("8080EXM.COM", 23803381171);
}
