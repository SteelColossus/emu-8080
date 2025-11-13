use std::collections::HashMap;

use enum_map::{Enum, EnumMap};
use log::{Level, debug, log_enabled};
// #[cfg(test)]
// use mutagen::mutate;

pub mod arithmetic_instructions;
#[cfg(test)]
pub mod base_test_functions;
pub mod bit_operations;
pub mod branch_instructions;
pub mod disassembler;
pub mod logical_instructions;
pub mod runner;
pub mod stack_instructions;
pub mod transfer_instructions;

#[derive(Copy, Clone, Enum, Eq, PartialEq, Hash, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub type RegisterState = EnumMap<Register, u8>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

impl std::fmt::Display for RegisterPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegisterPair::BC => "BC",
                RegisterPair::DE => "DE",
                RegisterPair::HL => "HL",
                RegisterPair::SP => "SP",
            }
        )
    }
}

#[derive(Copy, Clone, Enum, Eq, PartialEq, Hash, Debug)]
pub enum ConditionFlag {
    Zero,
    Sign,
    Parity,
    Carry,
    AuxiliaryCarry,
}

pub type ConditionFlags = EnumMap<ConditionFlag, bool>;
pub type Condition = (ConditionFlag, bool);

pub trait Ports {
    fn read_in_port(&self, port_number: u8) -> u8;
    fn write_out_port(&mut self, port_number: u8, value: u8);
    fn in_port_static_value(&self, port_number: u8) -> Option<u8>;
    fn set_in_port_static_value(&mut self, port_number: u8, value: u8);
}

struct DefaultPorts;

impl Ports for DefaultPorts {
    fn read_in_port(&self, _port_number: u8) -> u8 {
        0
    }
    fn write_out_port(&mut self, _port_number: u8, _value: u8) {}
    fn in_port_static_value(&self, _port_number: u8) -> Option<u8> {
        None
    }
    fn set_in_port_static_value(&mut self, _port_number: u8, _value: u8) {}
}

const MEMORY_SIZE: usize = u16::MAX as usize + 1;

const CONDITION_FLAG_BITS: [(ConditionFlag, u8); 5] = [
    (ConditionFlag::Carry, 0),
    (ConditionFlag::Parity, 2),
    (ConditionFlag::AuxiliaryCarry, 4),
    (ConditionFlag::Zero, 6),
    (ConditionFlag::Sign, 7),
];

pub struct State {
    pub registers: RegisterState,
    pub condition_flags: ConditionFlags,
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub memory: [u8; MEMORY_SIZE],
    pub are_interrupts_enabled: bool,
    pub is_halted: bool,
    pub ports: Box<dyn Ports>,
    cpu_total_state_count: usize,
}

impl Default for State {
    // #[cfg_attr(test, mutate)]
    fn default() -> Self {
        StateBuilder::default().build()
    }
}

impl State {
    pub fn load_memory(&mut self, contiguous_memory_bytes: &[u8]) {
        for (memory_address, memory_value) in contiguous_memory_bytes.iter().enumerate() {
            self.memory[memory_address] = *memory_value;
        }
    }

    // #[cfg_attr(test, mutate)]
    pub fn memory_value_at_pc(&self) -> u8 {
        self.memory[self.program_counter as usize]
    }

    // #[cfg_attr(test, mutate)]
    pub fn increase_register(&mut self, register: Register, relative_value: u8) -> (bool, bool) {
        let register_to_adjust = &mut self.registers[register];
        let (result, carry) = register_to_adjust.overflowing_add(relative_value);
        let auxiliary_carry =
            bit_operations::calculate_auxiliary_carry(*register_to_adjust, relative_value, false);
        *register_to_adjust = result;
        (carry, auxiliary_carry)
    }

    // #[cfg_attr(test, mutate)]
    pub fn decrease_register(&mut self, register: Register, relative_value: u8) -> (bool, bool) {
        let register_to_adjust = &mut self.registers[register];
        let (result, borrow) = register_to_adjust.overflowing_sub(relative_value);
        let auxiliary_borrow =
            bit_operations::calculate_auxiliary_carry(*register_to_adjust, relative_value, true);
        *register_to_adjust = result;
        (borrow, auxiliary_borrow)
    }

    // #[cfg_attr(test, mutate)]
    pub fn set_register_by_function_with_value<F>(
        &mut self,
        target_register: Register,
        value: u8,
        f: F,
    ) where
        F: FnOnce(u8, u8) -> u8,
    {
        let target_register_value = self.registers[target_register];
        self.registers[target_register] = f(value, target_register_value);
    }

    // #[cfg_attr(test, mutate)]
    pub fn exchange_register_values(&mut self, register1: Register, register2: Register) {
        let register1_value = self.registers[register1];
        let register2_value = self.registers[register2];
        self.registers[register2] = register1_value;
        self.registers[register1] = register2_value;
    }

    // #[cfg_attr(test, mutate)]
    pub fn set_condition_flags_from_result(&mut self, result: u8) {
        self.condition_flags[ConditionFlag::Zero] = result == 0;
        self.condition_flags[ConditionFlag::Sign] = bit_operations::is_bit_set(result, 7);
        self.condition_flags[ConditionFlag::Parity] = bit_operations::parity(result);
    }

    // #[cfg_attr(test, mutate)]
    pub fn set_condition_flags_from_register_value(&mut self, register: Register) {
        let register_value = self.registers[register];
        self.set_condition_flags_from_result(register_value);
    }

    // #[cfg_attr(test, mutate)]
    pub fn condition_flag_byte(&self) -> u8 {
        let mut condition_flag_byte = 0b0000_0010;
        for (condition_flag, bit_index) in &CONDITION_FLAG_BITS {
            bit_operations::set_bit_in_value(
                &mut condition_flag_byte,
                *bit_index,
                self.condition_flags[*condition_flag],
            );
        }
        condition_flag_byte
    }

    // #[cfg_attr(test, mutate)]
    pub fn set_condition_flag_byte(&mut self, memory_address: u16) {
        let condition_flag_byte = self.memory[memory_address as usize];
        for (condition_flag, bit_index) in &CONDITION_FLAG_BITS {
            self.condition_flags[*condition_flag] =
                bit_operations::is_bit_set(condition_flag_byte, *bit_index);
        }
    }

    // #[cfg_attr(test, mutate)]
    pub fn is_condition_true(&self, condition: Condition) -> bool {
        self.condition_flags[condition.0] == condition.1
    }

    // #[cfg_attr(test, mutate)]
    pub fn low_high_rp_value(&self, register_pair: RegisterPair) -> (u8, u8) {
        match register_pair {
            RegisterPair::BC => (self.registers[Register::C], self.registers[Register::B]),
            RegisterPair::DE => (self.registers[Register::E], self.registers[Register::D]),
            RegisterPair::HL => (self.registers[Register::L], self.registers[Register::H]),
            RegisterPair::SP => {
                let (low_value, high_value) =
                    bit_operations::split_to_low_high_bytes(self.stack_pointer);
                (low_value, high_value)
            }
        }
    }

    // #[cfg_attr(test, mutate)]
    pub fn full_rp_value(&self, register_pair: RegisterPair) -> u16 {
        if register_pair == RegisterPair::SP {
            return self.stack_pointer;
        }

        let (low_value, high_value) = self.low_high_rp_value(register_pair);
        bit_operations::concat_low_high_bytes(low_value, high_value)
    }

    // #[cfg_attr(test, mutate)]
    pub fn set_low_high_rp_value(
        &mut self,
        register_pair: RegisterPair,
        low_value: u8,
        high_value: u8,
    ) {
        match register_pair {
            RegisterPair::BC => {
                self.registers[Register::C] = low_value;
                self.registers[Register::B] = high_value;
            }
            RegisterPair::DE => {
                self.registers[Register::E] = low_value;
                self.registers[Register::D] = high_value;
            }
            RegisterPair::HL => {
                self.registers[Register::L] = low_value;
                self.registers[Register::H] = high_value;
            }
            RegisterPair::SP => {
                self.stack_pointer = bit_operations::concat_low_high_bytes(low_value, high_value);
            }
        };
    }

    // #[cfg_attr(test, mutate)]
    pub fn set_full_rp_value(&mut self, register_pair: RegisterPair, value: u16) {
        if register_pair == RegisterPair::SP {
            self.stack_pointer = value;
        }

        let (low_value, high_value) = bit_operations::split_to_low_high_bytes(value);
        self.set_low_high_rp_value(register_pair, low_value, high_value);
    }

    // #[cfg_attr(test, mutate)]
    pub fn cpu_total_state_count(&self) -> usize {
        self.cpu_total_state_count
    }

    pub fn run_operation(&mut self, operation: &Operation) {
        let op_code_pc = self.program_counter;
        self.program_counter += 1;

        let mut additional_byte_1 = None;
        let mut additional_byte_2 = None;
        let instruction_data_type = operation.additional_data_required();

        if instruction_data_type == InstructionDataType::Single
            || instruction_data_type == InstructionDataType::LowHigh
        {
            additional_byte_1 = Some(self.memory_value_at_pc());
            self.program_counter += 1;
        }

        if instruction_data_type == InstructionDataType::LowHigh {
            additional_byte_2 = Some(self.memory_value_at_pc());
            self.program_counter += 1;
        }

        self.log_current_state(op_code_pc);
        runner::run_operation(operation, self, additional_byte_1, additional_byte_2);
        self.cpu_total_state_count += operation.machine_states(self) as usize;
    }

    fn log_current_state(&self, op_code_pc: u16) {
        if log_enabled!(Level::Debug) {
            debug!(
                "PC: {:04X}, AF: {:04X}, BC: {:04X}, DE: {:04X}, HL: {:04X}, SP: {:04X}, CYC: {}\t\
                ({:02X} {:02X} {:02X} {:02X})",
                op_code_pc,
                bit_operations::concat_low_high_bytes(
                    self.condition_flag_byte(),
                    self.registers[Register::A]
                ),
                self.full_rp_value(RegisterPair::BC),
                self.full_rp_value(RegisterPair::DE),
                self.full_rp_value(RegisterPair::HL),
                self.stack_pointer,
                self.cpu_total_state_count,
                self.memory[op_code_pc as usize],
                self.memory[op_code_pc as usize + 1],
                self.memory[op_code_pc as usize + 2],
                self.memory[op_code_pc as usize + 3],
            );
        }
    }
}

#[derive(Default)]
pub struct StateBuilder {
    register_values: Option<HashMap<Register, u8>>,
    condition_flag_values: Option<HashMap<ConditionFlag, bool>>,
    program_counter: Option<u16>,
    stack_pointer: Option<u16>,
    memory_values: Option<HashMap<u16, u8>>,
    are_interrupts_enabled: Option<bool>,
    is_halted: Option<bool>,
}

impl StateBuilder {
    // #[cfg_attr(test, mutate)]
    pub fn register_values(&mut self, register_values: HashMap<Register, u8>) -> &mut Self {
        let new = self;
        new.register_values = Some(register_values);
        new
    }

    // #[cfg_attr(test, mutate)]
    pub fn condition_flag_values(
        &mut self,
        condition_flag_values: HashMap<ConditionFlag, bool>,
    ) -> &mut Self {
        let new = self;
        new.condition_flag_values = Some(condition_flag_values);
        new
    }

    // #[cfg_attr(test, mutate)]
    pub fn program_counter(&mut self, program_counter: u16) -> &mut Self {
        let new = self;
        new.program_counter = Some(program_counter);
        new
    }

    // #[cfg_attr(test, mutate)]
    pub fn stack_pointer(&mut self, stack_pointer: u16) -> &mut Self {
        let new = self;
        new.stack_pointer = Some(stack_pointer);
        new
    }

    // #[cfg_attr(test, mutate)]
    pub fn memory_values(&mut self, memory_values: HashMap<u16, u8>) -> &mut Self {
        let new = self;
        new.memory_values = Some(memory_values);
        new
    }

    // #[cfg_attr(test, mutate)]
    pub fn interrupts_enabled(&mut self, are_interrupts_enabled: bool) -> &mut Self {
        let new = self;
        new.are_interrupts_enabled = Some(are_interrupts_enabled);
        new
    }

    // #[cfg_attr(test, mutate)]
    pub fn halted(&mut self, is_halted: bool) -> &mut Self {
        let new = self;
        new.is_halted = Some(is_halted);
        new
    }

    // #[cfg_attr(test, mutate)]
    pub fn build(&self) -> State {
        let mut registers = RegisterState::default();
        let mut condition_flags = ConditionFlags::default();
        let mut memory = [0; MEMORY_SIZE];

        if let Some(rvs) = &self.register_values {
            for (register, value) in rvs {
                registers[*register] = *value;
            }
        }

        if let Some(cfvs) = &self.condition_flag_values {
            for (condition_flag, value) in cfvs {
                condition_flags[*condition_flag] = *value;
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
            is_halted: self.is_halted.unwrap_or(false),
            ports: Box::new(DefaultPorts),
            cpu_total_state_count: 0,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum InstructionDataType {
    None,
    Single,
    LowHigh,
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
    AddMem,
    Adi,
    Adc(Register),
    AdcMem,
    Aci,
    Sub(Register),
    SubMem,
    Sui,
    Sbb(Register),
    SbbMem,
    Sbi,
    Inr(Register),
    InrMem,
    Dcr(Register),
    DcrMem,
    Inx(RegisterPair),
    Dcx(RegisterPair),
    Dad(RegisterPair),
    Daa,
    Ana(Register),
    AnaMem,
    Ani,
    Xra(Register),
    XraMem,
    Xri,
    Ora(Register),
    OraMem,
    Ori,
    Cmp(Register),
    CmpMem,
    Cpi,
    Rlc,
    Rrc,
    Ral,
    Rar,
    Cma,
    Cmc,
    Stc,
    Jmp,
    Jcond(Condition),
    Call,
    Ccond(Condition),
    Ret,
    Rcond(Condition),
    Rst(u8),
    Pchl,
    Push(RegisterPair),
    PushPsw,
    Pop(RegisterPair),
    PopPsw,
    Xthl,
    Sphl,
    In,
    Out,
    Ei,
    Di,
    Hlt,
    Nop,
}

impl Operation {
    pub fn additional_data_required(&self) -> InstructionDataType {
        match self {
            Operation::Mvi(_) => InstructionDataType::Single,
            Operation::MviMem => InstructionDataType::Single,
            Operation::Lxi(_) => InstructionDataType::LowHigh,
            Operation::Lda => InstructionDataType::LowHigh,
            Operation::Sta => InstructionDataType::LowHigh,
            Operation::Lhld => InstructionDataType::LowHigh,
            Operation::Shld => InstructionDataType::LowHigh,
            Operation::Adi => InstructionDataType::Single,
            Operation::Aci => InstructionDataType::Single,
            Operation::Sui => InstructionDataType::Single,
            Operation::Sbi => InstructionDataType::Single,
            Operation::Ani => InstructionDataType::Single,
            Operation::Xri => InstructionDataType::Single,
            Operation::Ori => InstructionDataType::Single,
            Operation::Cpi => InstructionDataType::Single,
            Operation::Jmp => InstructionDataType::LowHigh,
            Operation::Jcond(_) => InstructionDataType::LowHigh,
            Operation::Call => InstructionDataType::LowHigh,
            Operation::Ccond(_) => InstructionDataType::LowHigh,
            Operation::In => InstructionDataType::Single,
            Operation::Out => InstructionDataType::Single,
            _ => InstructionDataType::None,
        }
    }

    pub fn machine_cycles(&self, state: &State) -> u8 {
        match self {
            Operation::Mov(_, _) => 1,
            Operation::MovFromMem(_) | Operation::MovToMem(_) => 2,
            Operation::Mvi(_) => 2,
            Operation::MviMem => 3,
            Operation::Lxi(_) => 3,
            Operation::Lda | Operation::Sta => 4,
            Operation::Lhld | Operation::Shld => 5,
            Operation::Ldax(_) | Operation::Stax(_) => 2,
            Operation::Xchg => 1,
            Operation::Add(_) | Operation::Adc(_) | Operation::Sub(_) | Operation::Sbb(_) => 1,
            Operation::AddMem | Operation::AdcMem | Operation::SubMem | Operation::SbbMem => 2,
            Operation::Adi | Operation::Aci | Operation::Sui | Operation::Sbi => 2,
            Operation::Inr(_) | Operation::Dcr(_) => 1,
            Operation::InrMem | Operation::DcrMem => 2,
            Operation::Inx(_) | Operation::Dcx(_) => 1,
            Operation::Dad(_) => 3,
            Operation::Daa => 1,
            Operation::Ana(_) | Operation::Xra(_) | Operation::Ora(_) => 1,
            Operation::AnaMem | Operation::XraMem | Operation::OraMem => 2,
            Operation::Ani | Operation::Xri | Operation::Ori => 2,
            Operation::Cmp(_) => 1,
            Operation::CmpMem => 2,
            Operation::Cpi => 2,
            Operation::Rlc | Operation::Ral | Operation::Rrc | Operation::Rar => 1,
            Operation::Cma | Operation::Cmc => 1,
            Operation::Stc => 1,
            Operation::Jmp | Operation::Jcond(_) => 3,
            Operation::Call => 5,
            Operation::Ccond(condition) => {
                if state.is_condition_true(*condition) {
                    5
                } else {
                    3
                }
            }
            Operation::Ret => 3,
            Operation::Rcond(condition) => {
                if state.is_condition_true(*condition) {
                    3
                } else {
                    1
                }
            }
            Operation::Rst(_) => 3,
            Operation::Pchl => 1,
            Operation::Push(_) | Operation::PushPsw => 3,
            Operation::Pop(_) | Operation::PopPsw => 3,
            Operation::Xthl => 5,
            Operation::Sphl => 1,
            Operation::In | Operation::Out => 3,
            Operation::Ei | Operation::Di => 1,
            Operation::Hlt => 1,
            Operation::Nop => 1,
        }
    }

    // This confusing terminology comes from the 8080 manual:
    // a state really refers to the smallest unit of processing activity.
    pub fn machine_states(&self, state: &State) -> u8 {
        match self {
            Operation::Mov(_, _) => 5,
            Operation::MovFromMem(_) | Operation::MovToMem(_) => 7,
            Operation::Mvi(_) => 7,
            Operation::MviMem => 10,
            Operation::Lxi(_) => 10,
            Operation::Lda | Operation::Sta => 13,
            Operation::Lhld | Operation::Shld => 16,
            Operation::Ldax(_) | Operation::Stax(_) => 7,
            Operation::Xchg => 4,
            Operation::Add(_) | Operation::Adc(_) | Operation::Sub(_) | Operation::Sbb(_) => 4,
            Operation::AddMem | Operation::AdcMem | Operation::SubMem | Operation::SbbMem => 7,
            Operation::Adi | Operation::Aci | Operation::Sui | Operation::Sbi => 7,
            Operation::Inr(_) | Operation::Dcr(_) => 5,
            Operation::InrMem | Operation::DcrMem => 10,
            Operation::Inx(_) | Operation::Dcx(_) => 5,
            Operation::Dad(_) => 10,
            Operation::Daa => 4,
            Operation::Ana(_) | Operation::Xra(_) | Operation::Ora(_) => 4,
            Operation::AnaMem | Operation::XraMem | Operation::OraMem => 7,
            Operation::Ani | Operation::Xri | Operation::Ori => 7,
            Operation::Cmp(_) => 4,
            Operation::CmpMem => 7,
            Operation::Cpi => 7,
            Operation::Rlc | Operation::Ral | Operation::Rrc | Operation::Rar => 4,
            Operation::Cma | Operation::Cmc => 4,
            Operation::Stc => 4,
            Operation::Jmp | Operation::Jcond(_) => 10,
            Operation::Call => 17,
            Operation::Ccond(condition) => {
                if state.is_condition_true(*condition) {
                    17
                } else {
                    11
                }
            }
            Operation::Ret => 10,
            Operation::Rcond(condition) => {
                if state.is_condition_true(*condition) {
                    11
                } else {
                    5
                }
            }
            Operation::Rst(_) => 11,
            Operation::Pchl => 5,
            Operation::Push(_) | Operation::PushPsw => 11,
            Operation::Pop(_) | Operation::PopPsw => 10,
            Operation::Xthl => 18,
            Operation::Sphl => 5,
            Operation::In | Operation::Out => 10,
            Operation::Ei | Operation::Di => 4,
            Operation::Hlt => 7,
            Operation::Nop => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_test_functions::assert_state_is_as_expected;

    #[test]
    fn default_state_has_all_default_values() {
        let state = State::default();
        assert_state_is_as_expected(&state, &State::default());
        assert_eq!(state.cpu_total_state_count(), 0);
    }

    #[test]
    fn stack_pointer_value_returned_by_register_pair_is_same_as_actual_value() {
        let state = StateBuilder::default().stack_pointer(0xF00F).build();
        assert_eq!((0x0F, 0xF0), state.low_high_rp_value(RegisterPair::SP));
        assert_eq!(state.stack_pointer, state.full_rp_value(RegisterPair::SP));
    }
}
