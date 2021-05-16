use std::collections::HashMap;

use maplit::hashmap;
#[cfg(test)]
use mutagen::mutate;

pub mod arithmetic_instructions;
#[cfg(test)]
pub mod base_test_functions;
pub mod bit_operations;
pub mod branch_instructions;
pub mod disassembler;
pub mod logical_instructions;
pub mod stack_instructions;
pub mod transfer_instructions;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

impl RegisterPair {
    #[cfg_attr(test, mutate)]
    pub fn get_low_high_value(&self, state: &State) -> (i8, i8) {
        match self {
            RegisterPair::BC => (
                state.get_register_value(Register::C),
                state.get_register_value(Register::B),
            ),
            RegisterPair::DE => (
                state.get_register_value(Register::E),
                state.get_register_value(Register::D),
            ),
            RegisterPair::HL => (
                state.get_register_value(Register::L),
                state.get_register_value(Register::H),
            ),
            RegisterPair::SP => {
                let (low_value, high_value) =
                    bit_operations::split_to_low_high_bytes(state.stack_pointer);
                (low_value as i8, high_value as i8)
            }
        }
    }

    #[cfg_attr(test, mutate)]
    pub fn get_full_value(&self, state: &State) -> u16 {
        if self == &RegisterPair::SP {
            return state.stack_pointer;
        }

        let (low_value, high_value) = self.get_low_high_value(state);
        bit_operations::concat_low_high_bytes(low_value as u8, high_value as u8)
    }

    #[cfg_attr(test, mutate)]
    pub fn set_low_high_value(&self, state: &mut State, low_value: i8, high_value: i8) {
        match self {
            RegisterPair::BC => {
                state.set_register(Register::C, low_value);
                state.set_register(Register::B, high_value);
            }
            RegisterPair::DE => {
                state.set_register(Register::E, low_value);
                state.set_register(Register::D, high_value);
            }
            RegisterPair::HL => {
                state.set_register(Register::L, low_value);
                state.set_register(Register::H, high_value);
            }
            RegisterPair::SP => {
                state.stack_pointer =
                    bit_operations::concat_low_high_bytes(low_value as u8, high_value as u8)
            }
        };
    }

    #[cfg_attr(test, mutate)]
    pub fn set_full_value(&self, state: &mut State, value: u16) {
        if self == &RegisterPair::SP {
            state.stack_pointer = value;
        }

        let (low_value, high_value) = bit_operations::split_to_low_high_bytes(value);
        self.set_low_high_value(state, low_value as i8, high_value as i8);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ConditionFlag {
    Zero,
    Sign,
    Parity,
    Carry,
    AuxiliaryCarry,
}

pub type Condition = (ConditionFlag, bool);

#[derive(Copy, Clone, Debug)]
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
    pub are_interrupts_enabled: bool,
    memory_footprint: HashMap<u16, u8>,
    is_memory_loaded: bool,
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
    pub fn set_condition_flag_value(&mut self, condition_flag: ConditionFlag, value: bool) {
        self.condition_flags.set_value(condition_flag, value);
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
    pub fn load_memory(&mut self, contiguous_memory_bytes: Vec<u8>) {
        for (memory_address, memory_value) in contiguous_memory_bytes.iter().enumerate() {
            self.set_value_at_memory_location(memory_address as u16, *memory_value);
        }
        self.is_memory_loaded = true;
    }

    #[cfg_attr(test, mutate)]
    pub fn get_value_at_memory_location(&self, memory_address: u16) -> u8 {
        self.memory[memory_address as usize]
    }

    #[cfg_attr(test, mutate)]
    pub fn get_memory_value_at_program_counter(&self) -> u8 {
        self.get_value_at_memory_location(self.program_counter)
    }

    #[cfg_attr(test, mutate)]
    pub fn set_value_at_memory_location(&mut self, memory_address: u16, value: u8) {
        self.memory[memory_address as usize] = value;

        if self.is_memory_loaded {
            if value == 0 {
                self.memory_footprint.remove(&memory_address);
            } else {
                self.memory_footprint.insert(memory_address, value);
            }
        }
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

    #[cfg_attr(test, mutate)]
    pub fn run_operation(&mut self, operation: Operation) {
        let op_code_pc = self.program_counter;
        self.program_counter += 1;

        let mut additional_byte_1 = None;
        let mut additional_byte_2 = None;
        let num_additional_bytes = operation.num_additional_bytes();

        if num_additional_bytes >= 1 {
            additional_byte_1 = Some(self.get_memory_value_at_program_counter());
            self.program_counter += 1;
        }

        if num_additional_bytes >= 2 {
            additional_byte_2 = Some(self.get_memory_value_at_program_counter());
            self.program_counter += 1;
        }

        if let (Some(byte_1), Some(byte_2)) = (additional_byte_1, additional_byte_2) {
            println!(
                "{:04X?} {:?} {:04X?}",
                op_code_pc,
                operation,
                bit_operations::concat_low_high_bytes(byte_1, byte_2)
            );
        } else if let Some(byte_1) = additional_byte_1 {
            println!("{:04X?} {:?} {:02X?}", op_code_pc, operation, byte_1);
        } else {
            println!("{:04X?} {:?}", op_code_pc, operation);
        }

        operation.run_operation(self, additional_byte_1, additional_byte_2);

        const DISPLAY_MEMORY_FOOTPRINT: bool = false;

        if operation != Operation::Nop {
            println!(
                "## pc: {:04X?}, sp: {:04X?}, registers: {:?}, {:?} ##",
                self.program_counter, self.stack_pointer, self.registers, self.condition_flags
            );

            if DISPLAY_MEMORY_FOOTPRINT {
                println!("## memory: {:?} ##", self.memory_footprint);
            }
        }
    }
}

#[derive(Default)]
pub struct StateBuilder {
    register_values: Option<RegisterState>,
    condition_flag_values: Option<HashMap<ConditionFlag, bool>>,
    program_counter: Option<u16>,
    stack_pointer: Option<u16>,
    memory_values: Option<HashMap<u16, u8>>,
    are_interrupts_enabled: Option<bool>,
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

    pub fn are_interrupts_enabled(&mut self, are_interrupts_enabled: bool) -> &mut Self {
        let mut new = self;
        new.are_interrupts_enabled = Some(are_interrupts_enabled);
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
            are_interrupts_enabled: self.are_interrupts_enabled.unwrap_or(false),
            memory_footprint: HashMap::new(),
            is_memory_loaded: false,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Mov(Register, Register),
    MovFromMem(Register),
    MovToMem(Register),
    Mvi(Register),
    MviMem,
    Lxi(RegisterPair),
    Lda,
    Sta,
    Lhld,
    Shld,
    Ldax(RegisterPair),
    Stax(RegisterPair),
    Xchg,
    Add(Register),
    Adi,
    Aci,
    Sub(Register),
    Sui,
    Sbi,
    Inr(Register),
    InrMem,
    Dcr(Register),
    DcrMem,
    Inx(RegisterPair),
    Dcx(RegisterPair),
    Dad(RegisterPair),
    Cmp(Register),
    Ana(Register),
    Ani,
    Xra(Register),
    Xri,
    Ora(Register),
    Ori,
    Cpi,
    Rlc,
    Rrc,
    Ral,
    Rar,
    Cmc,
    Stc,
    Jmp,
    Jcond(Condition),
    Call,
    Ccond(Condition),
    Ret,
    Rcond(Condition),
    Push(RegisterPair),
    PushPsw,
    Pop(RegisterPair),
    PopPsw,
    In,
    Out,
    Ei,
    Di,
    Hlt,
    Nop,
}

impl Operation {
    #[cfg_attr(test, mutate)]
    pub fn num_additional_bytes(&self) -> u8 {
        match self {
            Operation::Mvi(_) => 1,
            Operation::MviMem => 1,
            Operation::Lxi(_) => 2,
            Operation::Lda => 2,
            Operation::Sta => 2,
            Operation::Lhld => 2,
            Operation::Shld => 2,
            Operation::Adi => 1,
            Operation::Aci => 1,
            Operation::Sui => 1,
            Operation::Sbi => 1,
            Operation::Ani => 1,
            Operation::Xri => 1,
            Operation::Ori => 1,
            Operation::Cpi => 1,
            Operation::Jmp => 2,
            Operation::Jcond(_) => 2,
            Operation::Call => 2,
            Operation::Ccond(_) => 2,
            Operation::In => 1,
            Operation::Out => 1,
            _ => 0,
        }
    }

    #[cfg_attr(test, mutate)]
    pub fn run_operation(
        &self,
        state: &mut State,
        additional_byte_1: Option<u8>,
        additional_byte_2: Option<u8>,
    ) {
        let mut is_low_data_required = false;
        let mut is_high_data_required = false;

        let mut get_low_data = || {
            is_low_data_required = true;
            additional_byte_1.expect("Expected byte 1 to be present but it was not")
        };

        let mut get_high_data = || {
            is_high_data_required = true;
            additional_byte_2.expect("Expected byte 2 to be present but it was not")
        };

        match self {
            Operation::Mov(source_register, destination_register) => {
                transfer_instructions::mov_instruction(
                    state,
                    *source_register,
                    *destination_register,
                )
            }
            Operation::MovToMem(register) => {
                transfer_instructions::mov_to_mem_instruction(state, *register)
            }
            Operation::MovFromMem(register) => {
                transfer_instructions::mov_from_mem_instruction(state, *register)
            }
            Operation::Mvi(register) => {
                transfer_instructions::mvi_instruction(state, *register, get_low_data() as i8)
            }
            Operation::MviMem => {
                transfer_instructions::mvi_mem_instruction(state, get_low_data() as i8)
            }
            Operation::Lxi(register_pair) => transfer_instructions::lxi_instruction(
                state,
                *register_pair,
                get_low_data() as i8,
                get_high_data() as i8,
            ),
            Operation::Lda => {
                transfer_instructions::lda_instruction(state, get_low_data(), get_high_data())
            }
            Operation::Sta => {
                transfer_instructions::sta_instruction(state, get_low_data(), get_high_data())
            }
            Operation::Lhld => {
                transfer_instructions::lhld_instruction(state, get_low_data(), get_high_data())
            }
            Operation::Shld => {
                transfer_instructions::shld_instruction(state, get_low_data(), get_high_data())
            }
            Operation::Ldax(register_pair) => {
                transfer_instructions::ldax_instruction(state, *register_pair)
            }
            Operation::Stax(register_pair) => {
                transfer_instructions::stax_instruction(state, *register_pair)
            }
            Operation::Xchg => transfer_instructions::xchg_instruction(state),
            Operation::Add(register) => arithmetic_instructions::add_instruction(state, *register),
            Operation::Adi => arithmetic_instructions::adi_instruction(state, get_low_data() as i8),
            Operation::Aci => arithmetic_instructions::aci_instruction(state, get_low_data() as i8),
            Operation::Sub(register) => arithmetic_instructions::sub_instruction(state, *register),
            Operation::Sui => arithmetic_instructions::sui_instruction(state, get_low_data() as i8),
            Operation::Sbi => arithmetic_instructions::sbi_instruction(state, get_low_data() as i8),
            Operation::Inr(register) => arithmetic_instructions::inr_instruction(state, *register),
            Operation::InrMem => arithmetic_instructions::inr_mem_instruction(state),
            Operation::Dcr(register) => arithmetic_instructions::dcr_instruction(state, *register),
            Operation::DcrMem => arithmetic_instructions::dcr_mem_instruction(state),
            Operation::Inx(register_pair) => {
                arithmetic_instructions::inx_instruction(state, *register_pair)
            }
            Operation::Dcx(register_pair) => {
                arithmetic_instructions::dcx_instruction(state, *register_pair)
            }
            Operation::Dad(register_pair) => {
                arithmetic_instructions::dad_instruction(state, *register_pair)
            }
            Operation::Cmp(register) => logical_instructions::cmp_instruction(state, *register),
            Operation::Ana(register) => logical_instructions::ana_instruction(state, *register),
            Operation::Ani => logical_instructions::ani_instruction(state, get_low_data() as i8),
            Operation::Xra(register) => logical_instructions::xra_instruction(state, *register),
            Operation::Xri => logical_instructions::xri_instruction(state, get_low_data() as i8),
            Operation::Ora(register) => logical_instructions::ora_instruction(state, *register),
            Operation::Ori => logical_instructions::ori_instruction(state, get_low_data() as i8),
            Operation::Cpi => logical_instructions::cpi_instruction(state, get_low_data() as i8),
            Operation::Rlc => logical_instructions::rlc_instruction(state),
            Operation::Rrc => logical_instructions::rrc_instruction(state),
            Operation::Ral => logical_instructions::ral_instruction(state),
            Operation::Rar => logical_instructions::rar_instruction(state),
            Operation::Cmc => logical_instructions::cmc_instruction(state),
            Operation::Stc => logical_instructions::stc_instruction(state),
            Operation::Jmp => {
                branch_instructions::jmp_instruction(state, get_low_data(), get_high_data())
            }
            Operation::Jcond(condition) => branch_instructions::jcond_instruction(
                state,
                get_low_data(),
                get_high_data(),
                *condition,
            ),
            Operation::Call => {
                branch_instructions::call_instruction(state, get_low_data(), get_high_data())
            }
            Operation::Ccond(condition) => branch_instructions::ccond_instruction(
                state,
                get_low_data(),
                get_high_data(),
                *condition,
            ),
            Operation::Ret => branch_instructions::ret_instruction(state),
            Operation::Rcond(condition) => {
                branch_instructions::rcond_instruction(state, *condition)
            }
            Operation::Push(register_pair) => {
                stack_instructions::push_instruction(state, *register_pair)
            }
            Operation::Pop(register_pair) => {
                stack_instructions::pop_instruction(state, *register_pair)
            }
            Operation::PushPsw => {
                stack_instructions::push_psw_instruction(state);
            }
            Operation::PopPsw => {
                stack_instructions::pop_psw_instruction(state);
            }
            Operation::In => {
                println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --");
                let port_number = get_low_data();
                match port_number {
                    1 => {
                        println!("Port {}", port_number);
                        state.set_register(Register::A, 0b0000_1000);
                    }
                    2 => {
                        println!("Port {}", port_number);
                        state.set_register(Register::A, 0b0000_0000);
                    }
                    _ => panic!("Can't handle Port {}", port_number),
                };
            }
            Operation::Out => {
                println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --");
                let port_number = get_low_data();
                match port_number {
                    3 | 5 | 6 => println!("Port {}", port_number),
                    _ => panic!("Can't handle Port {}", port_number),
                };
            }
            Operation::Ei => stack_instructions::ei_instruction(state),
            Operation::Di => stack_instructions::di_instruction(state),
            Operation::Hlt => {
                todo!();
            }
            Operation::Nop => (),
        };

        if !is_high_data_required && additional_byte_2.is_some() {
            panic!("Expected byte 2 to not be present but it was");
        }

        if !is_low_data_required && additional_byte_1.is_some() {
            panic!("Expected byte 1 to not be present but it was");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_test_functions::assert_state_is_as_expected;

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

    #[allow(overflowing_literals)]
    #[test]
    fn stack_pointer_value_returned_by_register_pair_is_same_as_actual_value() {
        let state = StateBuilder::default().stack_pointer(0xF00F).build();
        assert_eq!((0x0F, 0xF0), RegisterPair::SP.get_low_high_value(&state));
        assert_eq!(state.stack_pointer, RegisterPair::SP.get_full_value(&state));
    }

    #[test]
    #[should_panic(expected = "Invalid bit index of 8")]
    fn is_bit_set_panics_when_given_an_invalid_bit_index() {
        bit_operations::is_bit_set(127, 8);
    }

    #[test]
    #[should_panic(expected = "Invalid bit index of 8")]
    fn get_value_with_bit_set_panics_when_given_an_invalid_bit_index() {
        bit_operations::set_bit_in_value(&mut 127, 8, true);
    }
}
