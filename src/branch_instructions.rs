use crate::{bit_operations, Condition, ConditionFlag, RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
fn is_condition_true(state: &State, condition: Condition, base_instruction: &str) -> bool {
    if condition.0 == ConditionFlag::AuxiliaryCarry {
        panic!("The auxiliary carry flag is not a supported condition for {base_instruction}");
    }

    state.is_condition_true(condition)
}

#[cfg_attr(test, mutate)]
pub fn jmp_instruction(state: &mut State, low_data: u8, high_data: u8) {
    state.program_counter = bit_operations::concat_low_high_bytes(low_data, high_data);
}

#[cfg_attr(test, mutate)]
pub fn jcond_instruction(state: &mut State, low_data: u8, high_data: u8, condition: Condition) {
    if is_condition_true(state, condition, "JMP") {
        jmp_instruction(state, low_data, high_data);
    }
}

#[cfg_attr(test, mutate)]
pub fn call_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let (pc_low, pc_high) = bit_operations::split_to_low_high_bytes(state.program_counter);
    let sp_minus_one = state.stack_pointer.wrapping_sub(1);
    let sp_minus_two = state.stack_pointer.wrapping_sub(2);
    state.memory[sp_minus_one as usize] = pc_high;
    state.memory[sp_minus_two as usize] = pc_low;
    state.stack_pointer = sp_minus_two;
    state.program_counter = bit_operations::concat_low_high_bytes(low_data, high_data);
}

#[cfg_attr(test, mutate)]
pub fn ccond_instruction(state: &mut State, low_data: u8, high_data: u8, condition: Condition) {
    if is_condition_true(state, condition, "CALL") {
        call_instruction(state, low_data, high_data);
    }
}

#[cfg_attr(test, mutate)]
pub fn ret_instruction(state: &mut State) {
    let sp_plus_one = state.stack_pointer.wrapping_add(1);
    let sp_plus_two = state.stack_pointer.wrapping_add(2);
    let value_for_pc_low = state.memory[state.stack_pointer as usize];
    let value_for_pc_high = state.memory[sp_plus_one as usize];
    state.program_counter =
        bit_operations::concat_low_high_bytes(value_for_pc_low, value_for_pc_high);
    state.stack_pointer = sp_plus_two;
}

#[cfg_attr(test, mutate)]
pub fn rcond_instruction(state: &mut State, condition: Condition) {
    if is_condition_true(state, condition, "RET") {
        ret_instruction(state);
    }
}

#[cfg_attr(test, mutate)]
pub fn rst_instruction(state: &mut State, reset_index: u8) {
    if reset_index >= 8 {
        panic!("Invalid reset index of {reset_index}");
    }

    call_instruction(state, reset_index * 8, 0x00);
}

#[cfg_attr(test, mutate)]
pub fn pchl_instruction(state: &mut State) {
    state.program_counter = state.full_rp_value(RegisterPair::HL);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{Register, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn jmp_sets_the_program_counter_to_the_given_value() {
        let mut state = State::default();
        jmp_instruction(&mut state, 0x0D, 0xD0);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default().program_counter(0xD00D).build(),
        );
    }

    #[test]
    fn jcond_sets_the_program_counter_when_condition_is_true() {
        let mut state = State::default();
        jcond_instruction(&mut state, 0xFF, 0xFF, (ConditionFlag::Zero, false));
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default().program_counter(0xFFFF).build(),
        );
    }

    #[test]
    fn jcond_does_not_set_the_program_counter_when_condition_is_false() {
        let mut state = State::default();
        jcond_instruction(&mut state, 0xFF, 0xFF, (ConditionFlag::Zero, true));
        assert_state_is_as_expected(&state, &State::default());
    }

    #[test]
    fn jcond_sets_the_program_counter_when_carry_flag_is_set_for_condition() {
        let mut state = StateBuilder::default()
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        jcond_instruction(&mut state, 0x0F, 0x00, (ConditionFlag::Carry, true));
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
                .program_counter(0x000F)
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The auxiliary carry flag is not a supported condition for JMP")]
    fn jcond_does_not_support_auxiliary_carry_as_condition() {
        let mut state = StateBuilder::default()
            .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
            .build();
        jcond_instruction(
            &mut state,
            0x0D,
            0xF0,
            (ConditionFlag::AuxiliaryCarry, true),
        );
    }

    #[test]
    fn call_calls_the_function_at_the_given_value() {
        let mut state = StateBuilder::default()
            .program_counter(0x33FA)
            .stack_pointer(0x77E1)
            .build();
        call_instruction(&mut state, 0x6F, 0x7B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .program_counter(0x7B6F)
                .stack_pointer(0x77DF)
                .memory_values(hashmap! { 0x77DF => 250, 0x77E0 => 51 })
                .build(),
        );
    }

    #[test]
    fn call_underflows_if_stack_pointer_is_near_zero() {
        let mut state = StateBuilder::default()
            .program_counter(0x40E0)
            .stack_pointer(0x0001)
            .build();
        call_instruction(&mut state, 0x0D, 0x4E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .program_counter(0x4E0D)
                .stack_pointer(0xFFFF)
                .memory_values(hashmap! { 0xFFFF => 224, 0x0000 => 64 })
                .build(),
        );
    }

    #[test]
    fn call_overwrites_if_pointers_are_default() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0xFFFD => 208, 0xFFFE => 146, 0xFFFF => 80, 0x0000 => 73 })
            .build();
        call_instruction(&mut state, 0xB3, 0x2D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .program_counter(0x2DB3)
                .stack_pointer(0xFFFE)
                .memory_values(hashmap! { 0xFFFD => 208, 0x0000 => 73 })
                .build(),
        );
    }

    #[test]
    fn ccond_calls_the_function_when_condition_is_true() {
        let mut state = StateBuilder::default()
            .program_counter(0x33FA)
            .stack_pointer(0x77E1)
            .condition_flag_values(hashmap! { ConditionFlag::Zero => true })
            .build();
        ccond_instruction(&mut state, 0x6F, 0x7B, (ConditionFlag::Zero, true));
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .program_counter(0x7B6F)
                .stack_pointer(0x77DF)
                .memory_values(hashmap! { 0x77DF => 250, 0x77E0 => 51 })
                .condition_flag_values(hashmap! { ConditionFlag::Zero => true })
                .build(),
        );
    }

    #[test]
    fn ccond_does_not_call_the_function_when_condition_is_false() {
        let mut state = StateBuilder::default()
            .program_counter(0x33FA)
            .stack_pointer(0x77E1)
            .condition_flag_values(hashmap! { ConditionFlag::Zero => true })
            .build();
        ccond_instruction(&mut state, 0x6F, 0x7B, (ConditionFlag::Zero, false));
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .program_counter(0x33FA)
                .stack_pointer(0x77E1)
                .condition_flag_values(hashmap! { ConditionFlag::Zero => true })
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The auxiliary carry flag is not a supported condition for CALL")]
    fn ccond_does_not_support_auxiliary_carry_as_condition() {
        let mut state = StateBuilder::default()
            .condition_flag_values(hashmap! { ConditionFlag::AuxiliaryCarry => true })
            .build();
        ccond_instruction(
            &mut state,
            0x0D,
            0xF0,
            (ConditionFlag::AuxiliaryCarry, false),
        );
    }

    #[test]
    fn ret_returns_from_the_stack_pointer_value() {
        let mut state = StateBuilder::default()
            .stack_pointer(0xA462)
            .memory_values(hashmap! { 0xA462 => 72, 0xA463 => 201 })
            .build();
        ret_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .stack_pointer(0xA464)
                .program_counter(0xC948)
                .memory_values(hashmap! { 0xA462 => 72, 0xA463 => 201 })
                .build(),
        );
    }

    #[test]
    fn ret_overflows_if_stack_pointer_is_near_max() {
        let mut state = StateBuilder::default()
            .stack_pointer(0xFFFF)
            .memory_values(hashmap! { 0xFFFE => 33, 0xFFFF => 118, 0x0000 => 160, 0x0001 => 155 })
            .build();
        ret_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .program_counter(0xA076)
                .stack_pointer(0x0001)
                .memory_values(
                    hashmap! { 0xFFFE => 33, 0xFFFF => 118, 0x0000 => 160, 0x0001 => 155 },
                )
                .build(),
        );
    }

    #[test]
    fn rcond_returns_when_condition_is_true() {
        let mut state = StateBuilder::default()
            .stack_pointer(0xA462)
            .memory_values(hashmap! { 0xA462 => 72, 0xA463 => 201 })
            .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
            .build();
        rcond_instruction(&mut state, (ConditionFlag::Parity, true));
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .stack_pointer(0xA464)
                .program_counter(0xC948)
                .memory_values(hashmap! { 0xA462 => 72, 0xA463 => 201 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    fn rcond_does_not_return_when_condition_is_false() {
        let mut state = StateBuilder::default()
            .stack_pointer(0xA462)
            .memory_values(hashmap! { 0xA462 => 72, 0xA463 => 201 })
            .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
            .build();
        rcond_instruction(&mut state, (ConditionFlag::Parity, false));
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .stack_pointer(0xA462)
                .memory_values(hashmap! { 0xA462 => 72, 0xA463 => 201 })
                .condition_flag_values(hashmap! { ConditionFlag::Parity => true })
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The auxiliary carry flag is not a supported condition for RET")]
    fn rcond_does_not_support_auxiliary_carry_as_condition() {
        let mut state = State::default();
        rcond_instruction(&mut state, (ConditionFlag::AuxiliaryCarry, true));
    }

    #[test]
    fn rst_calls_the_function_at_a_set_offset_from_the_given_value() {
        let mut state = StateBuilder::default()
            .program_counter(0x1914)
            .stack_pointer(0x9DEC)
            .build();
        rst_instruction(&mut state, 7);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .program_counter(0x0038)
                .stack_pointer(0x9DEA)
                .memory_values(hashmap! { 0x9DEA => 20, 0x9DEB => 25 })
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "Invalid reset index of 8")]
    fn rst_panics_when_given_an_invalid_reset_index() {
        let mut state = State::default();
        rst_instruction(&mut state, 8);
    }

    #[test]
    fn pchl_sets_the_program_counter_from_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 192, Register::L => 63 })
            .build();
        pchl_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 192, Register::L => 63 })
                .program_counter(0xC03F)
                .build(),
        );
    }
}
