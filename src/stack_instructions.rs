use crate::{RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn push_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::SP {
        panic!(
            "The register pair {:?} is not supported by the PUSH operation",
            register_pair
        );
    }

    let (register_pair_low, register_pair_high) = register_pair.get_low_high_value(state);
    let sp_minus_one = state.stack_pointer.wrapping_sub(1);
    let sp_minus_two = state.stack_pointer.wrapping_sub(2);
    state.set_value_at_memory_location(sp_minus_one, register_pair_high as u8);
    state.set_value_at_memory_location(sp_minus_two, register_pair_low as u8);
    state.stack_pointer = sp_minus_two;
}

#[cfg_attr(test, mutate)]
pub fn pop_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::SP {
        panic!(
            "The register pair {:?} is not supported by the POP operation",
            register_pair
        );
    }

    let sp_plus_one = state.stack_pointer.wrapping_add(1);
    let sp_plus_two = state.stack_pointer.wrapping_add(2);
    let value_for_register_pair_low = state.get_value_at_memory_location(state.stack_pointer);
    let value_for_register_pair_high = state.get_value_at_memory_location(sp_plus_one);
    register_pair.set_low_high_value(
        state,
        value_for_register_pair_low as i8,
        value_for_register_pair_high as i8,
    );
    state.stack_pointer = sp_plus_two;
}

#[cfg_attr(test, mutate)]
pub fn sphl_instruction(state: &mut State) {
    state.stack_pointer = RegisterPair::HL.get_full_value(&state);
}

#[cfg_attr(test, mutate)]
pub fn ei_instruction(state: &mut State) {
    state.are_interrupts_enabled = true;
}

#[cfg_attr(test, mutate)]
pub fn di_instruction(state: &mut State) {
    state.are_interrupts_enabled = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{Register, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn push_sets_stack_pointer_values_based_on_given_register_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => -35, Register::C => 101 })
            .stack_pointer(0xF028)
            .build();
        push_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => -35, Register::C => 101 })
                .stack_pointer(0xF026)
                .memory_values(hashmap! { 0xF026 => 101, 0xF027 => 221 })
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The register pair SP is not supported by the PUSH operation")]
    fn push_does_not_support_stack_pointer_as_given_register_pair() {
        let mut state = State::default();
        push_instruction(&mut state, RegisterPair::SP);
    }

    #[test]
    fn pop_sets_given_register_pair_values_based_on_stack_pointer() {
        let mut state = StateBuilder::default()
            .stack_pointer(0x8CCD)
            .memory_values(hashmap! { 0x8CCC => 102, 0x8CCD => 40, 0x8CCE => 204, 0x8CCF => 16 })
            .build();
        pop_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => -52, Register::E => 40 })
                .stack_pointer(0x8CCF)
                .memory_values(
                    hashmap! { 0x8CCC => 102, 0x8CCD => 40, 0x8CCE => 204, 0x8CCF => 16 },
                )
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The register pair SP is not supported by the POP operation")]
    fn pop_does_not_support_stack_pointer_as_given_register_pair() {
        let mut state = State::default();
        pop_instruction(&mut state, RegisterPair::SP);
    }

    #[test]
    fn sphl_sets_the_stack_pointer_to_register_values() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 10, Register::L => -100 })
            .build();
        sphl_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 10, Register::L => -100 })
                .stack_pointer(0x0A9C)
                .build(),
        );
    }

    #[test]
    fn ei_sets_interrupts_as_enabled() {
        let mut state = State::default();
        ei_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default().are_interrupts_enabled(true).build(),
        )
    }

    #[test]
    fn di_sets_interrupts_as_disabled() {
        let mut state = StateBuilder::default().are_interrupts_enabled(true).build();
        di_instruction(&mut state);
        assert_state_is_as_expected(&state, &State::default())
    }
}
