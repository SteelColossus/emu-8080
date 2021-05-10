use crate::{Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn sphl_instruction(state: &mut State) {
    let h_register_value = state.get_register_value(Register::H) as u8;
    let l_register_value = state.get_register_value(Register::L) as u8;
    state.stack_pointer =
        crate::bit_operations::concat_low_high_bytes(l_register_value, h_register_value);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_full_state_is_as_expected;
    use crate::{Register, State};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn sphl_sets_the_stack_pointer_to_register_values() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::H => 10, Register::L => -100 });
        crate::stack_instructions::sphl_instruction(&mut state);
        assert_full_state_is_as_expected(
            &state,
            hashmap! { Register::H => 10, Register::L => -100 },
            HashMap::new(),
            0x0000,
            0x0A9C,
        );
    }
}
