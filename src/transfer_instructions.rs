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
pub fn lda_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let memory_address = crate::bit_operations::concat_low_high_bytes(low_data, high_data);
    let memory_location_value = state.get_value_at_memory_location(memory_address);
    state.set_register(Register::A, memory_location_value as i8);
}

#[cfg_attr(test, mutate)]
pub fn sta_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let memory_address = crate::bit_operations::concat_low_high_bytes(low_data, high_data);
    let accumulator_value = state.get_register_value(Register::A);
    state.set_value_at_memory_location(memory_address, accumulator_value as u8);
}

#[cfg_attr(test, mutate)]
pub fn lhld_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let first_memory_address = crate::bit_operations::concat_low_high_bytes(low_data, high_data);
    let second_memory_address = first_memory_address.wrapping_add(1);
    let first_memory_value = state.get_value_at_memory_location(first_memory_address);
    let second_memory_value = state.get_value_at_memory_location(second_memory_address);
    state.set_register(Register::L, first_memory_value as i8);
    state.set_register(Register::H, second_memory_value as i8);
}

#[cfg_attr(test, mutate)]
pub fn shld_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let first_memory_address = crate::bit_operations::concat_low_high_bytes(low_data, high_data);
    let second_memory_address = first_memory_address.wrapping_add(1);
    let h_register_value = state.get_register_value(Register::H) as u8;
    let l_register_value = state.get_register_value(Register::L) as u8;
    state.set_value_at_memory_location(first_memory_address, l_register_value);
    state.set_value_at_memory_location(second_memory_address, h_register_value);
}

#[cfg_attr(test, mutate)]
pub fn xchg_instruction(state: &mut State) {
    state.exchange_register_values(Register::D, Register::H);
    state.exchange_register_values(Register::E, Register::L);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{Register, State, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn mvi_loads_data_into_one_register() {
        let mut state = State::default();
        crate::transfer_instructions::mvi_instruction(&mut state, Register::A, 64);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 64 })
                .build(),
        );
    }

    #[test]
    fn mvi_loads_data_into_multiple_registers() {
        let mut state = State::default();
        crate::transfer_instructions::mvi_instruction(&mut state, Register::B, 1);
        crate::transfer_instructions::mvi_instruction(&mut state, Register::D, 127);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 1, Register::D => 127 })
                .build(),
        );
    }

    #[test]
    fn mov_moves_value_from_one_register_to_another() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 99 })
            .build();
        crate::transfer_instructions::mov_instruction(&mut state, Register::H, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 99, Register::A => 99 })
                .build(),
        );
    }

    #[test]
    fn mov_does_nothing_when_both_registers_are_the_same() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::L => 121 })
            .build();
        crate::transfer_instructions::mov_instruction(&mut state, Register::L, Register::L);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::L => 121 })
                .build(),
        );
    }

    #[test]
    fn multiple_mov_can_move_to_multiple_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 91 })
            .build();
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::C);
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::E);
        crate::transfer_instructions::mov_instruction(&mut state, Register::A, Register::L);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! {
                    Register::A => 91,
                    Register::C => 91,
                    Register::E => 91,
                    Register::L => 91,
                })
                .build(),
        );
    }

    #[test]
    fn lda_loads_the_value_at_the_given_memory_location_into_the_accumulator() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0x0040 => 214 })
            .build();
        crate::transfer_instructions::lda_instruction(&mut state, 0x40, 0x00);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => -42 })
                .memory_values(hashmap! { 0x0040 => 214 })
                .build(),
        );
    }

    #[test]
    fn sta_stores_the_accumulator_value_into_the_given_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => -42 })
            .build();
        crate::transfer_instructions::sta_instruction(&mut state, 0x99, 0x01);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => -42 })
                .memory_values(hashmap! { 0x0199 => 214 })
                .build(),
        );
    }

    #[test]
    fn lhld_loads_values_at_given_and_following_memory_location_into_registers() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0x592B => 100, 0x592C => 176 })
            .build();
        crate::transfer_instructions::lhld_instruction(&mut state, 0x2B, 0x59);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -80, Register::L => 100 })
                .memory_values(hashmap! { 0x592B => 100, 0x592C => 176 })
                .build(),
        )
    }

    #[test]
    fn lhld_at_max_memory_location_overflows_around_to_retrieving_from_first() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0xFFFF => 89, 0x0000 => 187 })
            .build();
        crate::transfer_instructions::lhld_instruction(&mut state, 0xFF, 0xFF);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -69, Register::L => 89 })
                .memory_values(hashmap! { 0xFFFF => 89, 0x0000 => 187 })
                .build(),
        )
    }

    #[test]
    fn shld_stores_register_values_at_given_and_following_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 106, Register::L => -22 })
            .build();
        crate::transfer_instructions::shld_instruction(&mut state, 0xFF, 0xD3);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 106, Register::L => -22 })
                .memory_values(hashmap! { 0xD3FF => 234, 0xD400 => 106 })
                .build(),
        );
    }

    #[test]
    fn shld_at_max_memory_location_overflows_around_to_storing_at_first() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => -96, Register::L => 69 })
            .build();
        crate::transfer_instructions::shld_instruction(&mut state, 0xFF, 0xFF);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -96, Register::L => 69 })
                .memory_values(hashmap! { 0xFFFF => 69, 0x0000 => 160 })
                .build(),
        );
    }

    #[test]
    fn xchg_exchanges_content_of_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! {
                Register::D => 78,
                Register::E => 69,
                Register::L => 11,
            })
            .build();
        crate::transfer_instructions::xchg_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! {
                    Register::H => 78,
                    Register::E => 11,
                    Register::L => 69,
                })
                .build(),
        );
    }
}
