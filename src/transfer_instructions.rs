use crate::{Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn mvi_instruction(state: &mut State, register: Register, data: u8) {
    state.set_register(register, data);
}

#[cfg_attr(test, mutate)]
pub fn mov_instruction(state: &mut State, from_register: Register, to_register: Register) {
    let from_register_value = state.get_register_value(from_register);
    mvi_instruction(state, to_register, from_register_value);
}

#[cfg_attr(test, mutate)]
pub fn xchg_instruction(state: &mut State) {
    state.exchange_register_values(Register::D, Register::H);
    state.exchange_register_values(Register::E, Register::L);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{Register, State};
    use maplit::hashmap;
    use std::collections::HashMap;

    #[test]
    fn mvi_loads_data_into_one_register() {
        let mut state = State::default();
        crate::transfer_instructions::mvi_instruction(&mut state, Register::A, 64);
        assert_state_is_as_expected(&state, hashmap! { Register::A => 64 }, HashMap::new());
    }

    #[test]
    fn mvi_loads_data_into_multiple_registers() {
        let mut state = State::default();
        crate::transfer_instructions::mvi_instruction(&mut state, Register::B, 128);
        crate::transfer_instructions::mvi_instruction(&mut state, Register::D, 255);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 128, Register::D => 255 },
            HashMap::new(),
        );
    }

    #[test]
    fn mov_moves_value_from_one_register_to_another() {
        let mut state = State::with_initial_register_state(hashmap! { Register::H => 99 });
        crate::transfer_instructions::mov_instruction(&mut state, Register::H, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::H => 99, Register::A => 99 },
            HashMap::new(),
        );
    }

    #[test]
    fn mov_does_nothing_when_both_registers_are_the_same() {
        let mut state = State::with_initial_register_state(hashmap! { Register::L => 121 });
        crate::transfer_instructions::mov_instruction(&mut state, Register::L, Register::L);
        assert_state_is_as_expected(&state, hashmap! { Register::L => 121 }, HashMap::new());
    }

    #[test]
    fn multiple_mov_can_move_to_multiple_registers() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 218 });
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::C);
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::E);
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::L);
        assert_state_is_as_expected(
            &state,
            hashmap! {
                Register::A => 218,
                Register::C => 218,
                Register::E => 218,
                Register::L => 218,
            },
            HashMap::new(),
        );
    }

    #[test]
    fn xchg_exchanges_content_of_registers() {
        let mut state = State::with_initial_register_state(hashmap! {
            Register::D => 205,
            Register::E => 69,
            Register::L => 11,
        });
        crate::transfer_instructions::xchg_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            hashmap! {
                Register::H => 205,
                Register::E => 11,
                Register::L => 69,
            },
            HashMap::new(),
        );
    }
}
