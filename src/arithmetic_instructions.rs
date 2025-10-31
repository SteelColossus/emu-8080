use crate::{bit_operations, ConditionFlag, Register, RegisterPair, State};
// #[cfg(test)]
// use mutagen::mutate;

// #[cfg_attr(test, mutate)]
pub fn add_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.registers[source_register];
    adi_instruction(state, source_register_value);
}

// #[cfg_attr(test, mutate)]
pub fn add_mem_instruction(state: &mut State) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    let memory_value = state.memory[memory_address as usize];
    adi_instruction(state, memory_value);
}

// #[cfg_attr(test, mutate)]
pub fn adi_instruction(state: &mut State, data: u8) {
    let (carry, auxiliary_carry) = state.increase_register(Register::A, data);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags[ConditionFlag::Carry] = carry;
    state.condition_flags[ConditionFlag::AuxiliaryCarry] = auxiliary_carry;
}

// #[cfg_attr(test, mutate)]
pub fn adc_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.registers[source_register];
    aci_instruction(state, source_register_value);
}

// #[cfg_attr(test, mutate)]
pub fn adc_mem_instruction(state: &mut State) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    let memory_value = state.memory[memory_address as usize];
    aci_instruction(state, memory_value);
}

// #[cfg_attr(test, mutate)]
pub fn aci_instruction(state: &mut State, data: u8) {
    let carry_value = state.condition_flags[ConditionFlag::Carry] as u8;
    let (main_carry, main_auxiliary_carry) = state.increase_register(Register::A, data);
    let (carry_from_carry, auxiliary_carry_from_carry) =
        state.increase_register(Register::A, carry_value);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags[ConditionFlag::Carry] = main_carry || carry_from_carry;
    state.condition_flags[ConditionFlag::AuxiliaryCarry] =
        main_auxiliary_carry || auxiliary_carry_from_carry;
}

// #[cfg_attr(test, mutate)]
pub fn sub_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.registers[source_register];
    sui_instruction(state, source_register_value);
}

// #[cfg_attr(test, mutate)]
pub fn sub_mem_instruction(state: &mut State) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    let memory_value = state.memory[memory_address as usize];
    sui_instruction(state, memory_value);
}

// #[cfg_attr(test, mutate)]
pub fn sui_instruction(state: &mut State, data: u8) {
    let (borrow, auxiliary_borrow) = state.decrease_register(Register::A, data);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags[ConditionFlag::Carry] = borrow;
    state.condition_flags[ConditionFlag::AuxiliaryCarry] = auxiliary_borrow;
}

// #[cfg_attr(test, mutate)]
pub fn sbb_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.registers[source_register];
    sbi_instruction(state, source_register_value);
}

// #[cfg_attr(test, mutate)]
pub fn sbb_mem_instruction(state: &mut State) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    let memory_value = state.memory[memory_address as usize];
    sbi_instruction(state, memory_value);
}

// #[cfg_attr(test, mutate)]
pub fn sbi_instruction(state: &mut State, data: u8) {
    let carry_value = state.condition_flags[ConditionFlag::Carry] as u8;
    let (main_borrow, main_auxiliary_borrow) = state.decrease_register(Register::A, data);
    let (borrow_from_borrow, auxiliary_borrow_from_borrow) =
        state.decrease_register(Register::A, carry_value);
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags[ConditionFlag::Carry] = main_borrow || borrow_from_borrow;
    // This was found through trial and error so may be incorrect, but in a way makes sense -
    // instead of an 'or' we use an 'and', to account for the auxiliary carry being the opposite of what it should be.
    state.condition_flags[ConditionFlag::AuxiliaryCarry] =
        main_auxiliary_borrow && auxiliary_borrow_from_borrow;
}

// #[cfg_attr(test, mutate)]
pub fn inr_instruction(state: &mut State, register: Register) {
    let (_, auxiliary_carry) = state.increase_register(register, 1);
    state.set_condition_flags_from_register_value(register);
    state.condition_flags[ConditionFlag::AuxiliaryCarry] = auxiliary_carry;
}

// #[cfg_attr(test, mutate)]
pub fn inr_mem_instruction(state: &mut State) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    let memory_value = state.memory[memory_address as usize];
    let new_memory_value = memory_value.wrapping_add(1);
    state.memory[memory_address as usize] = new_memory_value;
    state.set_condition_flags_from_result(new_memory_value);
    state.condition_flags[ConditionFlag::AuxiliaryCarry] =
        bit_operations::calculate_auxiliary_carry(memory_value, 1, false);
}

// #[cfg_attr(test, mutate)]
pub fn dcr_instruction(state: &mut State, register: Register) {
    let (_, auxiliary_borrow) = state.decrease_register(register, 1);
    state.set_condition_flags_from_register_value(register);
    state.condition_flags[ConditionFlag::AuxiliaryCarry] = auxiliary_borrow;
}

// #[cfg_attr(test, mutate)]
pub fn dcr_mem_instruction(state: &mut State) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    let memory_value = state.memory[memory_address as usize];
    let new_memory_value = memory_value.wrapping_sub(1);
    state.memory[memory_address as usize] = new_memory_value;
    state.set_condition_flags_from_result(new_memory_value);
    state.condition_flags[ConditionFlag::AuxiliaryCarry] =
        bit_operations::calculate_auxiliary_carry(memory_value, 1, true);
}

// #[cfg_attr(test, mutate)]
pub fn inx_instruction(state: &mut State, register_pair: RegisterPair) {
    let mut value = state.full_rp_value(register_pair);
    value = value.wrapping_add(1);
    state.set_full_rp_value(register_pair, value);
}

// #[cfg_attr(test, mutate)]
pub fn dcx_instruction(state: &mut State, register_pair: RegisterPair) {
    let mut value = state.full_rp_value(register_pair);
    value = value.wrapping_sub(1);
    state.set_full_rp_value(register_pair, value);
}

// #[cfg_attr(test, mutate)]
pub fn dad_instruction(state: &mut State, register_pair: RegisterPair) {
    let hl_value = state.full_rp_value(RegisterPair::HL);
    let register_pair_value = state.full_rp_value(register_pair);
    let (hl_value_after_addition, carry) = hl_value.overflowing_add(register_pair_value);
    state.set_full_rp_value(RegisterPair::HL, hl_value_after_addition);
    state.condition_flags[ConditionFlag::Carry] = carry;
}

// #[cfg_attr(test, mutate)]
pub fn daa_instruction(state: &mut State) {
    let mut result = state.registers[Register::A];
    let mut carry = false;
    let mut auxiliary_carry = false;
    let lower_nibble = result & 0b0000_1111;

    if lower_nibble > 9 || state.condition_flags[ConditionFlag::AuxiliaryCarry] {
        const LOWER_ADDITION: u8 = 6;
        let (result_from_op, carry_from_op) = result.overflowing_add(LOWER_ADDITION);
        carry = carry_from_op;
        auxiliary_carry |= bit_operations::calculate_auxiliary_carry(result, LOWER_ADDITION, false);
        result = result_from_op;
    }

    let higher_nibble = result >> 4;

    // DAA seems to add to the higher nibble even if the lower nibble resulted in a carry,
    // which would otherwise cause the higher nibble to now not be greater than 9
    if higher_nibble > 9 || state.condition_flags[ConditionFlag::Carry] || carry {
        const HIGHER_ADDITION: u8 = 6 << 4;
        let result_from_op = result.wrapping_add(HIGHER_ADDITION);
        // DAA does not reset the carry flag if it is already set, even if the result does not require a carry
        carry = true;
        auxiliary_carry |=
            bit_operations::calculate_auxiliary_carry(result, HIGHER_ADDITION, false);
        result = result_from_op;
    }

    state.registers[Register::A] = result;
    state.set_condition_flags_from_result(result);
    state.condition_flags[ConditionFlag::Carry] = carry;
    state.condition_flags[ConditionFlag::AuxiliaryCarry] = auxiliary_carry;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{ConditionFlag, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn add_adds_the_existing_register_value_to_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 24 })
            .build();
        add_instruction(&mut state, Register::D);
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
        add_instruction(&mut state, Register::B);
        add_instruction(&mut state, Register::B);
        add_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 31, Register::A => 93 })
                .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
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
        add_instruction(&mut state, Register::B);
        add_instruction(&mut state, Register::C);
        add_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! {
                    Register::B => 11,
                    Register::C => 13,
                    Register::D => 15,
                    Register::A => 39,
                })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true, ConditionFlag::AuxiliaryCarry => true })
                .build(),
        )
    }

    #[test]
    fn add_adds_the_value_onto_any_existing_value_in_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 154, Register::E => 95 })
            .build();
        add_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 95, Register::A => 249 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Sign => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn add_doubles_the_accumulator_value_if_it_is_given_as_the_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 31 })
            .build();
        add_instruction(&mut state, Register::A);
        add_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 124 })
                .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
                .build(),
        );
    }

    #[test]
    fn add_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::default();
        add_instruction(&mut state, Register::A);
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
            .register_values(hashmap! { Register::A => 180, Register::E => 91 })
            .build();
        add_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 91, Register::A => 15 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
                )
                .build(),
        );
    }

    #[test]
    fn add_mem_adds_the_existing_memory_value_to_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 109, Register::H => 190, Register::L => 28 })
            .memory_values(hashmap! { 0xBE1C => 185 })
            .build();
        add_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 38, Register::H => 190, Register::L => 28 },
                )
                .memory_values(hashmap! { 0xBE1C => 185 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true })
                .build(),
        );
    }

    #[test]
    fn adi_adds_the_given_value_onto_the_default_accumulator_value() {
        let mut state = State::default();
        adi_instruction(&mut state, 64);
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
        adi_instruction(&mut state, 37);
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
        adi_instruction(&mut state, 0);
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
            .register_values(hashmap! { Register::A => 161 })
            .build();
        adi_instruction(&mut state, 241);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 146 })
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
        adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 82, Register::B => 44 })
                .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
                .build(),
        )
    }

    #[test]
    fn adc_adds_to_the_accumulator_the_register_value_and_carry_flag_when_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 38, Register::B => 44 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 83, Register::B => 44 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true, ConditionFlag::AuxiliaryCarry => true })
                .build(),
        )
    }

    #[test]
    fn adc_correctly_overflows_with_the_carry_flag_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 127, Register::B => 227 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 99, Register::B => 227 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn adc_correctly_overflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 224, Register::B => 31 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        adc_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 31 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        )
    }

    #[test]
    fn adc_mem_adds_memory_value_and_carry_flag_to_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(
                hashmap! { Register::A => 139, Register::H => 121, Register::L => 137 },
            )
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .memory_values(hashmap! { 0x7989 => 188 })
            .build();
        adc_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 72, Register::H => 121, Register::L => 137 },
                )
                .condition_flag_values(hashmap! {
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .memory_values(hashmap! { 0x7989 => 188 })
                .build(),
        )
    }

    #[test]
    fn aci_adds_the_given_value_to_accumulator_when_carry_flag_is_not_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 38 })
            .build();
        aci_instruction(&mut state, 44);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 82 })
                .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
                .build(),
        )
    }

    #[test]
    fn aci_adds_to_the_accumulator_the_given_value_and_carry_flag_when_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 38 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        aci_instruction(&mut state, 44);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 83 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true, ConditionFlag::AuxiliaryCarry => true })
                .build(),
        )
    }

    #[test]
    fn aci_correctly_overflows_with_the_carry_flag_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 127 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        aci_instruction(&mut state, 227);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 99 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn aci_correctly_overflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 96 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        aci_instruction(&mut state, 159);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        )
    }

    #[test]
    fn sub_subtracts_the_existing_register_value_from_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 127, Register::D => 24 })
            .build();
        sub_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 24, Register::A => 103 })
                .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
                .build(),
        )
    }

    #[test]
    fn sub_can_subtract_a_value_from_the_accumulator_multiple_times() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 127, Register::B => 42 })
            .build();
        sub_instruction(&mut state, Register::B);
        sub_instruction(&mut state, Register::B);
        sub_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 42, Register::A => 1 })
                .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
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
        sub_instruction(&mut state, Register::B);
        sub_instruction(&mut state, Register::C);
        sub_instruction(&mut state, Register::D);
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
        sub_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn sub_sets_the_carry_flag_when_underflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 87, Register::E => 182 })
            .build();
        sub_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 182, Register::A => 161 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Sign => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn sub_mem_subtracts_the_existing_memory_value_from_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 111, Register::H => 122, Register::L => 5 })
            .memory_values(hashmap! { 0x7A05 => 211 })
            .build();
        sub_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 156, Register::H => 122, Register::L => 5 },
                )
                .memory_values(hashmap! { 0x7A05 => 211 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Sign => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn sui_subtracts_the_given_value_from_any_existing_value_in_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 66 })
            .build();
        sui_instruction(&mut state, 55);
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
        sui_instruction(&mut state, 75);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn sui_sets_the_carry_flag_when_underflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 87 })
            .build();
        sui_instruction(&mut state, 182);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 161 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Sign => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn sbb_subtracts_the_register_value_from_accumulator_when_carry_flag_is_not_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 88, Register::B => 77 })
            .build();
        sbb_instruction(&mut state, Register::B);
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
        sbb_instruction(&mut state, Register::B);
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
            .register_values(hashmap! { Register::A => 28, Register::B => 195 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 88, Register::B => 195 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true })
                .build(),
        )
    }

    #[test]
    fn sbb_correctly_underflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 31, Register::B => 31 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        sbb_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 255, Register::B => 31 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Sign => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                })
                .build(),
        )
    }

    #[test]
    fn sbb_mem_subtracts_memory_value_and_carry_flag_from_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 49, Register::H => 12, Register::L => 109 })
            .memory_values(hashmap! { 0x0C6D => 177 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        sbb_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 127, Register::H => 12, Register::L => 109 },
                )
                .memory_values(hashmap! { 0x0C6D => 177 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        )
    }

    #[test]
    fn sbi_subtracts_the_given_value_from_accumulator_when_carry_flag_is_not_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 88 })
            .build();
        sbi_instruction(&mut state, 77);
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
        sbi_instruction(&mut state, 77);
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
            .register_values(hashmap! { Register::A => 29 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        sbi_instruction(&mut state, 156);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 128 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Sign => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        )
    }

    #[test]
    fn sbi_correctly_underflows_as_a_result_of_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 159 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        sbi_instruction(&mut state, 159);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 255 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Sign => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                })
                .build(),
        )
    }

    #[test]
    fn inr_increments_default_register_value() {
        let mut state = State::default();
        inr_instruction(&mut state, Register::C);
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
        inr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 64 })
                .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
                .build(),
        );
    }

    #[test]
    fn inr_does_not_set_carry_flag_when_overflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::C => 255 })
            .build();
        inr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn inr_mem_increments_existing_memory_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 132, Register::L => 55 })
            .memory_values(hashmap! { 0x8436 => 13, 0x8437 => 14, 0x8438 => 109 })
            .build();
        inr_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 132, Register::L => 55 })
                .memory_values(hashmap! { 0x8436 => 13, 0x8437 => 15, 0x8438 => 109 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    fn inr_mem_does_not_set_carry_flag_when_overflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 21, Register::L => 91 })
            .memory_values(hashmap! { 0x155B => 255 })
            .build();
        inr_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 21, Register::L => 91 })
                .memory_values(hashmap! { 0x155B => 0 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }

    #[test]
    fn dcr_decrements_default_register_value() {
        let mut state = State::default();
        dcr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::C => 255 })
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
        dcr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::E => 126 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true, ConditionFlag::AuxiliaryCarry => true })
                .build(),
        );
    }

    #[test]
    fn dcr_mem_decrements_existing_memory_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 191, Register::L => 154 })
            .memory_values(hashmap! { 0xBF99 => 87, 0xBF9A => 225, 0xBF9B => 83 })
            .build();
        dcr_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 191, Register::L => 154 })
                .memory_values(hashmap! { 0xBF99 => 87, 0xBF9A => 224, 0xBF9B => 83 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::AuxiliaryCarry => true },
                )
                .build(),
        );
    }

    #[test]
    fn dcr_mem_does_not_set_carry_flag_when_underflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 56, Register::L => 206 })
            .memory_values(hashmap! { 0x38CE => 0 })
            .build();
        dcr_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 56, Register::L => 206 })
                .memory_values(hashmap! { 0x38CE => 255 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn inx_increments_the_register_pair_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 47, Register::E => 141 })
            .build();
        inx_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 47, Register::E => 142 })
                .build(),
        );
    }

    #[test]
    fn inx_can_overflow_into_higher_register_of_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 204, Register::L => 255 })
            .build();
        inx_instruction(&mut state, RegisterPair::HL);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 205, Register::L => 0 })
                .build(),
        );
    }

    #[test]
    fn dcx_decrements_the_register_pair_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 81, Register::L => 134 })
            .build();
        dcx_instruction(&mut state, RegisterPair::HL);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 81, Register::L => 133 })
                .build(),
        );
    }

    #[test]
    fn dcx_can_underflow_into_higher_register_of_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 149, Register::E => 0 })
            .build();
        dcx_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 148, Register::E => 255 })
                .build(),
        );
    }

    #[test]
    fn dad_adds_the_given_register_pair_value_onto_existing_register_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! {
                Register::B => 117,
                Register::C => 168,
                Register::H => 75,
                Register::L => 43,
            })
            .build();
        dad_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! {
                    Register::B => 117,
                    Register::C => 168,
                    Register::H => 192,
                    Register::L => 211,
                })
                .build(),
        );
    }

    #[test]
    fn dad_doubles_the_existing_register_pair_value_if_given() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 32, Register::L => 175 })
            .build();
        dad_instruction(&mut state, RegisterPair::HL);
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
            .register_values(hashmap! { Register::H => 160, Register::L => 6 })
            .stack_pointer(0x13FE)
            .build();
        dad_instruction(&mut state, RegisterPair::SP);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 180, Register::L => 4 })
                .stack_pointer(0x13FE)
                .build(),
        );
    }

    #[test]
    fn dad_sets_the_carry_flag_when_overflowing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! {
                Register::D => 146,
                Register::E => 208,
                Register::H => 109,
                Register::L => 48,
            })
            .build();
        dad_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 146, Register::E => 208 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn daa_does_nothing_if_no_conditions_are_met() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b1001_1001 })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b1001_1001 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn daa_adds_to_lower_nibble_if_greater_than_threshold() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b0111_1010 })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b1000_0000 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::AuxiliaryCarry => true },
                )
                .build(),
        );
    }

    #[test]
    fn daa_adds_to_lower_nibble_if_auxiliary_carry_flag_is_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b1001_1001 })
            .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b1001_1111 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn daa_adds_to_higher_nibble_if_greater_than_threshold() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b1010_1001 })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b0000_1001 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
                )
                .build(),
        );
    }

    #[test]
    fn daa_adds_to_higher_nibble_if_carry_flag_is_set() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b0001_1001 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b0111_1001 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn daa_adds_to_both_nibbles_if_both_conditions_are_met() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b1111_0011 })
            .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b0101_1001 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
                )
                .build(),
        );
    }

    #[test]
    fn daa_lower_nibble_can_fulfill_threshold_for_higher_nibble() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b1001_1011 })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b0000_0001 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true })
                .build(),
        );
    }

    #[test]
    fn daa_can_carry_from_add_to_lower_nibble() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b1111_1010 })
            .build();
        daa_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b0110_0000 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .build(),
        );
    }
}
