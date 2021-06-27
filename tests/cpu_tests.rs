use emu_8080::{disassembler, Operation, Register, RegisterPair, State, StateBuilder};
use std::fs;
use std::io::Write;

fn run_next_operation(state: &mut State) -> bool {
    let memory_value = state.get_memory_value_at_program_counter();
    let operation = disassembler::disassemble_op_code(memory_value);
    let is_out_operation = operation == Operation::Out;

    state.run_operation(operation);

    if is_out_operation {
        let port_number = state.get_value_at_memory_location(state.program_counter - 1);
        match port_number {
            0 => return true,
            1 => print_test_output(state),
            _ => {}
        };
    }

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
    };
}

fn read_test_file(test_path: &str) -> State {
    let pc_start = 0x0100;
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
    let actual_cpu_cycles = state.get_cpu_total_state_count();
    assert_eq!(
        actual_cpu_cycles, expected_cpu_cycles,
        "Expected test to take {} cycles, but it actually took {} cycles",
        expected_cpu_cycles, actual_cpu_cycles
    );
}

#[test]
fn cpu_test_tst8080() {
    init();
    let mut state = read_test_file("cpu_tests/TST8080.COM");

    'running: loop {
        let should_quit = run_next_operation(&mut state);
        if should_quit {
            break 'running;
        }
    }

    assert_cpu_cycles_are_as_expected(&state, 4924);
}

#[test]
fn cpu_test_cputest() {
    init();
    let mut state = read_test_file("cpu_tests/CPUTEST.COM");

    'running: loop {
        let should_quit = run_next_operation(&mut state);
        if should_quit {
            break 'running;
        }
    }

    assert_cpu_cycles_are_as_expected(&state, 255653383);
}

#[test]
fn cpu_test_8080pre() {
    init();
    let mut state = read_test_file("cpu_tests/8080PRE.COM");

    'running: loop {
        let should_quit = run_next_operation(&mut state);
        if should_quit {
            break 'running;
        }
    }

    assert_cpu_cycles_are_as_expected(&state, 7817);
}

#[test]
fn cpu_test_8080exm() {
    init();
    let mut state = read_test_file("cpu_tests/8080EXM.COM");

    'running: loop {
        let should_quit = run_next_operation(&mut state);
        if should_quit {
            break 'running;
        }
    }

    assert_cpu_cycles_are_as_expected(&state, 23803381171);
}
