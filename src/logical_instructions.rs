use crate::{bit_operations, Register, RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn ana_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    ani_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn ana_mem_instruction(state: &mut State) {
    let memory_address = RegisterPair::HL.get_full_value(state);
    let memory_value = state.get_value_at_memory_location(memory_address);
    ani_instruction(state, memory_value)
}

#[cfg_attr(test, mutate)]
pub fn ani_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |value, target_value| {
        value & target_value
    });
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags.carry = false;
    // TODO: This is not technically correct at the moment, since this function is shared with the other AND operations
    state.condition_flags.auxiliary_carry = false;
}

#[cfg_attr(test, mutate)]
pub fn xra_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    xri_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn xra_mem_instruction(state: &mut State) {
    let memory_address = RegisterPair::HL.get_full_value(state);
    let memory_value = state.get_value_at_memory_location(memory_address);
    xri_instruction(state, memory_value)
}

#[cfg_attr(test, mutate)]
pub fn xri_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |value, target_value| {
        value ^ target_value
    });
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags.carry = false;
    state.condition_flags.auxiliary_carry = false;
}

#[cfg_attr(test, mutate)]
pub fn ora_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    ori_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn ora_mem_instruction(state: &mut State) {
    let memory_address = RegisterPair::HL.get_full_value(state);
    let memory_value = state.get_value_at_memory_location(memory_address);
    ori_instruction(state, memory_value)
}

#[cfg_attr(test, mutate)]
pub fn ori_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |value, target_value| {
        value | target_value
    });
    state.set_condition_flags_from_register_value(Register::A);
    state.condition_flags.carry = false;
    state.condition_flags.auxiliary_carry = false;
}

#[cfg_attr(test, mutate)]
pub fn cmp_instruction(state: &mut State, register: Register) {
    let register_value = state.get_register_value(register);
    cpi_instruction(state, register_value);
}

#[cfg_attr(test, mutate)]
pub fn cmp_mem_instruction(state: &mut State) {
    let memory_address = RegisterPair::HL.get_full_value(state);
    let memory_value = state.get_value_at_memory_location(memory_address);
    cpi_instruction(state, memory_value);
}

#[cfg_attr(test, mutate)]
pub fn cpi_instruction(state: &mut State, data: u8) {
    let accumulator_value = state.get_register_value(Register::A);
    let result = accumulator_value.wrapping_sub(data);
    state.set_condition_flags_from_result(result);
    state.condition_flags.carry = accumulator_value < data;
}

#[cfg_attr(test, mutate)]
pub fn rlc_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, accumulator_value.rotate_left(1));
    state.condition_flags.carry = bit_operations::is_bit_set(accumulator_value, 7);
}

#[cfg_attr(test, mutate)]
pub fn rrc_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, accumulator_value.rotate_right(1));
    state.condition_flags.carry = bit_operations::is_bit_set(accumulator_value, 0);
}

#[cfg_attr(test, mutate)]
pub fn ral_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    let previous_carry = state.condition_flags.carry;
    let mut result = accumulator_value.rotate_left(1);
    let bit_index = 0;
    let carry = bit_operations::is_bit_set(result, bit_index);
    bit_operations::set_bit_in_value(&mut result, bit_index, previous_carry);
    state.set_register(Register::A, result);
    state.condition_flags.carry = carry;
}

#[cfg_attr(test, mutate)]
pub fn rar_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    let previous_carry = state.condition_flags.carry;
    let mut result = accumulator_value.rotate_right(1);
    let bit_index = 7;
    let carry = bit_operations::is_bit_set(result, bit_index);
    bit_operations::set_bit_in_value(&mut result, bit_index, previous_carry);
    state.set_register(Register::A, result);
    state.condition_flags.carry = carry;
}

#[cfg_attr(test, mutate)]
pub fn cma_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, !accumulator_value);
}

#[cfg_attr(test, mutate)]
pub fn cmc_instruction(state: &mut State) {
    state.condition_flags.carry = !state.condition_flags.carry;
}

#[cfg_attr(test, mutate)]
pub fn stc_instruction(state: &mut State) {
    state.condition_flags.carry = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{ConditionFlag, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn ana_logically_ands_the_accumulator_with_the_value_of_the_given_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 })
            .build();
        ana_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 0b01000101, Register::A => 0b01000100 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    fn ana_applied_to_an_existing_accumulator_value_does_nothing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b10100110 })
            .build();
        ana_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b10100110 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn ana_clears_the_carry_flag() {
        let mut state = StateBuilder::default()
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        ana_instruction(&mut state, Register::A);
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
    fn ana_mem_logically_ands_the_accumulator_with_the_memory_value() {
        let mut state = StateBuilder::default()
            .register_values(
                hashmap! { Register::A => 0b01010101, Register::H => 148, Register::L => 77 },
            )
            .memory_values(hashmap! { 0x944D => 0b11011011 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        ana_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 0b01010001, Register::H => 148, Register::L => 77 },
                )
                .memory_values(hashmap! { 0x944D => 0b11011011 })
                .build(),
        );
    }

    #[test]
    fn ani_logically_ands_the_accumulator_with_the_given_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b11000110 })
            .build();
        ani_instruction(&mut state, 0b01100011);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b01000010 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    fn ani_clears_the_carry_and_auxiliary_carry_flags() {
        let mut state = StateBuilder::default()
            .condition_flag_values(
                hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true },
            )
            .build();
        ani_instruction(&mut state, 0b00000000);
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
    fn xra_logically_xors_the_accumulator_with_the_value_of_the_given_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 })
            .build();
        xra_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 0b01000101, Register::A => 0b00010001 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    fn xra_applied_to_an_existing_accumulator_value_zeroes_that_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b10100110 })
            .build();
        xra_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b00000000 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn xra_clears_the_carry_and_auxiliary_carry_flags() {
        let mut state = StateBuilder::default()
            .condition_flag_values(
                hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true },
            )
            .build();
        xra_instruction(&mut state, Register::A);
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
    fn xra_mem_logically_xors_the_accumulator_with_the_memory_value() {
        let mut state = StateBuilder::default()
            .register_values(
                hashmap! { Register::A => 0b01010101, Register::H => 30, Register::L => 49 },
            )
            .memory_values(hashmap! { 0x1E31 => 0b11011011 })
            .condition_flag_values(
                hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true },
            )
            .build();
        xra_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 0b10001110, Register::H => 30, Register::L => 49 },
                )
                .memory_values(hashmap! { 0x1E31 => 0b11011011 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn xri_logically_xors_the_accumulator_with_the_given_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b11000110 })
            .build();
        xri_instruction(&mut state, 0b01100011);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b10100101 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn xri_clears_the_carry_and_auxiliary_carry_flags() {
        let mut state = StateBuilder::default()
            .condition_flag_values(
                hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true },
            )
            .build();
        xri_instruction(&mut state, 0b00000000);
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
    fn ora_logically_ors_the_accumulator_with_the_value_of_the_given_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 })
            .build();
        ora_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 0b01000101, Register::A => 0b01010101 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    fn ora_applied_to_an_existing_accumulator_value_does_nothing() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b10100110 })
            .build();
        ora_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b10100110 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn ora_clears_the_carry_and_auxiliary_carry_flags() {
        let mut state = StateBuilder::default()
            .condition_flag_values(
                hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true },
            )
            .build();
        ora_instruction(&mut state, Register::A);
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
    fn ora_mem_logically_ors_the_accumulator_with_the_memory_value() {
        let mut state = StateBuilder::default()
            .register_values(
                hashmap! { Register::A => 0b01010101, Register::H => 227, Register::L => 137 },
            )
            .memory_values(hashmap! { 0xE389 => 0b11011011 })
            .condition_flag_values(
                hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true },
            )
            .build();
        ora_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 0b11011111, Register::H => 227, Register::L => 137 },
                )
                .memory_values(hashmap! { 0xE389 => 0b11011011 })
                .condition_flag_values(hashmap! { ConditionFlag::Sign => true })
                .build(),
        );
    }

    #[test]
    fn ori_logically_ors_the_accumulator_with_the_given_value() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b11000110 })
            .build();
        ori_instruction(&mut state, 0b01100011);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b11100111 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn ori_clears_the_carry_and_auxiliary_carry_flags() {
        let mut state = StateBuilder::default()
            .condition_flag_values(
                hashmap! { ConditionFlag::Carry => true, ConditionFlag::AuxiliaryCarry => true },
            )
            .build();
        ori_instruction(&mut state, 0b00000000);
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
    fn cmp_sets_the_zero_flag_if_both_are_same() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 36, Register::H => 36 })
            .build();
        cmp_instruction(&mut state, Register::H);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 36, Register::H => 36 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn cmp_sets_the_carry_flag_if_register_value_is_greater() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 24, Register::E => 48 })
            .build();
        cmp_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 24, Register::E => 48 })
                .condition_flag_values(hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn cmp_mem_sets_the_zero_flag_if_both_are_same() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 99, Register::H => 163, Register::L => 57 })
            .memory_values(hashmap! { 0xA339 => 99 })
            .build();
        cmp_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 99, Register::H => 163, Register::L => 57 },
                )
                .memory_values(hashmap! { 0xA339 => 99 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn cmp_mem_sets_the_carry_flag_if_memory_value_is_greater() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 51, Register::H => 120, Register::L => 113 })
            .memory_values(hashmap! { 0x7871 => 253 })
            .build();
        cmp_mem_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::A => 51, Register::H => 120, Register::L => 113 },
                )
                .memory_values(hashmap! { 0x7871 => 253 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Parity => true, ConditionFlag::Carry => true },
                )
                .build(),
        );
    }

    #[test]
    fn cpi_sets_the_zero_flag_if_both_are_same() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 54 })
            .build();
        cpi_instruction(&mut state, 54);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 54 })
                .condition_flag_values(
                    hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
                )
                .build(),
        );
    }

    #[test]
    fn cpi_sets_the_carry_flag_if_given_value_is_greater() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 196 })
            .build();
        cpi_instruction(&mut state, 220);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 196 })
                .condition_flag_values(hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true, ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn rlc_shifts_the_accumulator_value_left() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b01100011 })
            .build();
        rlc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b11000110 })
                .build(),
        );
    }

    #[test]
    fn rlc_wraps_shifted_bit_and_sets_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b10000000 })
            .build();
        rlc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b00000001 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn rrc_shifts_the_accumulator_value_right() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b11000110 })
            .build();
        rrc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b01100011 })
                .build(),
        );
    }

    #[test]
    fn rrc_wraps_shifted_bit_and_sets_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b00000001 })
            .build();
        rrc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b10000000 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn ral_shifts_the_accumulator_value_left_setting_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b11000110 })
            .build();
        ral_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b10001100 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn ral_shifts_the_accumulator_value_left_including_the_carry_flag() {
        let mut state = StateBuilder::default()
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        ral_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b00000001 })
                .build(),
        );
    }

    #[test]
    fn ral_shifts_the_accumulator_value_left_setting_and_including_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b10100101 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        ral_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b01001011 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn rar_shifts_the_accumulator_value_right_setting_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b01100011 })
            .build();
        rar_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b00110001 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn rar_shifts_the_accumulator_value_right_including_the_carry_flag() {
        let mut state = StateBuilder::default()
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        rar_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b10000000 })
                .build(),
        );
    }

    #[test]
    fn rar_shifts_the_accumulator_value_right_setting_and_including_the_carry_flag() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b10100101 })
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        rar_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b11010010 })
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }

    #[test]
    fn cma_complements_the_value_in_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 0b11000110 })
            .build();
        cma_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 0b00111001 })
                .build(),
        );
    }

    #[test]
    fn cmc_inverts_the_carry_flag() {
        let mut state = State::default();
        cmc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
        cmc_instruction(&mut state);
        assert_state_is_as_expected(&state, &State::default());
    }

    #[test]
    fn stc_sets_the_carry_flag_to_true() {
        let mut state = State::default();
        stc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .build(),
        );
    }
}
