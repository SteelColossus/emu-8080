use crate::{Register, RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn push_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::SP {
        panic!("The register pair {register_pair} is not supported by the PUSH operation");
    }

    let sp_minus_one = state.stack_pointer.wrapping_sub(1);
    let sp_minus_two = state.stack_pointer.wrapping_sub(2);
    let (register_pair_low, register_pair_high) = state.low_high_rp_value(register_pair);
    state.memory[sp_minus_one as usize] = register_pair_high;
    state.memory[sp_minus_two as usize] = register_pair_low;
    state.stack_pointer = sp_minus_two;
}

#[cfg_attr(test, mutate)]
pub fn push_psw_instruction(state: &mut State) {
    let sp_minus_one = state.stack_pointer.wrapping_sub(1);
    let sp_minus_two = state.stack_pointer.wrapping_sub(2);
    let accumulator_value = state.registers[Register::A];
    state.memory[sp_minus_one as usize] = accumulator_value;
    let condition_flag_byte = state.condition_flag_byte();
    state.memory[sp_minus_two as usize] = condition_flag_byte;
    state.stack_pointer = sp_minus_two;
}

#[cfg_attr(test, mutate)]
pub fn pop_instruction(state: &mut State, register_pair: RegisterPair) {
    if register_pair == RegisterPair::SP {
        panic!("The register pair {register_pair} is not supported by the POP operation");
    }

    let sp_plus_one = state.stack_pointer.wrapping_add(1);
    let sp_plus_two = state.stack_pointer.wrapping_add(2);
    let value_for_register_pair_low = state.memory[state.stack_pointer as usize];
    let value_for_register_pair_high = state.memory[sp_plus_one as usize];
    state.set_low_high_rp_value(
        register_pair,
        value_for_register_pair_low,
        value_for_register_pair_high,
    );
    state.stack_pointer = sp_plus_two;
}

#[cfg_attr(test, mutate)]
pub fn pop_psw_instruction(state: &mut State) {
    let sp_plus_one = state.stack_pointer.wrapping_add(1);
    let sp_plus_two = state.stack_pointer.wrapping_add(2);
    state.set_condition_flag_byte(state.stack_pointer);
    let accumulator_value = state.memory[sp_plus_one as usize];
    state.registers[Register::A] = accumulator_value;
    state.stack_pointer = sp_plus_two;
}

#[cfg_attr(test, mutate)]
pub fn xthl_instruction(state: &mut State) {
    let sp_plus_one = state.stack_pointer.wrapping_add(1);
    let low_memory_value = state.memory[state.stack_pointer as usize];
    let high_memory_value = state.memory[sp_plus_one as usize];
    let l_register_value = state.registers[Register::L];
    let h_register_value = state.registers[Register::H];
    state.registers[Register::L] = low_memory_value;
    state.memory[state.stack_pointer as usize] = l_register_value;
    state.registers[Register::H] = high_memory_value;
    state.memory[sp_plus_one as usize] = h_register_value;
}

#[cfg_attr(test, mutate)]
pub fn sphl_instruction(state: &mut State) {
    state.stack_pointer = state.full_rp_value(RegisterPair::HL);
}

#[cfg_attr(test, mutate)]
pub fn ei_instruction(state: &mut State) {
    state.are_interrupts_enabled = true;
}

#[cfg_attr(test, mutate)]
pub fn di_instruction(state: &mut State) {
    state.are_interrupts_enabled = false;
}

#[cfg_attr(test, mutate)]
pub fn hlt_instruction(state: &mut State) {
    state.is_halted = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::{ConditionFlag, Register, StateBuilder};
    use maplit::hashmap;

    #[test]
    fn push_sets_stack_pointer_values_based_on_given_register_pair() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::B => 221, Register::C => 101 })
            .stack_pointer(0xF028)
            .build();
        push_instruction(&mut state, RegisterPair::BC);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::B => 221, Register::C => 101 })
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
    fn push_psw_pushes_the_condition_flag_contents_into_memory() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::A => 167 })
            .condition_flag_values(hashmap! {
                ConditionFlag::Zero => true,
                ConditionFlag::Sign => false,
                ConditionFlag::Parity => true,
                ConditionFlag::Carry => false,
                ConditionFlag::AuxiliaryCarry => true,
            })
            .stack_pointer(0x6833)
            .build();
        push_psw_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::A => 167 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => true,
                    ConditionFlag::Sign => false,
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => false,
                    ConditionFlag::AuxiliaryCarry => true,
                })
                .stack_pointer(0x6831)
                .memory_values(hashmap! { 0x6831 => 0b01010110, 0x6832 => 167 })
                .build(),
        );
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
                .register_values(hashmap! { Register::D => 204, Register::E => 40 })
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
    fn pop_psw_pops_the_condition_flag_contents_from_memory() {
        let mut state = StateBuilder::default()
            .memory_values(
                hashmap! { 0x8279 => 194, 0x827A => 0b10101101, 0x827B => 154, 0x827C => 215 },
            )
            .stack_pointer(0x827A)
            .build();
        pop_psw_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .memory_values(
                    hashmap! { 0x8279 => 194, 0x827A => 0b10101101, 0x827B => 154, 0x827C => 215 },
                )
                .stack_pointer(0x827C)
                .register_values(hashmap! { Register::A => 154 })
                .condition_flag_values(hashmap! {
                    ConditionFlag::Zero => false,
                    ConditionFlag::Sign => true,
                    ConditionFlag::Parity => true,
                    ConditionFlag::Carry => true,
                    ConditionFlag::AuxiliaryCarry => false,
                })
                .build(),
        );
    }

    #[test]
    fn xthl_exchanges_the_stack_top_with_the_register_values() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 162, Register::L => 127 })
            .stack_pointer(0xD39B)
            .memory_values(hashmap! { 0xD39B => 213, 0xD39C => 86 })
            .build();
        xthl_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 86, Register::L => 213 })
                .stack_pointer(0xD39B)
                .memory_values(hashmap! { 0xD39B => 127, 0xD39C => 162 })
                .build(),
        );
    }

    #[test]
    fn sphl_sets_the_stack_pointer_to_register_values() {
        let mut state = StateBuilder::default()
            .register_values(hashmap! { Register::H => 10, Register::L => 156 })
            .build();
        sphl_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            &StateBuilder::default()
                .register_values(hashmap! { Register::H => 10, Register::L => 156 })
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
            &StateBuilder::default().interrupts_enabled(true).build(),
        )
    }

    #[test]
    fn di_sets_interrupts_as_disabled() {
        let mut state = StateBuilder::default().interrupts_enabled(true).build();
        di_instruction(&mut state);
        assert_state_is_as_expected(&state, &State::default())
    }

    #[test]
    fn hlt_halts_the_processor() {
        let mut state = State::default();
        assert_state_is_as_expected(&state, &StateBuilder::default().halted(false).build());
        hlt_instruction(&mut state);
        assert_state_is_as_expected(&state, &StateBuilder::default().halted(true).build());
    }
}
