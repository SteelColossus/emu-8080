use crate::{ConditionFlag, ConditionFlags, Register, RegisterState, State, MEMORY_SIZE};
use std::fmt::{Debug, Display};

fn assert_actual_equals_expected_for_type<T, V>(
    friendly_name: &str,
    type_being_checked: T,
    actual_value: V,
    expected_value: V,
) where
    T: Debug,
    V: Eq + Debug + Display,
{
    assert_eq!(
        actual_value, expected_value,
        "Expected {} {:?} to have value {}, but instead it had value {}",
        friendly_name, type_being_checked, expected_value, actual_value
    );
}

fn assert_register_has_value(register: Register, actual_value: u8, expected_value: u8) {
    assert_actual_equals_expected_for_type("register", register, actual_value, expected_value);
}

fn assert_values_of_registers(state: &State, expected_register_state: &RegisterState) {
    for (register, actual_value) in &state.registers {
        let expected_value = expected_register_state[register];
        assert_register_has_value(register, *actual_value, expected_value);
    }
}

fn assert_condition_flag_has_value(
    condition_flag: ConditionFlag,
    actual_value: bool,
    expected_value: bool,
) {
    assert_actual_equals_expected_for_type(
        "condition flag",
        condition_flag,
        actual_value,
        expected_value,
    );
}

fn assert_values_of_condition_flags(state: &State, expected_flags: &ConditionFlags) {
    for (condition_flag, actual_value) in &state.condition_flags {
        let expected_value = expected_flags[condition_flag];
        assert_condition_flag_has_value(condition_flag, *actual_value, expected_value);
    }
}

fn assert_program_counter_has_value(state: &State, expected_value: u16) {
    let actual_value = state.program_counter;
    assert_eq!(
        actual_value, expected_value,
        "Expected program counter to have value {:#06X}, but instead it had value {:#06X}",
        expected_value, actual_value
    );
}

fn assert_stack_pointer_has_value(state: &State, expected_value: u16) {
    let actual_value = state.stack_pointer;
    assert_eq!(
        actual_value, expected_value,
        "Expected stack pointer to have value {:#06X}, but instead it had value {:#06X}",
        expected_value, actual_value
    );
}

fn assert_memory_location_contains_value(state: &State, memory_address: u16, expected_value: u8) {
    let actual_value = state.memory[memory_address as usize];
    assert_eq!(
        actual_value, expected_value,
        "Expected memory location {:#06X} to have value {}, but instead it had value {}",
        memory_address, expected_value, actual_value
    );
}

fn assert_interrupts_enabled_state(state: &State, expected_interrupts_are_enabled: bool) {
    let actual_interrupts_are_enabled = state.are_interrupts_enabled;
    assert_eq!(
        actual_interrupts_are_enabled, expected_interrupts_are_enabled,
        "Expected interrupts enabled to be {}, but instead it was {}",
        expected_interrupts_are_enabled, actual_interrupts_are_enabled
    );
}

pub fn assert_state_is_as_expected(actual_state: &State, expected_state: &State) {
    assert_values_of_registers(actual_state, &expected_state.registers);
    assert_values_of_condition_flags(actual_state, &expected_state.condition_flags);
    assert_program_counter_has_value(actual_state, expected_state.program_counter);
    assert_stack_pointer_has_value(actual_state, expected_state.stack_pointer);
    assert_interrupts_enabled_state(actual_state, expected_state.are_interrupts_enabled);
    for memory_address in 0..MEMORY_SIZE {
        let expected_value = expected_state.memory[memory_address];
        assert_memory_location_contains_value(actual_state, memory_address as u16, expected_value);
    }
}
