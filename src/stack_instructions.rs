use crate::{RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn sphl_instruction(state: &mut State) {
    state.stack_pointer = RegisterPair::HL.get_full_value(&state);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{Register, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn sphl_sets_the_stack_pointer_to_register_values() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 10, Register::L => -100 })
            .build();
        crate::stack_instructions::sphl_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 10, Register::L => -100 })
                .stack_pointer(0x0A9C)
                .build(),
        );
    }
}
