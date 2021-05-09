use crate::{Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn inr_instruction(state: &mut State, register: Register) {
    state.increase_register(register, 1);
    let result = state.get_register_value(register);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
pub fn dcr_instruction(state: &mut State, register: Register) {
    state.decrease_register(register, 1);
    let result = state.get_register_value(register);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
pub fn add_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    adi_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn adi_instruction(state: &mut State, data: u8) {
    let carry = state.increase_register(Register::A, data);
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = carry;
}

#[cfg_attr(test, mutate)]
pub fn adc_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    aci_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn aci_instruction(state: &mut State, data: u8) {
    let carry_value = if state.condition_flags.carry { 1 } else { 0 };
    let mut carry = state.increase_register(Register::A, data);
    carry |= state.increase_register(Register::A, carry_value);
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = carry;
}

#[cfg_attr(test, mutate)]
pub fn sub_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    sui_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn sui_instruction(state: &mut State, data: u8) {
    let borrow = state.decrease_register(Register::A, data);
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = borrow;
}

#[cfg_attr(test, mutate)]
pub fn sbb_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    sbi_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn sbi_instruction(state: &mut State, data: u8) {
    let carry_value = if state.condition_flags.carry { 1 } else { 0 };
    let mut borrow = state.decrease_register(Register::A, data);
    borrow |= state.decrease_register(Register::A, carry_value);
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = borrow;
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{ConditionFlag, Register, RegisterState, State};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn inr_increments_default_register_value() {
        let mut state = State::default();
        crate::arithmetic_instructions::inr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(&state, hashmap! { Register::C => 1 }, HashMap::new());
    }

    #[test]
    fn inr_increments_existing_register_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::E => 127 });
        crate::arithmetic_instructions::inr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 128 },
            hashmap! { ConditionFlag::Sign => true },
        );
    }

    #[test]
    fn inr_does_not_set_carry_flag_when_overflowing() {
        let mut state = State::with_initial_register_state(hashmap! { Register::C => 255 });
        crate::arithmetic_instructions::inr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::C => 0 },
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn dcr_decrements_default_register_value_and_does_not_set_carry_flag_when_underflowing() {
        let mut state = State::default();
        crate::arithmetic_instructions::dcr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::C => 255 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn dcr_decrements_existing_register_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::E => 127 });
        crate::arithmetic_instructions::dcr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 126 },
            hashmap! { ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn add_adds_the_existing_register_value_to_the_accumulator() {
        let mut state = State::with_initial_register_state(hashmap! { Register::D => 24 });
        crate::arithmetic_instructions::add_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::D => 24, Register::A => 24 },
            hashmap! { ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn add_can_add_a_value_to_the_accumulator_multiple_times() {
        let mut state = State::with_initial_register_state(hashmap! { Register::B => 31 });
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
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
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::C);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::D);
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
        crate::arithmetic_instructions::add_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 95, Register::A => 197 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn add_doubles_the_accumulator_value_if_it_is_given_as_the_register() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 63 });
        crate::arithmetic_instructions::add_instruction(&mut state, Register::A);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 252 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn add_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::default();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::A);
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
        crate::arithmetic_instructions::add_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 183, Register::A => 83 },
            hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
        );
    }

    #[test]
    fn adi_adds_the_given_value_onto_the_default_accumulator_value() {
        let mut state = State::default();
        crate::arithmetic_instructions::adi_instruction(&mut state, 128);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 128 },
            hashmap! { ConditionFlag::Sign => true },
        );
    }

    #[test]
    fn adi_adds_the_given_value_onto_any_existing_value_in_the_accumulator() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 90 });
        crate::arithmetic_instructions::adi_instruction(&mut state, 37);
        assert_state_is_as_expected(&state, hashmap! { Register::A => 127 }, HashMap::new());
    }

    #[test]
    fn adi_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::default();
        crate::arithmetic_instructions::adi_instruction(&mut state, 0);
        assert_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn adi_sets_the_carry_flag_when_overflowing() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 174 });
        crate::arithmetic_instructions::adi_instruction(&mut state, 149);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 67 },
            hashmap! { ConditionFlag::Carry => true },
        );
    }

    #[test]
    fn adc_adds_the_register_value_to_accumulator_when_carry_flag_is_not_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 77, Register::B => 88 });
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 165, Register::B => 88 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn adc_adds_to_the_accumulator_the_register_value_and_carry_flag_when_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 77, Register::B => 88 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 166, Register::B => 88 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn adc_correctly_overflows_with_the_carry_flag_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 200, Register::B => 255 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 200, Register::B => 255 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
        )
    }

    #[test]
    fn adc_correctly_overflows_as_a_result_of_the_carry_flag() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 224, Register::B => 31 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0, Register::B => 31 },
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
        )
    }

    #[test]
    fn aci_adds_the_given_value_to_accumulator_when_carry_flag_is_not_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 77 });
        crate::arithmetic_instructions::aci_instruction(&mut state, 88);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 165 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn aci_adds_to_the_accumulator_the_given_value_and_carry_flag_when_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 77 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::aci_instruction(&mut state, 88);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 166 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn aci_correctly_overflows_with_the_carry_flag_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 200 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::aci_instruction(&mut state, 255);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 200 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
        )
    }

    #[test]
    fn aci_correctly_overflows_as_a_result_of_the_carry_flag() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 224 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::aci_instruction(&mut state, 31);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0 },
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
        )
    }

    #[test]
    fn sub_subtracts_the_existing_register_value_from_the_accumulator() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 255, Register::D => 24 });
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::D);
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
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
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
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::C);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::D);
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
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::A);
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
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 183, Register::A => 229 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
        );
    }

    #[test]
    fn sui_subtracts_the_given_value_from_any_existing_value_in_the_accumulator() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 183 });
        crate::arithmetic_instructions::sui_instruction(&mut state, 55);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 128 },
            hashmap! { ConditionFlag::Sign => true },
        );
    }

    #[test]
    fn sui_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 173 });
        crate::arithmetic_instructions::sui_instruction(&mut state, 173);
        assert_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn sui_sets_the_carry_flag_when_underflowing() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 149 });
        crate::arithmetic_instructions::sui_instruction(&mut state, 174);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 231 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
        );
    }

    #[test]
    fn sbb_subtracts_the_register_value_from_accumulator_when_carry_flag_is_not_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 88, Register::B => 77 });
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 11, Register::B => 77 },
            HashMap::new(),
        )
    }

    #[test]
    fn sbb_subtracts_from_the_accumulator_the_register_value_and_carry_flag_when_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 88, Register::B => 77 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 10, Register::B => 77 },
            hashmap! { ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn sbb_correctly_underflows_with_the_carry_flag_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 200, Register::B => 255 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 200, Register::B => 255 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
        )
    }

    #[test]
    fn sbb_correctly_underflows_as_a_result_of_the_carry_flag() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 31, Register::B => 31 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 255, Register::B => 31 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
        )
    }

    #[test]
    fn sbi_subtracts_the_given_value_from_accumulator_when_carry_flag_is_not_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 88 });
        crate::arithmetic_instructions::sbi_instruction(&mut state, 77);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 11 },
            HashMap::new(),
        )
    }

    #[test]
    fn sbi_subtracts_from_the_accumulator_the_given_value_and_carry_flag_when_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 88 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::sbi_instruction(&mut state, 77);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 10 },
            hashmap! { ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn sbi_correctly_underflows_with_the_carry_flag_set() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 200 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::sbi_instruction(&mut state, 255);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 200 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
        )
    }

    #[test]
    fn sbi_correctly_underflows_as_a_result_of_the_carry_flag() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 31 });
        state.condition_flags.carry = true;
        crate::arithmetic_instructions::sbi_instruction(&mut state, 31);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 255 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true },
        )
    }
}
