use std::collections::HashMap;

use maplit::hashmap;
#[cfg(test)]
use mutagen::mutate;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

type RegisterState = HashMap<Register, u8>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ConditionFlag {
    Zero,
    Sign,
    Parity,
    Carry,
    AuxiliaryCarry,
}

struct ConditionFlags {
    zero: bool,
    sign: bool,
    parity: bool,
    carry: bool,
    auxiliary_carry: bool,
}

impl ConditionFlags {
    #[cfg_attr(test, mutate)]
    fn default() -> Self {
        ConditionFlags {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            auxiliary_carry: false,
        }
    }
}

struct State {
    registers: RegisterState,
    condition_flags: ConditionFlags,
}

impl State {
    #[cfg_attr(test, mutate)]
    fn default() -> Self {
        State {
            registers: hashmap! {
                Register::A => 0,
                Register::B => 0,
                Register::C => 0,
                Register::D => 0,
                Register::E => 0,
                Register::H => 0,
                Register::L => 0,
            },
            condition_flags: ConditionFlags::default(),
        }
    }

    #[cfg_attr(test, mutate)]
    fn with_initial_register_state(initial_state: RegisterState) -> Self {
        let mut state = State::default();

        for (register, value) in initial_state {
            state.set_register(register, value);
        }

        state
    }

    #[cfg_attr(test, mutate)]
    fn get_register_state(&self) -> &RegisterState {
        &self.registers
    }

    #[cfg_attr(test, mutate)]
    fn get_condition_flag_value(&self, condition_flag: ConditionFlag) -> bool {
        match condition_flag {
            ConditionFlag::Zero => self.condition_flags.zero,
            ConditionFlag::Sign => self.condition_flags.sign,
            ConditionFlag::Parity => self.condition_flags.parity,
            ConditionFlag::Carry => self.condition_flags.carry,
            ConditionFlag::AuxiliaryCarry => self.condition_flags.auxiliary_carry,
        }
    }

    #[cfg_attr(test, mutate)]
    fn get_condition_flag_state(&self) -> HashMap<ConditionFlag, bool> {
        hashmap! {
            ConditionFlag::Zero => self.get_condition_flag_value(ConditionFlag::Zero),
            ConditionFlag::Sign => self.get_condition_flag_value(ConditionFlag::Sign),
            ConditionFlag::Parity => self.get_condition_flag_value(ConditionFlag::Parity),
            ConditionFlag::Carry => self.get_condition_flag_value(ConditionFlag::Carry),
            ConditionFlag::AuxiliaryCarry => self.get_condition_flag_value(ConditionFlag::AuxiliaryCarry),
        }
    }

    #[cfg_attr(test, mutate)]
    fn get_register_mut(&mut self, register: Register) -> &mut u8 {
        self.registers.get_mut(&register).unwrap()
    }

    #[cfg_attr(test, mutate)]
    fn get_register_value(&mut self, register: Register) -> u8 {
        *self.registers.get(&register).unwrap()
    }

    #[cfg_attr(test, mutate)]
    fn set_register(&mut self, register: Register, value: u8) {
        let register_to_set = self.get_register_mut(register);
        *register_to_set = value;
    }

    #[cfg_attr(test, mutate)]
    fn adjust_register(&mut self, register: Register, relative_adjustment: i16) {
        let register_to_adjust = self.get_register_mut(register);
        *register_to_adjust = (*register_to_adjust as i16 + relative_adjustment) as u8;
    }

    #[cfg_attr(test, mutate)]
    fn set_register_by_function_with_value<F>(&mut self, target_register: Register, value: u8, f: F)
    where
        F: FnOnce(u8, u8) -> u8,
    {
        let target_register_value = self.get_register_value(target_register);
        self.set_register(target_register, f(target_register_value, value));
    }

    #[cfg_attr(test, mutate)]
    fn set_register_by_function_with_register<F>(
        &mut self,
        source_register: Register,
        target_register: Register,
        f: F,
    ) where
        F: FnOnce(u8, u8) -> u8,
    {
        let source_register_value = self.get_register_value(source_register);
        self.set_register_by_function_with_value(target_register, source_register_value, f);
    }

    #[cfg_attr(test, mutate)]
    fn is_bit_set(&self, value: u8, bit_index: u8) -> bool {
        if bit_index >= 8 {
            panic!("Invalid bit index of {}", bit_index);
        }

        let bit_mask = u8::pow(2, bit_index as u32);
        value & bit_mask != 0
    }

    #[cfg_attr(test, mutate)]
    fn get_parity(&self, value: u8) -> bool {
        let mut parity = true;

        for bit_index in 0..=7 {
            if self.is_bit_set(value, bit_index) {
                parity = !parity
            }
        }

        parity
    }

    #[cfg_attr(test, mutate)]
    fn set_condition_flags_based_on_result(&mut self, result: u8) {
        self.condition_flags.zero = result == 0;
        self.condition_flags.sign = self.is_bit_set(result, 7);
        self.condition_flags.parity = self.get_parity(result);
    }
}

#[cfg_attr(test, mutate)]
fn mvi_instruction(state: &mut State, register: Register, data: u8) {
    state.set_register(register, data);
}

#[cfg_attr(test, mutate)]
fn mov_instruction(state: &mut State, from_register: Register, to_register: Register) {
    let from_register_value = state.get_register_value(from_register);
    mvi_instruction(state, to_register, from_register_value);
}

#[cfg_attr(test, mutate)]
fn inr_instruction(state: &mut State, register: Register) {
    state.adjust_register(register, 1);
    let result = state.get_register_value(register);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn dcr_instruction(state: &mut State, register: Register) {
    state.adjust_register(register, -1);
    let result = state.get_register_value(register);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn add_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    adi_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
fn adi_instruction(state: &mut State, data: u8) {
    state.adjust_register(Register::A, data as i16);
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn sub_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    sui_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
fn sui_instruction(state: &mut State, data: u8) {
    state.adjust_register(Register::A, -(data as i16));
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn ana_instruction(state: &mut State, source_register: Register) {
    state.set_register_by_function_with_register(
        source_register,
        Register::A,
        |source_value, target_value| source_value & target_value,
    );
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn ani_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |source_value, target_value| {
        source_value & target_value
    });
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn ora_instruction(state: &mut State, source_register: Register) {
    state.set_register_by_function_with_register(
        source_register,
        Register::A,
        |source_value, target_value| source_value | target_value,
    );
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn oni_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |source_value, target_value| {
        source_value | target_value
    });
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn xra_instruction(state: &mut State, source_register: Register) {
    state.set_register_by_function_with_register(
        source_register,
        Register::A,
        |source_value, target_value| source_value ^ target_value,
    );
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn xni_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |source_value, target_value| {
        source_value ^ target_value
    });
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
fn rlc_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, accumulator_value << 1);
}

#[cfg_attr(test, mutate)]
fn rrc_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, accumulator_value >> 1);
}

#[cfg_attr(test, mutate)]
fn cma_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, !accumulator_value);
}

#[cfg_attr(test, mutate)]
fn cmc_instruction(state: &mut State) {
    state.condition_flags.carry = !state.condition_flags.carry;
}

#[cfg_attr(test, mutate)]
fn stc_instruction(state: &mut State) {
    state.condition_flags.carry = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{Debug, Display};

    fn assert_actual_equals_expected_for_type<T, V>(
        friendly_name: &str,
        type_being_checked: T,
        actual_value: V,
        expected_value: V,
    ) where
        T: Debug,
        V: Eq + Debug + Display,
    {
        assert_eq!(
            actual_value, expected_value,
            "Expected {} {:?} to have value {}, but instead it had value {}",
            friendly_name, type_being_checked, expected_value, actual_value
        );
    }

    fn assert_register_has_value(register: Register, actual_value: u8, expected_value: u8) {
        assert_actual_equals_expected_for_type("register", register, actual_value, expected_value);
    }

    fn assert_values_of_registers(state: &State, expected_register_state: RegisterState) {
        let register_state = state.get_register_state();
        let default_value = 0;

        for (register, actual_value) in register_state {
            match expected_register_state.get(&register) {
                Some(expected_value) => {
                    assert_register_has_value(*register, *actual_value, *expected_value)
                }
                None => assert_register_has_value(*register, *actual_value, default_value),
            }
        }
    }

    fn assert_condition_flag_has_value(
        condition_flag: ConditionFlag,
        actual_value: bool,
        expected_value: bool,
    ) {
        assert_actual_equals_expected_for_type(
            "condition flag",
            condition_flag,
            actual_value,
            expected_value,
        );
    }

    fn assert_values_of_condition_flags(
        state: &State,
        expected_flags: HashMap<ConditionFlag, bool>,
    ) {
        let condition_flags_state = state.get_condition_flag_state();
        let default_value = false;

        for (condition_flag, actual_value) in condition_flags_state {
            match expected_flags.get(&condition_flag) {
                Some(expected_value) => {
                    assert_condition_flag_has_value(condition_flag, actual_value, *expected_value)
                }
                None => {
                    assert_condition_flag_has_value(condition_flag, actual_value, default_value)
                }
            }
        }
    }

    fn assert_state_is_as_expected(
        state: &State,
        expected_register_state: RegisterState,
        expected_flags: HashMap<ConditionFlag, bool>,
    ) {
        assert_values_of_registers(state, expected_register_state);
        assert_values_of_condition_flags(state, expected_flags);
    }

    #[test]
    fn can_get_state_of_all_registers() {
        let state = State::default();
        let register_state = state.get_register_state();
        assert_eq!(register_state.len(), 7);
    }

    #[test]
    fn default_register_state_has_all_default_values() {
        let state = State::default();
        assert_state_is_as_expected(&state, RegisterState::new(), HashMap::new());
    }

    #[test]
    fn can_create_state_with_initial_register_values() {
        let state =
            State::with_initial_register_state(hashmap! { Register::A => 23, Register::C => 34 });
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 23, Register::C => 34 },
            HashMap::new(),
        );
    }

    #[test]
    #[should_panic(expected = "Invalid bit index of 8")]
    fn is_bit_set_panics_when_given_an_invalid_bit_index() {
        let state = State::default();
        state.is_bit_set(255, 8);
    }

    #[test]
    fn mvi_loads_data_into_one_register() {
        let mut state = State::default();
        mvi_instruction(&mut state, Register::A, 64);
        assert_state_is_as_expected(&state, hashmap! { Register::A => 64 }, HashMap::new());
    }

    #[test]
    fn mvi_loads_data_into_multiple_registers() {
        let mut state = State::default();
        mvi_instruction(&mut state, Register::B, 128);
        mvi_instruction(&mut state, Register::D, 255);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 128, Register::D => 255 },
            HashMap::new(),
        );
    }

    #[test]
    fn mov_moves_value_from_one_register_to_another() {
        let mut state = State::with_initial_register_state(hashmap! { Register::H => 99 });
        mov_instruction(&mut state, Register::H, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::H => 99, Register::A => 99 },
            HashMap::new(),
        );
    }

    #[test]
    fn mov_does_nothing_when_both_registers_are_the_same() {
        let mut state = State::with_initial_register_state(hashmap! { Register::L => 121 });
        mov_instruction(&mut state, Register::L, Register::L);
        assert_state_is_as_expected(&state, hashmap! { Register::L => 121 }, HashMap::new());
    }

    #[test]
    fn multiple_mov_can_move_to_multiple_registers() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 218 });
        mov_instruction(&mut state, Register::A, Register::C);
        mov_instruction(&mut state, Register::A, Register::E);
        mov_instruction(&mut state, Register::A, Register::L);
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
    fn inr_increments_default_register_value() {
        let mut state = State::default();
        inr_instruction(&mut state, Register::C);
        assert_state_is_as_expected(&state, hashmap! { Register::C => 1 }, HashMap::new());
    }

    #[test]
    fn inr_increments_existing_register_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::E => 127 });
        inr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 128 },
            hashmap! { ConditionFlag::Sign => true },
        );
    }

    #[test]
    fn dcr_decrements_existing_register_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::E => 127 });
        dcr_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 126 },
            hashmap! { ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn add_adds_the_existing_register_value_to_the_accumulator() {
        let mut state = State::with_initial_register_state(hashmap! { Register::D => 24 });
        add_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::D => 24, Register::A => 24 },
            hashmap! { ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn add_can_add_a_value_to_the_accumulator_multiple_times() {
        let mut state = State::with_initial_register_state(hashmap! { Register::B => 31 });
        add_instruction(&mut state, Register::B);
        add_instruction(&mut state, Register::B);
        add_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 31, Register::A => 93 },
            HashMap::new(),
        )
    }

    #[test]
    fn add_can_add_values_from_multiple_different_registers() {
        let mut state = State::with_initial_register_state(hashmap! {
            Register::B => 11,
            Register::C => 13,
            Register::D => 15,
        });
        add_instruction(&mut state, Register::B);
        add_instruction(&mut state, Register::C);
        add_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            hashmap! {
                Register::B => 11,
                Register::C => 13,
                Register::D => 15,
                Register::A => 39,
            },
            hashmap! { ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn add_adds_the_value_onto_any_existing_value_in_the_accumulator() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 102, Register::E => 95 });
        add_instruction(&mut state, Register::E);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::E => 95, Register::A => 197 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn add_doubles_the_accumulator_value_if_it_is_given_as_the_register() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 63 });
        add_instruction(&mut state, Register::A);
        add_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 252 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn add_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::default();
        add_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn adi_adds_the_given_value_onto_the_default_accumulator_value() {
        let mut state = State::default();
        adi_instruction(&mut state, 128);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 128 },
            hashmap! { ConditionFlag::Sign => true },
        );
    }

    #[test]
    fn adi_adds_the_given_value_onto_any_existing_value_in_the_accumulator() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 90 });
        adi_instruction(&mut state, 37);
        assert_state_is_as_expected(&state, hashmap! { Register::A => 127 }, HashMap::new());
    }

    #[test]
    fn adi_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::default();
        adi_instruction(&mut state, 0);
        assert_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn sub_subtracts_the_existing_register_value_from_the_accumulator() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 255, Register::D => 24 });
        sub_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::D => 24, Register::A => 231 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn sub_can_subtract_a_value_from_the_accumulator_multiple_times() {
        let mut state =
            State::with_initial_register_state(hashmap! { Register::A => 253, Register::B => 42 });
        sub_instruction(&mut state, Register::B);
        sub_instruction(&mut state, Register::B);
        sub_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 42, Register::A => 127 },
            HashMap::new(),
        )
    }

    #[test]
    fn sub_can_subtract_values_from_multiple_different_registers() {
        let mut state = State::with_initial_register_state(hashmap! {
            Register::A => 255,
            Register::B => 11,
            Register::C => 13,
            Register::D => 15,
        });
        sub_instruction(&mut state, Register::B);
        sub_instruction(&mut state, Register::C);
        sub_instruction(&mut state, Register::D);
        assert_state_is_as_expected(
            &state,
            hashmap! {
                Register::B => 11,
                Register::C => 13,
                Register::D => 15,
                Register::A => 216,
            },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        )
    }

    #[test]
    fn sub_zeroes_the_accumulator_value_if_it_is_given_as_the_register() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 63 });
        sub_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0 },
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn sui_subtracts_the_given_value_from_any_existing_value_in_the_accumulator() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 183 });
        sui_instruction(&mut state, 55);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 128 },
            hashmap! { ConditionFlag::Sign => true },
        );
    }

    #[test]
    fn sui_sets_condition_flag_zero_to_true_if_result_is_zero() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 173 });
        sui_instruction(&mut state, 173);
        assert_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn ana_logically_ands_the_accumulator_with_the_value_of_the_given_register() {
        let mut state = State::with_initial_register_state(
            hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 },
        );
        ana_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 0b01000101, Register::A => 0b01000100 },
            hashmap! { ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn ana_applied_to_an_existing_accumulator_value_does_nothing() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b10100110 });
        ana_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b10100110 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn ani_logically_ands_the_accumulator_with_the_given_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
        ani_instruction(&mut state, 0b01100011);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b01000010 },
            hashmap! { ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn ora_logically_ors_the_accumulator_with_the_value_of_the_given_register() {
        let mut state = State::with_initial_register_state(
            hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 },
        );
        ora_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 0b01000101, Register::A => 0b01010101 },
            hashmap! { ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn ora_applied_to_an_existing_accumulator_value_does_nothing() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b10100110 });
        ora_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b10100110 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn oni_logically_ors_the_accumulator_with_the_given_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
        oni_instruction(&mut state, 0b01100011);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b11100111 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn xra_logically_xors_the_accumulator_with_the_value_of_the_given_register() {
        let mut state = State::with_initial_register_state(
            hashmap! { Register::A => 0b01010100, Register::B => 0b01000101 },
        );
        xra_instruction(&mut state, Register::B);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::B => 0b01000101, Register::A => 0b00010001 },
            hashmap! { ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn xra_applied_to_an_existing_accumulator_value_zeroes_that_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b10100110 });
        xra_instruction(&mut state, Register::A);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b00000000 },
            hashmap! { ConditionFlag::Zero => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn xni_logically_xors_the_accumulator_with_the_given_value() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
        xni_instruction(&mut state, 0b01100011);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b10100101 },
            hashmap! { ConditionFlag::Sign => true, ConditionFlag::Parity => true },
        );
    }

    #[test]
    fn rlc_shifts_the_accumulator_value_left() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
        rlc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b10001100 },
            HashMap::new(),
        );
    }

    #[test]
    fn rrc_shifts_the_accumulator_value_left() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b01100011 });
        rrc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b00110001 },
            HashMap::new(),
        );
    }

    #[test]
    fn cma_complements_the_value_in_the_accumulator() {
        let mut state = State::with_initial_register_state(hashmap! { Register::A => 0b11000110 });
        cma_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            hashmap! { Register::A => 0b00111001 },
            HashMap::new(),
        );
    }

    #[test]
    fn cmc_inverts_the_carry_flag() {
        let mut state = State::default();
        cmc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Carry => true },
        );
        cmc_instruction(&mut state);
        assert_state_is_as_expected(&state, RegisterState::new(), HashMap::new());
    }

    #[test]
    fn stc_sets_the_carry_flag_to_true() {
        let mut state = State::default();
        stc_instruction(&mut state);
        assert_state_is_as_expected(
            &state,
            RegisterState::new(),
            hashmap! { ConditionFlag::Carry => true },
        );
    }
}
