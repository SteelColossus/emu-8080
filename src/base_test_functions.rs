use crate::{ConditionFlag, Register, RegisterState, State};
use std::collections::HashMap;
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

fn assert_register_has_value(register: Register, actual_value: i8, expected_value: i8) {
    assert_actual_equals_expected_for_type("register", register, actual_value, expected_value);
}

fn assert_values_of_registers(state: &State, expected_register_state: RegisterState) {
    let register_state = state.get_register_state();
    let default_value = 0;

    for (register, actual_value) in register_state {
        match expected_register_state.get(&register) {
            Some(expected_value) => {
                assert_register_has_value(*register, *actual_value, *expected_value)
            }
            None => assert_register_has_value(*register, *actual_value, default_value),
        }
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

fn assert_values_of_condition_flags(state: &State, expected_flags: HashMap<ConditionFlag, bool>) {
    let condition_flags_state = state.get_condition_flag_state();
    let default_value = false;

    for (condition_flag, actual_value) in condition_flags_state {
        match expected_flags.get(&condition_flag) {
            Some(expected_value) => {
                assert_condition_flag_has_value(condition_flag, actual_value, *expected_value)
            }
            None => assert_condition_flag_has_value(condition_flag, actual_value, default_value),
        }
    }
}

fn assert_program_counter_has_value(state: &State, expected_value: u16) {
    let actual_value = state.program_counter;
    assert_eq!(
        actual_value, expected_value,
        "Expected program counter to have value {:#X}, but instead it had value {:#X}",
        expected_value, actual_value
    );
}

pub fn assert_memory_location_contains_value(
    state: &State,
    memory_address: u16,
    expected_value: u8,
) {
    let actual_value = state.get_value_at_memory_location(memory_address);
    assert_eq!(
        actual_value, expected_value,
        "Expected memory location to have value {}, but instead it had value {}",
        expected_value, actual_value
    );
}

pub fn assert_full_state_is_as_expected(
    state: &State,
    expected_register_state: RegisterState,
    expected_flags: HashMap<ConditionFlag, bool>,
    expected_program_counter: u16,
) {
    assert_values_of_registers(state, expected_register_state);
    assert_values_of_condition_flags(state, expected_flags);
    assert_program_counter_has_value(state, expected_program_counter);
}

pub fn assert_state_is_as_expected(
    state: &State,
    expected_register_state: RegisterState,
    expected_flags: HashMap<ConditionFlag, bool>,
) {
    assert_full_state_is_as_expected(state, expected_register_state, expected_flags, 0x0000);
}
