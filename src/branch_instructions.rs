use crate::{ConditionFlag, RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn jmp_instruction(state: &mut State, low_data: u8, high_data: u8) {
    state.program_counter = crate::bit_operations::concat_low_high_bytes(low_data, high_data);
}

#[cfg_attr(test, mutate)]
pub fn jcond_instruction(
    state: &mut State,
    low_data: u8,
    high_data: u8,
    condition: (ConditionFlag, bool),
) {
    if condition.0 == ConditionFlag::AuxiliaryCarry {
        panic!("The auxiliary carry flag is not a supported condition for JMP");
    }

    if state.get_condition_flag_value(condition.0) == condition.1 {
        state.program_counter = crate::bit_operations::concat_low_high_bytes(low_data, high_data);
    }
}

#[cfg_attr(test, mutate)]
pub fn pchl_instruction(state: &mut State) {
    state.program_counter = RegisterPair::HL.get_full_value(&state);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{ConditionFlag, Register, State, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn jmp_sets_the_program_counter_to_the_given_value() {
        let mut state = State::default();
        crate::branch_instructions::jmp_instruction(&mut state, 0x0D, 0xD0);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default().program_counter(0xD00D).build(),
        );
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
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default().program_counter(0xFFFF).build(),
        );
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
        assert_state_is_as_expected(&state, &State::default());
    }

    #[test]
    fn jcond_sets_the_program_counter_when_carry_flag_is_set_for_condition() {
        let mut state = StateBuilder::default()
            .condition_flag_values(hashmap! { ConditionFlag::Carry => true })
            .build();
        crate::branch_instructions::jcond_instruction(
            &mut state,
            0x0F,
            0x00,
            (ConditionFlag::Carry, true),
        );
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
        crate::branch_instructions::jcond_instruction(
            &mut state,
            0x0D,
            0xF0,
            (ConditionFlag::AuxiliaryCarry, true),
        );
    }

    #[test]
    fn pchl_sets_the_program_counter_from_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => -64, Register::L => 63 })
            .build();
        crate::branch_instructions::pchl_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -64, Register::L => 63 })
                .program_counter(0xC03F)
                .build(),
        );
    }
}
