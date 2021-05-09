use std::collections::HashMap;
use std::fmt::{Debug, Display};

use emu_8080::{ConditionFlag, Register, RegisterState, State};
use maplit::hashmap;

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

fn assert_state_is_as_expected(
    state: &State,
    expected_register_state: RegisterState,
    expected_flags: HashMap<ConditionFlag, bool>,
) {
    assert_values_of_registers(state, expected_register_state);
    assert_values_of_condition_flags(state, expected_flags);
}

#[test]
fn can_get_state_of_all_registers() {
    let state = State::default();
    let register_state = state.get_register_state();
    assert_eq!(register_state.len(), 7);
}

#[test]
fn default_register_state_has_all_default_values() {
    let state = State::default();
    assert_state_is_as_expected(&state, RegisterState::new(), HashMap::new());
}

#[test]
fn can_create_state_with_initial_register_values() {
    let state =
        State::with_initial_register_state(hashmap! { Register::A => 23, Register::C => 34 });
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 23, Register::C => 34 },
        HashMap::new(),
    );
}

#[test]
#[should_panic(expected = "Invalid bit index of 8")]
fn is_bit_set_panics_when_given_an_invalid_bit_index() {
    let state = State::default();
    state.is_bit_set(255, 8);
}

#[test]
fn mvi_loads_data_into_one_register() {
    let mut state = State::default();
    emu_8080::transfer_instructions::mvi_instruction(&mut state, Register::A, 64);
    assert_state_is_as_expected(&state, hashmap! { Register::A => 64 }, HashMap::new());
}

#[test]
fn mvi_loads_data_into_multiple_registers() {
    let mut state = State::default();
    emu_8080::transfer_instructions::mvi_instruction(&mut state, Register::B, 128);
    emu_8080::transfer_instructions::mvi_instruction(&mut state, Register::D, 255);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::B => 128, Register::D => 255 },
        HashMap::new(),
    );
}

#[test]
fn mov_moves_value_from_one_register_to_another() {
    let mut state = State::with_initial_register_state(hashmap! { Register::H => 99 });
    emu_8080::transfer_instructions::mov_instruction(&mut state, Register::H, Register::A);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::H => 99, Register::A => 99 },
        HashMap::new(),
    );
}

#[test]
fn mov_does_nothing_when_both_registers_are_the_same() {
    let mut state = State::with_initial_register_state(hashmap! { Register::L => 121 });
    emu_8080::transfer_instructions::mov_instruction(&mut state, Register::L, Register::L);
    assert_state_is_as_expected(&state, hashmap! { Register::L => 121 }, HashMap::new());
}

#[test]
fn multiple_mov_can_move_to_multiple_registers() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 218 });
    emu_8080::transfer_instructions::mov_instruction(&mut state, Register::A, Register::C);
    emu_8080::transfer_instructions::mov_instruction(&mut state, Register::A, Register::E);
    emu_8080::transfer_instructions::mov_instruction(&mut state, Register::A, Register::L);
    assert_state_is_as_expected(
        &state,
        hashmap! {
            Register::A => 218,
            Register::C => 218,
            Register::E => 218,
            Register::L => 218,
        },
        HashMap::new(),
    );
}

#[test]
fn xchg_exchanges_content_of_registers() {
    let mut state = State::with_initial_register_state(hashmap! {
        Register::D => 205,
        Register::E => 69,
        Register::L => 11,
    });
    emu_8080::transfer_instructions::xchg_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        hashmap! {
            Register::H => 205,
            Register::E => 11,
            Register::L => 69,
        },
        HashMap::new(),
    );
}

#[test]
fn inr_increments_default_register_value() {
    let mut state = State::default();
    emu_8080::arithmetic_instructions::inr_instruction(&mut state, Register::C);
    assert_state_is_as_expected(&state, hashmap! { Register::C => 1 }, HashMap::new());
}

#[test]
fn inr_increments_existing_register_value() {
    let mut state = State::with_initial_register_state(hashmap! { Register::E => 127 });
    emu_8080::arithmetic_instructions::inr_instruction(&mut state, Register::E);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::E => 128 },
        hashmap! { ConditionFlag::Sign => true },
    );
}

#[test]
fn inr_does_not_set_carry_flag_when_overflowing() {
    let mut state = State::with_initial_register_state(hashmap! { Register::C => 255 });
    emu_8080::arithmetic_instructions::inr_instruction(&mut state, Register::C);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::C => 0 },
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn dcr_decrements_default_register_value_and_does_not_set_carry_flag_when_underflowing() {
    let mut state = State::default();
    emu_8080::arithmetic_instructions::dcr_instruction(&mut state, Register::C);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::C => 255 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn dcr_decrements_existing_register_value() {
    let mut state = State::with_initial_register_state(hashmap! { Register::E => 127 });
    emu_8080::arithmetic_instructions::dcr_instruction(&mut state, Register::E);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::E => 126 },
        hashmap! { ConditionFlag::Parity => true },
    );
}

#[test]
fn add_adds_the_existing_register_value_to_the_accumulator() {
    let mut state = State::with_initial_register_state(hashmap! { Register::D => 24 });
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::D);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::D => 24, Register::A => 24 },
        hashmap! { ConditionFlag::Parity => true },
    )
}

#[test]
fn add_can_add_a_value_to_the_accumulator_multiple_times() {
    let mut state = State::with_initial_register_state(hashmap! { Register::B => 31 });
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::B);
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::B);
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::B);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::B => 31, Register::A => 93 },
        HashMap::new(),
    )
}

#[test]
fn add_can_add_values_from_multiple_different_registers() {
    let mut state = State::with_initial_register_state(hashmap! {
        Register::B => 11,
        Register::C => 13,
        Register::D => 15,
    });
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::B);
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::C);
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::D);
    assert_state_is_as_expected(
        &state,
        hashmap! {
            Register::B => 11,
            Register::C => 13,
            Register::D => 15,
            Register::A => 39,
        },
        hashmap! { ConditionFlag::Parity => true },
    )
}

#[test]
fn add_adds_the_value_onto_any_existing_value_in_the_accumulator() {
    let mut state =
        State::with_initial_register_state(hashmap! { Register::A => 102, Register::E => 95 });
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::E);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::E => 95, Register::A => 197 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn add_doubles_the_accumulator_value_if_it_is_given_as_the_register() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 63 });
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::A);
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 252 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn add_sets_condition_flag_zero_to_true_if_result_is_zero() {
    let mut state = State::default();
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn add_sets_the_carry_flag_when_overflowing() {
    let mut state =
        State::with_initial_register_state(hashmap! { Register::A => 156, Register::E => 183 });
    emu_8080::arithmetic_instructions::add_instruction(&mut state, Register::E);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::E => 183, Register::A => 83 },
        hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
    );
}

#[test]
fn adi_adds_the_given_value_onto_the_default_accumulator_value() {
    let mut state = State::default();
    emu_8080::arithmetic_instructions::adi_instruction(&mut state, 128);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 128 },
        hashmap! { ConditionFlag::Sign => true },
    );
}

#[test]
fn adi_adds_the_given_value_onto_any_existing_value_in_the_accumulator() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 90 });
    emu_8080::arithmetic_instructions::adi_instruction(&mut state, 37);
    assert_state_is_as_expected(&state, hashmap! { Register::A => 127 }, HashMap::new());
}

#[test]
fn adi_sets_condition_flag_zero_to_true_if_result_is_zero() {
    let mut state = State::default();
    emu_8080::arithmetic_instructions::adi_instruction(&mut state, 0);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn adi_sets_the_carry_flag_when_overflowing() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 174 });
    emu_8080::arithmetic_instructions::adi_instruction(&mut state, 149);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 67 },
        hashmap! { ConditionFlag::Carry => true },
    );
}

#[test]
fn sub_subtracts_the_existing_register_value_from_the_accumulator() {
    let mut state =
        State::with_initial_register_state(hashmap! { Register::A => 255, Register::D => 24 });
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::D);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::D => 24, Register::A => 231 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    )
}

#[test]
fn sub_can_subtract_a_value_from_the_accumulator_multiple_times() {
    let mut state =
        State::with_initial_register_state(hashmap! { Register::A => 253, Register::B => 42 });
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::B);
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::B);
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::B);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::B => 42, Register::A => 127 },
        HashMap::new(),
    )
}

#[test]
fn sub_can_subtract_values_from_multiple_different_registers() {
    let mut state = State::with_initial_register_state(hashmap! {
        Register::A => 255,
        Register::B => 11,
        Register::C => 13,
        Register::D => 15,
    });
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::B);
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::C);
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::D);
    assert_state_is_as_expected(
        &state,
        hashmap! {
            Register::B => 11,
            Register::C => 13,
            Register::D => 15,
            Register::A => 216,
        },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    )
}

#[test]
fn sub_zeroes_the_accumulator_value_if_it_is_given_as_the_register() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 63 });
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0 },
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn sub_sets_the_carry_flag_when_underflowing() {
    let mut state =
        State::with_initial_register_state(hashmap! { Register::A => 156, Register::E => 183 });
    emu_8080::arithmetic_instructions::sub_instruction(&mut state, Register::E);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::E => 183, Register::A => 229 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
    );
}

#[test]
fn sui_subtracts_the_given_value_from_any_existing_value_in_the_accumulator() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 183 });
    emu_8080::arithmetic_instructions::sui_instruction(&mut state, 55);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 128 },
        hashmap! { ConditionFlag::Sign => true },
    );
}

#[test]
fn sui_sets_condition_flag_zero_to_true_if_result_is_zero() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 173 });
    emu_8080::arithmetic_instructions::sui_instruction(&mut state, 173);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn sui_sets_the_carry_flag_when_underflowing() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 149 });
    emu_8080::arithmetic_instructions::sui_instruction(&mut state, 174);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 231 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
    );
}

#[test]
fn ana_logically_ands_the_accumulator_with_the_value_of_the_given_register() {
    let mut state = State::with_initial_register_state(
        hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 },
    );
    emu_8080::logical_instructions::ana_instruction(&mut state, Register::B);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::B => 0b01000101, Register::A => 0b01000100 },
        hashmap! { ConditionFlag::Parity => true },
    );
}

#[test]
fn ana_applied_to_an_existing_accumulator_value_does_nothing() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b10100110 });
    emu_8080::logical_instructions::ana_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b10100110 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn ana_clears_the_carry_flag() {
    let mut state = State::default();
    state.condition_flags.carry = true;
    emu_8080::logical_instructions::ana_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn ani_logically_ands_the_accumulator_with_the_given_value() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
    emu_8080::logical_instructions::ani_instruction(&mut state, 0b01100011);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b01000010 },
        hashmap! { ConditionFlag::Parity => true },
    );
}

#[test]
fn ani_clears_the_carry_flag() {
    let mut state = State::default();
    state.condition_flags.carry = true;
    emu_8080::logical_instructions::ani_instruction(&mut state, 0b00000000);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn ora_logically_ors_the_accumulator_with_the_value_of_the_given_register() {
    let mut state = State::with_initial_register_state(
        hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 },
    );
    emu_8080::logical_instructions::ora_instruction(&mut state, Register::B);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::B => 0b01000101, Register::A => 0b01010101 },
        hashmap! { ConditionFlag::Parity => true },
    );
}

#[test]
fn ora_applied_to_an_existing_accumulator_value_does_nothing() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b10100110 });
    emu_8080::logical_instructions::ora_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b10100110 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn ora_clears_the_carry_flag() {
    let mut state = State::default();
    state.condition_flags.carry = true;
    emu_8080::logical_instructions::ora_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn oni_logically_ors_the_accumulator_with_the_given_value() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
    emu_8080::logical_instructions::oni_instruction(&mut state, 0b01100011);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b11100111 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn oni_clears_the_carry_flag() {
    let mut state = State::default();
    state.condition_flags.carry = true;
    emu_8080::logical_instructions::oni_instruction(&mut state, 0b00000000);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn xra_logically_xors_the_accumulator_with_the_value_of_the_given_register() {
    let mut state = State::with_initial_register_state(
        hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 },
    );
    emu_8080::logical_instructions::xra_instruction(&mut state, Register::B);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::B => 0b01000101, Register::A => 0b00010001 },
        hashmap! { ConditionFlag::Parity => true },
    );
}

#[test]
fn xra_applied_to_an_existing_accumulator_value_zeroes_that_value() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b10100110 });
    emu_8080::logical_instructions::xra_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b00000000 },
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn xra_clears_the_carry_flag() {
    let mut state = State::default();
    state.condition_flags.carry = true;
    emu_8080::logical_instructions::xra_instruction(&mut state, Register::A);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn xni_logically_xors_the_accumulator_with_the_given_value() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
    emu_8080::logical_instructions::xni_instruction(&mut state, 0b01100011);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b10100101 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn xni_clears_the_carry_flag() {
    let mut state = State::default();
    state.condition_flags.carry = true;
    emu_8080::logical_instructions::xni_instruction(&mut state, 0b00000000);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn cmp_sets_the_zero_flag_if_both_are_same() {
    let mut state =
        State::with_initial_register_state(hashmap! { Register::A => 36, Register::H => 36 });
    emu_8080::logical_instructions::cmp_instruction(&mut state, Register::H);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 36, Register::H => 36 },
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn cmp_sets_the_carry_flag_if_register_value_is_greater() {
    let mut state =
        State::with_initial_register_state(hashmap! { Register::A => 24, Register::E => 48 });
    emu_8080::logical_instructions::cmp_instruction(&mut state, Register::E);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 24, Register::E => 48 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
    );
}

#[test]
fn cpi_sets_the_zero_flag_if_both_are_same() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 54 });
    emu_8080::logical_instructions::cpi_instruction(&mut state, 54);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
    );
}

#[test]
fn cpi_sets_the_carry_flag_if_register_value_is_greater() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 36 });
    emu_8080::logical_instructions::cpi_instruction(&mut state, 60);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 232 },
        hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
    );
}

#[test]
fn rlc_shifts_the_accumulator_value_left() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b01100011 });
    emu_8080::logical_instructions::rlc_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b11000110 },
        HashMap::new(),
    );
}

#[test]
fn rlc_wraps_shifted_bit_and_sets_carry_flag() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b10000000 });
    emu_8080::logical_instructions::rlc_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b00000001 },
        hashmap! { ConditionFlag::Carry => true },
    );
}

#[test]
fn rrc_shifts_the_accumulator_value_left() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
    emu_8080::logical_instructions::rrc_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b01100011 },
        HashMap::new(),
    );
}

#[test]
fn rrc_wraps_shifted_bit_and_sets_carry_flag() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b00000001 });
    emu_8080::logical_instructions::rrc_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b10000000 },
        hashmap! { ConditionFlag::Carry => true },
    );
}

#[test]
fn cma_complements_the_value_in_the_accumulator() {
    let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
    emu_8080::logical_instructions::cma_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        hashmap! { Register::A => 0b00111001 },
        HashMap::new(),
    );
}

#[test]
fn cmc_inverts_the_carry_flag() {
    let mut state = State::default();
    emu_8080::logical_instructions::cmc_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Carry => true },
    );
    emu_8080::logical_instructions::cmc_instruction(&mut state);
    assert_state_is_as_expected(&state, RegisterState::new(), HashMap::new());
}

#[test]
fn stc_sets_the_carry_flag_to_true() {
    let mut state = State::default();
    emu_8080::logical_instructions::stc_instruction(&mut state);
    assert_state_is_as_expected(
        &state,
        RegisterState::new(),
        hashmap! { ConditionFlag::Carry => true },
    );
}
