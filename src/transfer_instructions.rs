use crate::{Register, RegisterPair, State, bit_operations};
// #[cfg(test)]
// use mutagen::mutate;

// #[cfg_attr(test, mutate)]
pub fn mov_instruction(state: &mut State, from_register: Register, to_register: Register) {
    let from_register_value = state.registers[from_register];
    mvi_instruction(state, to_register, from_register_value);
}

// #[cfg_attr(test, mutate)]
pub fn mov_from_mem_instruction(state: &mut State, register: Register) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    let data = state.memory[memory_address as usize];
    mvi_instruction(state, register, data);
}

// #[cfg_attr(test, mutate)]
pub fn mov_to_mem_instruction(state: &mut State, register: Register) {
    let data = state.registers[register];
    mvi_mem_instruction(state, data);
}

// #[cfg_attr(test, mutate)]
pub fn mvi_instruction(state: &mut State, register: Register, data: u8) {
    state.registers[register] = data;
}

// #[cfg_attr(test, mutate)]
pub fn mvi_mem_instruction(state: &mut State, data: u8) {
    let memory_address = state.full_rp_value(RegisterPair::HL);
    state.memory[memory_address as usize] = data;
}

// #[cfg_attr(test, mutate)]
pub fn lxi_instruction(
    state: &mut State,
    register_pair: RegisterPair,
    low_data: u8,
    high_data: u8,
) {
    state.set_low_high_rp_value(register_pair, low_data, high_data);
}

// #[cfg_attr(test, mutate)]
pub fn lda_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let memory_address = bit_operations::concat_low_high_bytes(low_data, high_data);
    let memory_location_value = state.memory[memory_address as usize];
    state.registers[Register::A] = memory_location_value;
}

// #[cfg_attr(test, mutate)]
pub fn sta_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let memory_address = bit_operations::concat_low_high_bytes(low_data, high_data);
    let accumulator_value = state.registers[Register::A];
    state.memory[memory_address as usize] = accumulator_value;
}

// #[cfg_attr(test, mutate)]
pub fn lhld_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let first_memory_address = bit_operations::concat_low_high_bytes(low_data, high_data);
    let second_memory_address = first_memory_address.wrapping_add(1);
    let first_memory_value = state.memory[first_memory_address as usize];
    let second_memory_value = state.memory[second_memory_address as usize];
    state.registers[Register::L] = first_memory_value;
    state.registers[Register::H] = second_memory_value;
}

// #[cfg_attr(test, mutate)]
pub fn shld_instruction(state: &mut State, low_data: u8, high_data: u8) {
    let first_memory_address = bit_operations::concat_low_high_bytes(low_data, high_data);
    let second_memory_address = first_memory_address.wrapping_add(1);
    let h_register_value = state.registers[Register::H];
    let l_register_value = state.registers[Register::L];
    state.memory[first_memory_address as usize] = l_register_value;
    state.memory[second_memory_address as usize] = h_register_value;
}

// #[cfg_attr(test, mutate)]
pub fn ldax_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::HL || register_pair == RegisterPair::SP {
        panic!("The register pair {register_pair} is not supported by the LDAX operation");
    }

    let memory_address = state.full_rp_value(register_pair);
    let value = state.memory[memory_address as usize];
    state.registers[Register::A] = value;
}

// #[cfg_attr(test, mutate)]
pub fn stax_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::HL || register_pair == RegisterPair::SP {
        panic!("The register pair {register_pair} is not supported by the STAX operation");
    }

    let value = state.registers[Register::A];
    let memory_address = state.full_rp_value(register_pair);
    state.memory[memory_address as usize] = value;
}

// #[cfg_attr(test, mutate)]
pub fn xchg_instruction(state: &mut State) {
    state.exchange_register_values(Register::D, Register::H);
    state.exchange_register_values(Register::E, Register::L);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateBuilder;
    use crate::base_test_functions::assert_state_is_as_expected;
    use maplit::hashmap;

    #[test]
    fn mov_moves_value_from_one_register_to_another() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 99 })
            .build();
        mov_instruction(&mut state, Register::H, Register::A);
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
        mov_instruction(&mut state, Register::L, Register::L);
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
        mov_instruction(&mut state, Register::A, Register::C);
        mov_instruction(&mut state, Register::A, Register::E);
        mov_instruction(&mut state, Register::A, Register::L);
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
            .register_values(hashmap! { Register::H => 80, Register::L => 153 })
            .memory_values(hashmap! { 0x5099 => 187 })
            .build();
        mov_from_mem_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::H => 80, Register::L => 153, Register::B => 187 },
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
        mov_from_mem_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::D => 128 })
                .memory_values(hashmap! { 0x0000 => 128 })
                .build(),
        );
    }

    #[test]
    fn mov_from_mem_can_overwrite_with_default_memory() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 213, Register::L => 75, Register::E => 48 })
            .build();
        mov_from_mem_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 213, Register::L => 75 })
                .build(),
        );
    }

    #[test]
    fn mov_from_mem_can_overwrite_register_used_for_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 229, Register::L => 4 })
            .memory_values(hashmap! { 0xE503 => 76, 0xE504 => 179, 0xE505 => 148 })
            .build();
        mov_from_mem_instruction(&mut state, Register::L);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 229, Register::L => 179 })
                .memory_values(hashmap! { 0xE503 => 76, 0xE504 => 179, 0xE505 => 148 })
                .build(),
        );
    }

    #[test]
    fn mov_to_mem_moves_from_given_register_to_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 117, Register::L => 35, Register::C => 63 })
            .build();
        mov_to_mem_instruction(&mut state, Register::C);
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
            .register_values(hashmap! { Register::H => 72, Register::L => 200 })
            .memory_values(hashmap! { 0x48C7 => 53, 0x48C8 => 235, 0x48C9 => 159 })
            .build();
        mov_to_mem_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 72, Register::L => 200 })
                .memory_values(hashmap! { 0x48C7 => 53, 0x48C9 => 159 })
                .build(),
        );
    }

    #[test]
    fn mov_to_mem_can_move_to_memory_from_default_registers() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => 18 })
            .build();
        mov_to_mem_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 18 })
                .memory_values(hashmap! { 0x0000 => 18 })
                .build(),
        );
    }

    #[test]
    fn mvi_loads_data_into_one_register() {
        let mut state = State::default();
        mvi_instruction(&mut state, Register::A, 64);
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
        mvi_instruction(&mut state, Register::B, 1);
        mvi_instruction(&mut state, Register::D, 127);
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
        mvi_mem_instruction(&mut state, 192);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 62, Register::L => 13 })
                .memory_values(hashmap! { 0x3E0D => 192 })
                .build(),
        );
    }

    #[test]
    fn lxi_loads_the_given_data_into_the_given_register_pair() {
        let mut state = State::default();
        lxi_instruction(&mut state, RegisterPair::BC, 96, 227);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 227, Register::C => 96 })
                .build(),
        );
    }

    #[test]
    fn lxi_loads_the_given_data_into_the_stack_pointer() {
        let mut state = State::default();
        lxi_instruction(&mut state, RegisterPair::SP, 199, 77);
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
        lda_instruction(&mut state, 0x40, 0x00);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 214 })
                .memory_values(hashmap! { 0x0040 => 214 })
                .build(),
        );
    }

    #[test]
    fn sta_stores_the_accumulator_value_into_the_given_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 214 })
            .build();
        sta_instruction(&mut state, 0x99, 0x01);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 214 })
                .memory_values(hashmap! { 0x0199 => 214 })
                .build(),
        );
    }

    #[test]
    fn lhld_loads_values_at_given_and_following_memory_location_into_registers() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0x592B => 100, 0x592C => 176 })
            .build();
        lhld_instruction(&mut state, 0x2B, 0x59);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 176, Register::L => 100 })
                .memory_values(hashmap! { 0x592B => 100, 0x592C => 176 })
                .build(),
        )
    }

    #[test]
    fn lhld_at_max_memory_location_overflows_around_to_retrieving_from_first() {
        let mut state = StateBuilder::default()
            .memory_values(hashmap! { 0xFFFF => 89, 0x0000 => 187 })
            .build();
        lhld_instruction(&mut state, 0xFF, 0xFF);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 187, Register::L => 89 })
                .memory_values(hashmap! { 0xFFFF => 89, 0x0000 => 187 })
                .build(),
        )
    }

    #[test]
    fn shld_stores_register_values_at_given_and_following_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 106, Register::L => 234 })
            .build();
        shld_instruction(&mut state, 0xFF, 0xD3);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 106, Register::L => 234 })
                .memory_values(hashmap! { 0xD3FF => 234, 0xD400 => 106 })
                .build(),
        );
    }

    #[test]
    fn shld_at_max_memory_location_overflows_around_to_storing_at_first() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 160, Register::L => 69 })
            .build();
        shld_instruction(&mut state, 0xFF, 0xFF);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 160, Register::L => 69 })
                .memory_values(hashmap! { 0xFFFF => 69, 0x0000 => 160 })
                .build(),
        );
    }

    #[test]
    fn ldax_loads_memory_location_content_into_the_accumulator() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 43, Register::E => 230 })
            .memory_values(hashmap! { 0x2BE5 => 27, 0x2BE6 => 107, 0x2BE7 => 243})
            .build();
        ldax_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::D => 43, Register::E => 230, Register::A => 107 },
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
        ldax_instruction(&mut state, RegisterPair::BC);
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
            .register_values(hashmap! { Register::B => 47, Register::C => 31, Register::A => 247 })
            .build();
        ldax_instruction(&mut state, RegisterPair::BC);
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
        ldax_instruction(&mut state, RegisterPair::HL);
    }

    #[test]
    #[should_panic(expected = "The register pair SP is not supported by the LDAX operation")]
    fn ldax_does_not_support_the_sp_register_pair() {
        let mut state = State::default();
        ldax_instruction(&mut state, RegisterPair::SP);
    }

    #[test]
    fn stax_stores_accumulator_value_into_the_memory_location() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::D => 160, Register::E => 17, Register::A => 222 })
            .build();
        stax_instruction(&mut state, RegisterPair::DE);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(
                    hashmap! { Register::D => 160, Register::E => 17, Register::A => 222 },
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
        stax_instruction(&mut state, RegisterPair::DE);
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
            .register_values(hashmap! { Register::B => 218, Register::C => 15 })
            .memory_values(hashmap! { 0xDA0F => 174 })
            .build();
        stax_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 218, Register::C => 15 })
                .build(),
        );
    }

    #[test]
    #[should_panic(expected = "The register pair HL is not supported by the STAX operation")]
    fn stax_does_not_support_the_hl_register_pair() {
        let mut state = State::default();
        stax_instruction(&mut state, RegisterPair::HL);
    }

    #[test]
    #[should_panic(expected = "The register pair SP is not supported by the STAX operation")]
    fn stax_does_not_support_the_sp_register_pair() {
        let mut state = State::default();
        stax_instruction(&mut state, RegisterPair::SP);
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
        xchg_instruction(&mut state);
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
