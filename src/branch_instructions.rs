use crate::State;
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn jmp_instruction(state: &mut State, high_data: u8, low_data: u8) {
    state.program_counter = state.concat_high_low_bytes(high_data, low_data);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_full_state_is_as_expected;
    use crate::{RegisterState, State};
    use std::collections::HashMap;

    #[test]
    fn jmp_sets_the_program_counter_to_the_given_value() {
        let mut state = State::default();
        crate::branch_instructions::jmp_instruction(&mut state, 0xD0, 0x0D);
        assert_full_state_is_as_expected(&state, RegisterState::new(), HashMap::new(), 0xD00D);
    }
}
