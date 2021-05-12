pub mod arithmetic_instructions;
#[cfg(test)]
pub mod base_test_functions;
pub mod branch_instructions;
pub mod logical_instructions;
pub mod stack_instructions;
pub mod transfer_instructions;

use maplit::hashmap;
#[cfg(test)]
use mutagen::mutate;
use std::collections::HashMap;

pub mod bit_operations {
    #[cfg(test)]
    use mutagen::mutate;

    #[cfg_attr(test, mutate)]
    pub fn is_bit_set(value: i8, bit_index: u8) -> bool {
        if bit_index >= 8 {
            panic!("Invalid bit index of {}", bit_index);
        }

        let shifted_value = value >> bit_index;
        shifted_value & 0b00000001 != 0
    }

    #[cfg_attr(test, mutate)]
    pub fn get_value_with_bit_set(value: i8, bit_index: u8, bit_flag: bool) -> i8 {
        if bit_index >= 8 {
            panic!("Invalid bit index of {}", bit_index);
        }

        let bit_mask = 1 << bit_index;
        let bit_value_mask = if bit_flag { bit_mask } else { 0b00000000 };
        value & !bit_mask | bit_value_mask
    }

    #[cfg_attr(test, mutate)]
    pub fn get_parity(value: i8) -> bool {
        let mut parity = true;

        for bit_index in 0..=7 {
            if is_bit_set(value, bit_index) {
                parity = !parity
            }
        }

        parity
    }

    #[cfg_attr(test, mutate)]
    pub fn concat_low_high_bytes(low_byte: u8, high_byte: u8) -> u16 {
        u16::from(high_byte) << 8 | u16::from(low_byte)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub type RegisterState = HashMap<Register, i8>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ConditionFlag {
    Zero,
    Sign,
    Parity,
    Carry,
    AuxiliaryCarry,
}

#[derive(Clone)]
pub struct ConditionFlags {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub auxiliary_carry: bool,
}

impl Default for ConditionFlags {
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

impl ConditionFlags {
    #[cfg_attr(test, mutate)]
    fn get_mut(&mut self, condition_flag: ConditionFlag) -> &mut bool {
        match condition_flag {
            ConditionFlag::Zero => &mut self.zero,
            ConditionFlag::Sign => &mut self.sign,
            ConditionFlag::Parity => &mut self.parity,
            ConditionFlag::Carry => &mut self.carry,
            ConditionFlag::AuxiliaryCarry => &mut self.auxiliary_carry,
        }
    }

    #[cfg_attr(test, mutate)]
    pub fn get_value(&self, condition_flag: ConditionFlag) -> bool {
        match condition_flag {
            ConditionFlag::Zero => self.zero,
            ConditionFlag::Sign => self.sign,
            ConditionFlag::Parity => self.parity,
            ConditionFlag::Carry => self.carry,
            ConditionFlag::AuxiliaryCarry => self.auxiliary_carry,
        }
    }

    #[cfg_attr(test, mutate)]
    pub fn set_value(&mut self, condition_flag: ConditionFlag, value: bool) {
        let flag = self.get_mut(condition_flag);
        *flag = value;
    }
}

const MEMORY_SIZE: usize = u16::MAX as usize + 1;

pub struct State {
    registers: RegisterState,
    pub condition_flags: ConditionFlags,
    pub program_counter: u16,
    pub stack_pointer: u16,
    memory: [u8; MEMORY_SIZE],
}

impl Default for State {
    #[cfg_attr(test, mutate)]
    fn default() -> Self {
        StateBuilder::default().build()
    }
}

impl State {
    #[cfg_attr(test, mutate)]
    pub fn get_register_state(&self) -> &RegisterState {
        &self.registers
    }

    #[cfg_attr(test, mutate)]
    pub fn get_condition_flag_value(&self, condition_flag: ConditionFlag) -> bool {
        self.condition_flags.get_value(condition_flag)
    }

    #[cfg_attr(test, mutate)]
    pub fn get_condition_flag_state(&self) -> HashMap<ConditionFlag, bool> {
        hashmap! {
            ConditionFlag::Zero => self.get_condition_flag_value(ConditionFlag::Zero),
            ConditionFlag::Sign => self.get_condition_flag_value(ConditionFlag::Sign),
            ConditionFlag::Parity => self.get_condition_flag_value(ConditionFlag::Parity),
            ConditionFlag::Carry => self.get_condition_flag_value(ConditionFlag::Carry),
            ConditionFlag::AuxiliaryCarry => self.get_condition_flag_value(ConditionFlag::AuxiliaryCarry),
        }
    }

    #[cfg_attr(test, mutate)]
    fn get_register_mut(&mut self, register: Register) -> &mut i8 {
        self.registers.get_mut(&register).unwrap()
    }

    #[cfg_attr(test, mutate)]
    pub fn get_register_value(&self, register: Register) -> i8 {
        *self.registers.get(&register).unwrap()
    }

    #[cfg_attr(test, mutate)]
    pub fn set_register(&mut self, register: Register, value: i8) {
        let register_to_set = self.get_register_mut(register);
        *register_to_set = value;
    }

    #[cfg_attr(test, mutate)]
    pub fn get_value_at_memory_location(&self, memory_address: u16) -> u8 {
        self.memory[memory_address as usize]
    }

    #[cfg_attr(test, mutate)]
    pub fn set_value_at_memory_location(&mut self, memory_address: u16, value: u8) {
        self.memory[memory_address as usize] = value;
    }

    #[cfg_attr(test, mutate)]
    pub fn increase_register(&mut self, register: Register, relative_value: i8) -> bool {
        let register_to_adjust = self.get_register_mut(register);
        let (result, carry) = register_to_adjust.overflowing_add(relative_value);
        *register_to_adjust = result;
        carry
    }

    #[cfg_attr(test, mutate)]
    pub fn decrease_register(&mut self, register: Register, relative_value: i8) -> bool {
        let register_to_adjust = self.get_register_mut(register);
        let (result, borrow) = register_to_adjust.overflowing_sub(relative_value);
        *register_to_adjust = result;
        borrow
    }

    #[cfg_attr(test, mutate)]
    pub fn set_register_by_function_with_value<F>(
        &mut self,
        target_register: Register,
        value: i8,
        f: F,
    ) where
        F: FnOnce(i8, i8) -> i8,
    {
        let target_register_value = self.get_register_value(target_register);
        self.set_register(target_register, f(value, target_register_value));
    }

    #[cfg_attr(test, mutate)]
    pub fn exchange_register_values(&mut self, register1: Register, register2: Register) {
        let register1_value = self.get_register_value(register1);
        let register2_value = self.get_register_value(register2);
        self.set_register(register2, register1_value);
        self.set_register(register1, register2_value);
    }

    #[cfg_attr(test, mutate)]
    pub fn set_condition_flags_from_result(&mut self, result: i8) {
        self.condition_flags.zero = result == 0;
        self.condition_flags.sign = bit_operations::is_bit_set(result, 7);
        self.condition_flags.parity = bit_operations::get_parity(result);
    }

    #[cfg_attr(test, mutate)]
    pub fn set_condition_flags_from_register_value(&mut self, register: Register) {
        let register_value = self.get_register_value(register);
        self.set_condition_flags_from_result(register_value);
    }
}

#[derive(Default)]
pub struct StateBuilder {
    register_values: Option<RegisterState>,
    condition_flag_values: Option<HashMap<ConditionFlag, bool>>,
    program_counter: Option<u16>,
    stack_pointer: Option<u16>,
    memory_values: Option<HashMap<u16, u8>>,
}

impl StateBuilder {
    #[cfg_attr(test, mutate)]
    pub fn register_values(&mut self, register_values: RegisterState) -> &mut Self {
        let mut new = self;
        new.register_values = Some(register_values);
        new
    }

    #[cfg_attr(test, mutate)]
    pub fn condition_flag_values(
        &mut self,
        condition_flag_values: HashMap<ConditionFlag, bool>,
    ) -> &mut Self {
        let mut new = self;
        new.condition_flag_values = Some(condition_flag_values);
        new
    }

    #[cfg_attr(test, mutate)]
    pub fn program_counter(&mut self, program_counter: u16) -> &mut Self {
        let mut new = self;
        new.program_counter = Some(program_counter);
        new
    }

    #[cfg_attr(test, mutate)]
    pub fn stack_pointer(&mut self, stack_pointer: u16) -> &mut Self {
        let mut new = self;
        new.stack_pointer = Some(stack_pointer);
        new
    }

    #[cfg_attr(test, mutate)]
    pub fn memory_values(&mut self, memory_values: HashMap<u16, u8>) -> &mut Self {
        let mut new = self;
        new.memory_values = Some(memory_values);
        new
    }

    #[cfg_attr(test, mutate)]
    pub fn build(&self) -> State {
        let mut registers = hashmap! {
            Register::A => 0,
            Register::B => 0,
            Register::C => 0,
            Register::D => 0,
            Register::E => 0,
            Register::H => 0,
            Register::L => 0,
        };
        let mut condition_flags = ConditionFlags::default();
        let mut memory = [0; MEMORY_SIZE];

        if let Some(rvs) = &self.register_values {
            for (register, value) in rvs {
                registers.insert(*register, *value);
            }
        }

        if let Some(cfvs) = &self.condition_flag_values {
            for (condition_flag, value) in cfvs {
                condition_flags.set_value(*condition_flag, *value);
            }
        }

        if let Some(mvs) = &self.memory_values {
            for (memory_address, value) in mvs {
                memory[*memory_address as usize] = *value;
            }
        }

        State {
            registers,
            condition_flags,
            program_counter: self.program_counter.unwrap_or(0x0000),
            stack_pointer: self.stack_pointer.unwrap_or(0x0000),
            memory,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base_test_functions::assert_state_is_as_expected;
    use crate::State;

    #[test]
    fn can_get_state_of_all_registers() {
        let state = State::default();
        let register_state = state.get_register_state();
        assert_eq!(register_state.len(), 7);
    }

    #[test]
    fn default_state_has_all_default_values() {
        let state = State::default();
        assert_state_is_as_expected(&state, &State::default());
    }

    #[test]
    #[should_panic(expected = "Invalid bit index of 8")]
    fn is_bit_set_panics_when_given_an_invalid_bit_index() {
        crate::bit_operations::is_bit_set(127, 8);
    }

    #[test]
    #[should_panic(expected = "Invalid bit index of 8")]
    fn get_value_with_bit_set_panics_when_given_an_invalid_bit_index() {
        crate::bit_operations::get_value_with_bit_set(127, 8, true);
    }
}
