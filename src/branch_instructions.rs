use crate::{ConditionFlag, Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn jmp_instruction(state: &mut State, high_data: u8, low_data: u8) {
    state.program_counter = state.concat_high_low_bytes(high_data, low_data);
}

#[cfg_attr(test, mutate)]
pub fn jcond_instruction(
    state: &mut State,
    high_data: u8,
    low_data: u8,
    condition: (ConditionFlag, bool),
) {
    if condition.0 == ConditionFlag::AuxiliaryCarry {
        panic!("The auxiliary carry flag is not a supported condition for JMP");
    }

    if state.get_condition_flag_value(condition.0) == condition.1 {
        state.program_counter = state.concat_high_low_bytes(high_data, low_data);
    }
}

#[cfg_attr(test, mutate)]
pub fn pchl_instruction(state: &mut State) {
    let h_register_value = state.get_register_value(Register::H) as u8;
    let l_register_value = state.get_register_value(Register::L) as u8;
    state.program_counter = state.concat_high_low_bytes(h_register_value, l_register_value);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::{
        assert_full_state_is_as_expected, assert_state_is_as_expected,
    };
    use crate::{ConditionFlag, Register, RegisterState, State};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn jmp_sets_the_program_counter_to_the_given_value() {
        let mut state = State::default();
        crate::branch_instructions::jmp_instruction(&mut state, 0xD0, 0x0D);
        assert_full_state_is_as_expected(&state, RegisterState::new(), HashMap::new(), 0xD00D);
    }

    #[test]
    fn jcond_sets_the_program_counter_when_condition_is_true() {
        let mut state = State::default();
        crate::branch_instructions::jcond_instruction(
            &mut state,
            0xFF,
            0xFF,
            (ConditionFlag::Zero, false),
        );
        assert_full_state_is_as_expected(&state, RegisterState::new(), HashMap::new(), 0xFFFF);
    }

    #[test]
    fn jcond_does_not_set_the_program_counter_when_condition_is_false() {
        let mut state = State::default();
        crate::branch_instructions::jcond_instruction(
            &mut state,
            0xFF,
            0xFF,
            (ConditionFlag::Zero, true),
        );
        assert_state_is_as_expected(&state, RegisterState::new(), HashMap::new());
    }

    #[test]
    fn jcond_sets_the_program_counter_when_carry_flag_is_set_for_condition() {
        let mut state = State::default();
        state.condition_flags.carry = true;
        crate::branch_instructions::jcond_instruction(
            &mut state,
            0x00,
            0x0F,
            (ConditionFlag::Carry, true),
        );
        assert_full_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Carry => true },
            0x000F,
        );
    }

    #[test]
    #[should_panic(expected = "The auxiliary carry flag is not a supported condition for JMP")]
    fn jcond_does_not_support_auxiliary_carry_as_condition() {
        let mut state = State::default();
        state.condition_flags.auxiliary_carry = true;
        crate::branch_instructions::jcond_instruction(
            &mut state,
            0xF0,
            0x0D,
            (ConditionFlag::AuxiliaryCarry, true),
        );
    }

    #[test]
    fn pchl_sets_the_program_counter_from_registers() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::H => -64, Register::L => 63 });
        crate::branch_instructions::pchl_instruction(&mut state);
        assert_full_state_is_as_expected(
            &state,
            hashmap! { Register::H => -64, Register::L => 63 },
            HashMap::new(),
            0xC03F,
        );
    }
}
