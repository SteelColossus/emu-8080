use crate::{Register, RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn inr_instruction(state: &mut State, register: Register) {
    state.increase_register(register, 1);
    state.set_condition_flags_from_register_value(register);
}

#[cfg_attr(test, mutate)]
pub fn dcr_instruction(state: &mut State, register: Register) {
    state.decrease_register(register, 1);
    state.set_condition_flags_from_register_value(register);
}

#[cfg_attr(test, mutate)]
pub fn inx_instruction(state: &mut State, register_pair: RegisterPair) {
    let mut value = register_pair.get_full_value(state);
    value = value.wrapping_add(1);
    register_pair.set_full_value(state, value);
}

#[cfg_attr(test, mutate)]
pub fn dcx_instruction(state: &mut State, register_pair: RegisterPair) {
    let mut value = register_pair.get_full_value(state);
    value = value.wrapping_sub(1);
    register_pair.set_full_value(state, value);
}

#[cfg_attr(test, mutate)]
pub fn dad_instruction(state: &mut State, register_pair: RegisterPair) {
    let hl_value = RegisterPair::HL.get_full_value(state);
    let register_pair_value = register_pair.get_full_value(state);
    let (hl_value_after_addition, carry) = hl_value.overflowing_add(register_pair_value);
    RegisterPair::HL.set_full_value(state, hl_value_after_addition);
    state.condition_flags.carry = carry;
}

#[cfg_attr(test, mutate)]
pub fn add_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    adi_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn adi_instruction(state: &mut State, data: i8) {
    let carry = state.increase_register(Register::A, data);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags.carry = carry;
}

#[cfg_attr(test, mutate)]
pub fn adc_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    aci_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn aci_instruction(state: &mut State, data: i8) {
    let carry_value = if state.condition_flags.carry { 1 } else { 0 };
    let mut carry = state.increase_register(Register::A, data);
    carry |= state.increase_register(Register::A, carry_value);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags.carry = carry;
}

#[cfg_attr(test, mutate)]
pub fn sub_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    sui_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn sui_instruction(state: &mut State, data: i8) {
    let borrow = state.decrease_register(Register::A, data);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags.carry = borrow;
}

#[cfg_attr(test, mutate)]
pub fn sbb_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    sbi_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn sbi_instruction(state: &mut State, data: i8) {
    let carry_value = if state.condition_flags.carry { 1 } else { 0 };
    let mut borrow = state.decrease_register(Register::A, data);
    borrow |= state.decrease_register(Register::A, carry_value);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags.carry = borrow;
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{ConditionFlag, Register, RegisterPair, State, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn inr_increments_default_register_value() {
        let mut state = State::default();
        crate::arithmetic_instructions::inr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::C => 1 })
                .build(),
        );
    }

    #[test]
    fn inr_increments_existing_register_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::E => 63 })
            .build();
        crate::arithmetic_instructions::inr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 64 })
                .build(),
        );
    }

    #[test]
    fn inr_does_not_set_carry_flag_when_overflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::C => 127 })
            .build();
        crate::arithmetic_instructions::inr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::C => -128 })
                .condition_flag_values(hashmap! { ConditionFlag::Sign => true })
                .build(),
        );
    }

    #[test]
    fn dcr_decrements_default_register_value() {
        let mut state = State::default();
        crate::arithmetic_instructions::dcr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::C => -1 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn dcr_decrements_existing_register_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::E => 127 })
            .build();
        crate::arithmetic_instructions::dcr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 126 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    fn dcr_does_not_set_carry_flag_when_underflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::C => -128 })
            .build();
        crate::arithmetic_instructions::dcr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::C => 127 })
                .build(),
        );
    }

    #[test]
    fn inx_increases_the_register_pair_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 47, Register::E => -115 })
            .build();
        crate::arithmetic_instructions::inx_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 47, Register::E => -114 })
                .build(),
        );
    }

    #[test]
    fn inx_can_overflow_into_higher_register_of_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => -52, Register::L => -1 })
            .build();
        crate::arithmetic_instructions::inx_instruction(&mut state, RegisterPair::HL);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -51, Register::L => 0 })
                .build(),
        );
    }

    #[test]
    fn dcx_decreases_the_register_pair_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 81, Register::L => -122 })
            .build();
        crate::arithmetic_instructions::dcx_instruction(&mut state, RegisterPair::HL);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 81, Register::L => -123 })
                .build(),
        );
    }

    #[test]
    fn dcx_can_underflow_into_higher_register_of_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => -107, Register::E => 0 })
            .build();
        crate::arithmetic_instructions::dcx_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => -108, Register::E => -1 })
                .build(),
        );
    }

    #[test]
    fn dad_adds_the_given_register_pair_value_onto_existing_register_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => 117, Register::C => -88, Register::H => 75, Register::L => 43 })
            .build();
        crate::arithmetic_instructions::dad_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 117, Register::C => -88, Register::H => -64, Register::L => -45 })
                .build(),
        );
    }

    #[test]
    fn dad_doubles_the_existing_register_pair_value_if_given() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 32, Register::L => -81 })
            .build();
        crate::arithmetic_instructions::dad_instruction(&mut state, RegisterPair::HL);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 65, Register::L => 94 })
                .build(),
        );
    }

    #[test]
    fn dad_adds_the_stack_pointer_value_if_given() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => -96, Register::L => 6 })
            .stack_pointer(0x13FE)
            .build();
        crate::arithmetic_instructions::dad_instruction(&mut state, RegisterPair::SP);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -76, Register::L => 4 })
                .stack_pointer(0x13FE)
                .build(),
        );
    }

    #[test]
    fn dad_sets_the_carry_flag_when_overflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => -110, Register::E => -48, Register::H => 109, Register::L => 48 })
            .build();
        crate::arithmetic_instructions::dad_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => -110, Register::E => -48 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn add_adds_the_existing_register_value_to_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 24 })
            .build();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 24, Register::A => 24 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        )
    }

    #[test]
    fn add_can_add_a_value_to_the_accumulator_multiple_times() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => 31 })
            .build();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 31, Register::A => 93 })
                .build(),
        )
    }

    #[test]
    fn add_can_add_values_from_multiple_different_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! {
                Register::B => 11,
                Register::C => 13,
                Register::D => 15,
            })
            .build();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::C);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! {
                    Register::B => 11,
                    Register::C => 13,
                    Register::D => 15,
                    Register::A => 39,
                })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        )
    }

    #[test]
    fn add_adds_the_value_onto_any_existing_value_in_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -102, Register::E => 95 })
            .build();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 95, Register::A => -7 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn add_doubles_the_accumulator_value_if_it_is_given_as_the_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 31 })
            .build();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::A);
        crate::arithmetic_instructions::add_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 124 })
                .build(),
        );
    }

    #[test]
    fn add_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::default();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn add_sets_the_carry_flag_when_overflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 78, Register::E => 91 })
            .build();
        crate::arithmetic_instructions::add_instruction(&mut state, Register::E);
        assert_state_is_as_expected(&state, &StateBuilder::default().register_values(hashmap! { Register::E => 91, Register::A => -87 }).condition_flag_values(hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true }).build());
    }

    #[test]
    fn adi_adds_the_given_value_onto_the_default_accumulator_value() {
        let mut state = State::default();
        crate::arithmetic_instructions::adi_instruction(&mut state, 64);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 64 })
                .build(),
        );
    }

    #[test]
    fn adi_adds_the_given_value_onto_any_existing_value_in_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 90 })
            .build();
        crate::arithmetic_instructions::adi_instruction(&mut state, 37);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 127 })
                .build(),
        );
    }

    #[test]
    fn adi_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::default();
        crate::arithmetic_instructions::adi_instruction(&mut state, 0);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn adi_sets_the_carry_flag_when_overflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 87 })
            .build();
        crate::arithmetic_instructions::adi_instruction(&mut state, 74);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => -95 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
                )
                .build(),
        );
    }

    #[test]
    fn adc_adds_the_register_value_to_accumulator_when_carry_flag_is_not_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 38, Register::B => 44 })
            .build();
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 82, Register::B => 44 })
                .build(),
        )
    }

    #[test]
    fn adc_adds_to_the_accumulator_the_register_value_and_carry_flag_when_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 38, Register::B => 44 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 83, Register::B => 44 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        )
    }

    #[test]
    fn adc_correctly_overflows_with_the_carry_flag_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 100, Register::B => 127 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(&state, &StateBuilder::default().register_values(hashmap! { Register::A => -28, Register::B => 127 }).condition_flag_values(hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true }).build())
    }

    #[test]
    fn adc_correctly_overflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 96, Register::B => 31 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => -128, Register::B => 31 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
                )
                .build(),
        )
    }

    #[test]
    fn aci_adds_the_given_value_to_accumulator_when_carry_flag_is_not_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 38 })
            .build();
        crate::arithmetic_instructions::aci_instruction(&mut state, 44);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 82 })
                .build(),
        )
    }

    #[test]
    fn aci_adds_to_the_accumulator_the_given_value_and_carry_flag_when_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 38 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::aci_instruction(&mut state, 44);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 83 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        )
    }

    #[test]
    fn aci_correctly_overflows_with_the_carry_flag_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 100 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::aci_instruction(&mut state, 127);
        assert_state_is_as_expected(&state, &StateBuilder::default().register_values(hashmap! { Register::A => -28 }).condition_flag_values(hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true }).build())
    }

    #[test]
    fn aci_correctly_overflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 96 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::aci_instruction(&mut state, 31);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => -128 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Carry => true },
                )
                .build(),
        )
    }

    #[test]
    fn sub_subtracts_the_existing_register_value_from_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 127, Register::D => 24 })
            .build();
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 24, Register::A => 103 })
                .build(),
        )
    }

    #[test]
    fn sub_can_subtract_a_value_from_the_accumulator_multiple_times() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 127, Register::B => 42 })
            .build();
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 42, Register::A => 1 })
                .build(),
        )
    }

    #[test]
    fn sub_can_subtract_values_from_multiple_different_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! {
                Register::A => 127,
                Register::B => 11,
                Register::C => 13,
                Register::D => 15,
            })
            .build();
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::B);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::C);
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! {
                    Register::B => 11,
                    Register::C => 13,
                    Register::D => 15,
                    Register::A => 88,
                })
                .build(),
        )
    }

    #[test]
    fn sub_zeroes_the_accumulator_value_if_it_is_given_as_the_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 63 })
            .build();
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn sub_sets_the_carry_flag_when_underflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -74, Register::E => 87 })
            .build();
        crate::arithmetic_instructions::sub_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 87, Register::A => 95 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
                )
                .build(),
        );
    }

    #[test]
    fn sui_subtracts_the_given_value_from_any_existing_value_in_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 66 })
            .build();
        crate::arithmetic_instructions::sui_instruction(&mut state, 55);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 11 })
                .build(),
        );
    }

    #[test]
    fn sui_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 75 })
            .build();
        crate::arithmetic_instructions::sui_instruction(&mut state, 75);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn sui_sets_the_carry_flag_when_underflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -74 })
            .build();
        crate::arithmetic_instructions::sui_instruction(&mut state, 87);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 95 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
                )
                .build(),
        );
    }

    #[test]
    fn sbb_subtracts_the_register_value_from_accumulator_when_carry_flag_is_not_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 88, Register::B => 77 })
            .build();
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 11, Register::B => 77 })
                .build(),
        )
    }

    #[test]
    fn sbb_subtracts_from_the_accumulator_the_register_value_and_carry_flag_when_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 88, Register::B => 77 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 10, Register::B => 77 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        )
    }

    #[test]
    fn sbb_correctly_underflows_with_the_carry_flag_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -100, Register::B => 127 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 28, Register::B => 127 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        )
    }

    #[test]
    fn sbb_correctly_underflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -97, Register::B => 31 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 127, Register::B => 31 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        )
    }

    #[test]
    fn sbi_subtracts_the_given_value_from_accumulator_when_carry_flag_is_not_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 88 })
            .build();
        crate::arithmetic_instructions::sbi_instruction(&mut state, 77);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 11 })
                .build(),
        )
    }

    #[test]
    fn sbi_subtracts_from_the_accumulator_the_given_value_and_carry_flag_when_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 88 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::sbi_instruction(&mut state, 77);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 10 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        )
    }

    #[test]
    fn sbi_correctly_underflows_with_the_carry_flag_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -100 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::sbi_instruction(&mut state, 127);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 28 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        )
    }

    #[test]
    fn sbi_correctly_underflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -97 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::arithmetic_instructions::sbi_instruction(&mut state, 31);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 127 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        )
    }
}
