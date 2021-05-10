use crate::{Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn mvi_instruction(state: &mut State, register: Register, data: i8) {
    state.set_register(register, data);
}

#[cfg_attr(test, mutate)]
pub fn mov_instruction(state: &mut State, from_register: Register, to_register: Register) {
    let from_register_value = state.get_register_value(from_register);
    mvi_instruction(state, to_register, from_register_value);
}

#[cfg_attr(test, mutate)]
pub fn sta_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let memory_address = State::concat_low_high_bytes(low_data, high_data);
    let accumulator_value = state.get_register_value(Register::A);
    state.set_value_at_memory_location(memory_address, accumulator_value as u8);
}

#[cfg_attr(test, mutate)]
pub fn xchg_instruction(state: &mut State) {
    state.exchange_register_values(Register::D, Register::H);
    state.exchange_register_values(Register::E, Register::L);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::{
        assert_low_high_memory_location_contains_value, assert_state_is_as_expected,
    };
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
        crate::transfer_instructions::mvi_instruction(&mut state, Register::B, 1);
        crate::transfer_instructions::mvi_instruction(&mut state, Register::D, 127);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 1, Register::D => 127 },
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
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 91 });
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::C);
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::E);
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::L);
        assert_state_is_as_expected(
            &state,
            hashmap! {
                Register::A => 91,
                Register::C => 91,
                Register::E => 91,
                Register::L => 91,
            },
            HashMap::new(),
        );
    }

    #[test]
    fn sta_stores_the_accumulator_value_into_the_given_memory_address() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => -42 });
        let (low_data, high_data) = (0x99, 0x01);
        crate::transfer_instructions::sta_instruction(&mut state, low_data, high_data);
        assert_state_is_as_expected(&state, hashmap! { Register::A => -42 }, HashMap::new());
        assert_low_high_memory_location_contains_value(&state, low_data, high_data, 214);
    }

    #[test]
    fn xchg_exchanges_content_of_registers() {
        let mut state = State::with_initial_register_state(hashmap! {
            Register::D => 78,
            Register::E => 69,
            Register::L => 11,
        });
        crate::transfer_instructions::xchg_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            hashmap! {
                Register::H => 78,
                Register::E => 11,
                Register::L => 69,
            },
            HashMap::new(),
        );
    }
}
