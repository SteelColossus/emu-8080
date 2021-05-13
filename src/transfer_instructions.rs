use crate::{Register, RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn mvi_instruction(state: &mut State, register: Register, data: i8) {
    state.set_register(register, data);
}

#[cfg_attr(test, mutate)]
pub fn mvi_mem_instruction(state: &mut State, data: i8) {
    let memory_address = RegisterPair::HL.get_full_value(&state);
    state.set_value_at_memory_location(memory_address, data as u8);
}

#[cfg_attr(test, mutate)]
pub fn mov_instruction(state: &mut State, from_register: Register, to_register: Register) {
    let from_register_value = state.get_register_value(from_register);
    mvi_instruction(state, to_register, from_register_value);
}

#[cfg_attr(test, mutate)]
pub fn mov_from_mem_instruction(state: &mut State, register: Register) {
    let memory_address = RegisterPair::HL.get_full_value(&state);
    let data = state.get_value_at_memory_location(memory_address);
    mvi_instruction(state, register, data as i8);
}

#[cfg_attr(test, mutate)]
pub fn mov_to_mem_instruction(state: &mut State, register: Register) {
    let data = state.get_register_value(register);
    mvi_mem_instruction(state, data as i8);
}

#[cfg_attr(test, mutate)]
pub fn lxi_instruction(
    state: &mut State,
    register_pair: RegisterPair,
    low_data: i8,
    high_data: i8,
) {
    register_pair.set_low_high_value(state, low_data, high_data);
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
pub fn ldax_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::HL || register_pair == RegisterPair::SP {
        panic!(
            "The register pair {:?} is not supported by the LDAX operation",
            register_pair
        );
    }

    let memory_address = register_pair.get_full_value(&state);
    let value = state.get_value_at_memory_location(memory_address);
    state.set_register(Register::A, value as i8);
}

#[cfg_attr(test, mutate)]
pub fn stax_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::HL || register_pair == RegisterPair::SP {
        panic!(
            "The register pair {:?} is not supported by the STAX operation",
            register_pair
        );
    }

    let value = state.get_register_value(Register::A);
    let memory_address = register_pair.get_full_value(&state);
    state.set_value_at_memory_location(memory_address, value as u8);
}

#[cfg_attr(test, mutate)]
pub fn xchg_instruction(state: &mut State) {
    state.exchange_register_values(Register::D, Register::H);
    state.exchange_register_values(Register::E, Register::L);
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{Register, RegisterPair, State, StateBuilder};
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
    fn mvi_mem_loads_data_into_memory_location_determined_by_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 62, Register::L => 13 })
            .build();
        crate::transfer_instructions::mvi_mem_instruction(&mut state, -64);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 62, Register::L => 13 })
                .memory_values(hashmap! { 0x3E0D => 192 })
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
    fn mov_from_mem_moves_from_memory_to_given_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 80, Register::L => -103 })
            .memory_values(hashmap! { 0x5099 => 187 })
            .build();
        crate::transfer_instructions::mov_from_mem_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::H => 80, Register::L => -103, Register::B => -69 },
                )
                .memory_values(hashmap! { 0x5099 => 187 })
                .build(),
        );
    }

    #[test]
    fn mov_from_mem_can_move_with_default_registers() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0x0000 => 128 })
            .build();
        crate::transfer_instructions::mov_from_mem_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => -128 })
                .memory_values(hashmap! { 0x0000 => 128 })
                .build(),
        );
    }

    #[test]
    fn mov_from_mem_can_overwrite_with_default_memory() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => -43, Register::L => 75, Register::E => 48 })
            .build();
        crate::transfer_instructions::mov_from_mem_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -43, Register::L => 75 })
                .build(),
        );
    }

    #[test]
    fn mov_from_mem_can_overwrite_register_used_for_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => -27, Register::L => 4 })
            .memory_values(hashmap! { 0xE503 => 76, 0xE504 => 179, 0xE505 => 148 })
            .build();
        crate::transfer_instructions::mov_from_mem_instruction(&mut state, Register::L);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => -27, Register::L => -77 })
                .memory_values(hashmap! { 0xE503 => 76, 0xE504 => 179, 0xE505 => 148 })
                .build(),
        );
    }

    #[test]
    fn mov_to_mem_moves_from_given_register_to_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 117, Register::L => 35, Register::C => 63 })
            .build();
        crate::transfer_instructions::mov_to_mem_instruction(&mut state, Register::C);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::H => 117, Register::L => 35, Register::C => 63 },
                )
                .memory_values(hashmap! { 0x7523 => 63 })
                .build(),
        );
    }

    #[test]
    fn mov_to_mem_can_overwrite_with_default_register() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 72, Register::L => -56 })
            .memory_values(hashmap! { 0x48C7 => 53, 0x48C8 => 235, 0x48C9 => 159 })
            .build();
        crate::transfer_instructions::mov_to_mem_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 72, Register::L => -56 })
                .memory_values(hashmap! { 0x48C7 => 53, 0x48C9 => 159 })
                .build(),
        );
    }

    #[test]
    fn mov_to_mem_can_move_to_memory_from_default_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => 18 })
            .build();
        crate::transfer_instructions::mov_to_mem_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 18 })
                .memory_values(hashmap! { 0x0000 => 18 })
                .build(),
        );
    }

    #[test]
    fn lxi_loads_the_given_data_into_the_given_register_pair() {
        let mut state = State::default();
        crate::transfer_instructions::lxi_instruction(&mut state, RegisterPair::BC, 96, -29);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => -29, Register::C => 96 })
                .build(),
        );
    }

    #[test]
    fn lxi_loads_the_given_data_into_the_stack_pointer() {
        let mut state = State::default();
        crate::transfer_instructions::lxi_instruction(&mut state, RegisterPair::SP, -57, 77);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default().stack_pointer(0x4DC7).build(),
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
    fn ldax_loads_memory_location_content_into_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 43, Register::E => -26 })
            .memory_values(hashmap! { 0x2BE5 => 27, 0x2BE6 => 107, 0x2BE7 => 243})
            .build();
        crate::transfer_instructions::ldax_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::D => 43, Register::E => -26, Register::A => 107 },
                )
                .memory_values(hashmap! { 0x2BE5 => 27, 0x2BE6 => 107, 0x2BE7 => 243})
                .build(),
        );
    }

    #[test]
    fn ldax_can_load_with_default_register_pair_values() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0x0000 => 101 })
            .build();
        crate::transfer_instructions::ldax_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 101 })
                .memory_values(hashmap! { 0x0000 => 101 })
                .build(),
        );
    }

    #[test]
    fn ldax_overwrites_accumulator_with_default_memory() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => 47, Register::C => 31, Register::A => -9 })
            .build();
        crate::transfer_instructions::ldax_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 47, Register::C => 31 })
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The register pair HL is not supported by the LDAX operation")]
    fn ldax_does_not_support_the_hl_register_pair() {
        let mut state = State::default();
        crate::transfer_instructions::ldax_instruction(&mut state, RegisterPair::HL);
    }

    #[test]
    #[should_panic(expected = "The register pair SP is not supported by the LDAX operation")]
    fn ldax_does_not_support_the_sp_register_pair() {
        let mut state = State::default();
        crate::transfer_instructions::ldax_instruction(&mut state, RegisterPair::SP);
    }

    #[test]
    fn stax_stores_accumulator_value_into_the_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => -96, Register::E => 17, Register::A => -34 })
            .build();
        crate::transfer_instructions::stax_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::D => -96, Register::E => 17, Register::A => -34 },
                )
                .memory_values(hashmap! { 0xA011 => 222 })
                .build(),
        );
    }

    #[test]
    fn stax_can_store_with_default_register_pair_values() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 107 })
            .build();
        crate::transfer_instructions::stax_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 107 })
                .memory_values(hashmap! { 0x0000 => 107 })
                .build(),
        );
    }

    #[test]
    fn stax_overwrites_memory_location_with_default_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => -38, Register::C => 15 })
            .memory_values(hashmap! { 0xDA0F => 174 })
            .build();
        crate::transfer_instructions::stax_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => -38, Register::C => 15 })
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The register pair HL is not supported by the STAX operation")]
    fn stax_does_not_support_the_hl_register_pair() {
        let mut state = State::default();
        crate::transfer_instructions::stax_instruction(&mut state, RegisterPair::HL);
    }

    #[test]
    #[should_panic(expected = "The register pair SP is not supported by the STAX operation")]
    fn stax_does_not_support_the_sp_register_pair() {
        let mut state = State::default();
        crate::transfer_instructions::stax_instruction(&mut state, RegisterPair::SP);
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