use crate::{Condition, ConditionFlag, Register, RegisterPair, State};
#[cfg(test)]
use mutagen::mutate;

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
    Dcr(Register),
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

    pub fn run_operation(
        &self,
        state: &mut State,
        additional_byte_1: Option<u8>,
        additional_byte_2: Option<u8>,
    ) {
        match self {
            Operation::Mov(source_register, destination_register) => {
                crate::transfer_instructions::mov_instruction(
                    state,
                    *source_register,
                    *destination_register,
                )
            }
            Operation::MovToMem(register) => {
                crate::transfer_instructions::mov_to_mem_instruction(state, *register)
            }
            Operation::MovFromMem(register) => {
                crate::transfer_instructions::mov_from_mem_instruction(state, *register)
            }
            Operation::Mvi(register) => crate::transfer_instructions::mvi_instruction(
                state,
                *register,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::MviMem => crate::transfer_instructions::mvi_mem_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Lxi(register_pair) => crate::transfer_instructions::lxi_instruction(
                state,
                *register_pair,
                additional_byte_1.unwrap() as i8,
                additional_byte_2.unwrap() as i8,
            ),
            Operation::Lda => crate::transfer_instructions::lda_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
            ),
            Operation::Sta => crate::transfer_instructions::sta_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
            ),
            Operation::Lhld => crate::transfer_instructions::lhld_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
            ),
            Operation::Shld => crate::transfer_instructions::shld_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
            ),
            Operation::Ldax(register_pair) => {
                crate::transfer_instructions::ldax_instruction(state, *register_pair)
            }
            Operation::Stax(register_pair) => {
                crate::transfer_instructions::stax_instruction(state, *register_pair)
            }
            Operation::Xchg => crate::transfer_instructions::xchg_instruction(state),
            Operation::Add(register) => {
                crate::arithmetic_instructions::add_instruction(state, *register)
            }
            Operation::Adi => crate::arithmetic_instructions::adi_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Aci => crate::arithmetic_instructions::aci_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Sub(register) => {
                crate::arithmetic_instructions::sub_instruction(state, *register)
            }
            Operation::Sui => crate::arithmetic_instructions::sui_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Sbi => crate::arithmetic_instructions::sbi_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Inr(register) => {
                crate::arithmetic_instructions::inr_instruction(state, *register)
            }
            Operation::Dcr(register) => {
                crate::arithmetic_instructions::dcr_instruction(state, *register)
            }
            Operation::Inx(register_pair) => {
                crate::arithmetic_instructions::inx_instruction(state, *register_pair)
            }
            Operation::Dcx(register_pair) => {
                crate::arithmetic_instructions::dcx_instruction(state, *register_pair)
            }
            Operation::Dad(register_pair) => {
                crate::arithmetic_instructions::dad_instruction(state, *register_pair)
            }
            Operation::Cmp(register) => {
                crate::logical_instructions::cmp_instruction(state, *register)
            }
            Operation::Ana(register) => {
                crate::logical_instructions::ana_instruction(state, *register)
            }
            Operation::Ani => crate::logical_instructions::ani_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Xra(register) => {
                crate::logical_instructions::xra_instruction(state, *register)
            }
            Operation::Xri => crate::logical_instructions::xri_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Ora(register) => {
                crate::logical_instructions::ora_instruction(state, *register)
            }
            Operation::Ori => crate::logical_instructions::ori_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Cpi => crate::logical_instructions::cpi_instruction(
                state,
                additional_byte_1.unwrap() as i8,
            ),
            Operation::Rlc => crate::logical_instructions::rlc_instruction(state),
            Operation::Rrc => crate::logical_instructions::rrc_instruction(state),
            Operation::Ral => crate::logical_instructions::ral_instruction(state),
            Operation::Rar => crate::logical_instructions::rar_instruction(state),
            Operation::Jmp => crate::branch_instructions::jmp_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
            ),
            Operation::Jcond(condition) => crate::branch_instructions::jcond_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
                *condition,
            ),
            Operation::Call => crate::branch_instructions::call_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
            ),
            Operation::Ccond(condition) => crate::branch_instructions::ccond_instruction(
                state,
                additional_byte_1.unwrap(),
                additional_byte_2.unwrap(),
                *condition,
            ),
            Operation::Ret => crate::branch_instructions::ret_instruction(state),
            Operation::Rcond(condition) => {
                crate::branch_instructions::rcond_instruction(state, *condition)
            }
            Operation::Push(register_pair) => {
                crate::stack_instructions::push_instruction(state, *register_pair)
            }
            Operation::Pop(register_pair) => {
                crate::stack_instructions::pop_instruction(state, *register_pair)
            }
            Operation::PushPsw => {
                println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --")
            }
            Operation::PopPsw => {
                println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --")
            }
            Operation::In => {
                println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --")
            }
            Operation::Out => {
                println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --")
            }
            Operation::Ei => {
                crate::stack_instructions::ei_instruction(state)
            }
            Operation::Di => {
                crate::stack_instructions::di_instruction(state)
            }
            Operation::Hlt => {
                println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --")
            }
            Operation::Nop => (),
            _ => todo!("Running of instruction not yet implemented"),
        };
    }
}

#[cfg_attr(test, mutate)]
fn get_register_from_bit_pattern(bit_pattern: u8) -> Register {
    match bit_pattern {
        0b000 => Register::B,
        0b001 => Register::C,
        0b010 => Register::D,
        0b011 => Register::E,
        0b100 => Register::H,
        0b101 => Register::L,
        0b111 => Register::A,
        _ => panic!("Invalid bit pattern of {:#b} for register", bit_pattern),
    }
}

#[cfg_attr(test, mutate)]
fn get_condition_from_bit_pattern(bit_pattern: u8) -> Condition {
    match bit_pattern {
        0b000 => (ConditionFlag::Zero, false),
        0b001 => (ConditionFlag::Zero, true),
        0b010 => (ConditionFlag::Carry, false),
        0b011 => (ConditionFlag::Carry, true),
        0b100 => (ConditionFlag::Parity, false),
        0b101 => (ConditionFlag::Parity, true),
        0b110 => (ConditionFlag::Sign, false),
        0b111 => (ConditionFlag::Sign, true),
        _ => panic!("Invalid bit pattern of {:#b} for condition", bit_pattern),
    }
}

#[cfg_attr(test, mutate)]
fn get_register_pair_from_bit_pattern(bit_pattern: u8) -> RegisterPair {
    match bit_pattern {
        0b00 => RegisterPair::BC,
        0b01 => RegisterPair::DE,
        0b10 => RegisterPair::HL,
        0b11 => RegisterPair::SP,
        _ => panic!(
            "Invalid bit pattern of {:#b} for register pair",
            bit_pattern,
        ),
    }
}

#[allow(clippy::unusual_byte_groupings)]
#[cfg_attr(test, mutate)]
pub fn disassemble_op_code(op_code: u8) -> Operation {
    match op_code {
        0b01_000_000 => Operation::Mov(Register::B, Register::B),
        0b01_000_001 => Operation::Mov(Register::C, Register::B),
        0b01_000_010 => Operation::Mov(Register::D, Register::B),
        0b01_000_011 => Operation::Mov(Register::E, Register::B),
        0b01_000_100 => Operation::Mov(Register::H, Register::B),
        0b01_000_101 => Operation::Mov(Register::L, Register::B),
        0b01_000_111 => Operation::Mov(Register::A, Register::B),
        0b01_001_000 => Operation::Mov(Register::B, Register::C),
        0b01_001_001 => Operation::Mov(Register::C, Register::C),
        0b01_001_010 => Operation::Mov(Register::D, Register::C),
        0b01_001_011 => Operation::Mov(Register::E, Register::C),
        0b01_001_100 => Operation::Mov(Register::H, Register::C),
        0b01_001_101 => Operation::Mov(Register::L, Register::C),
        0b01_001_111 => Operation::Mov(Register::A, Register::C),
        0b01_010_000 => Operation::Mov(Register::B, Register::D),
        0b01_010_001 => Operation::Mov(Register::C, Register::D),
        0b01_010_010 => Operation::Mov(Register::D, Register::D),
        0b01_010_011 => Operation::Mov(Register::E, Register::D),
        0b01_010_100 => Operation::Mov(Register::H, Register::D),
        0b01_010_101 => Operation::Mov(Register::L, Register::D),
        0b01_010_111 => Operation::Mov(Register::A, Register::D),
        0b01_011_000 => Operation::Mov(Register::B, Register::E),
        0b01_011_001 => Operation::Mov(Register::C, Register::E),
        0b01_011_010 => Operation::Mov(Register::D, Register::E),
        0b01_011_011 => Operation::Mov(Register::E, Register::E),
        0b01_011_100 => Operation::Mov(Register::H, Register::E),
        0b01_011_101 => Operation::Mov(Register::L, Register::E),
        0b01_011_111 => Operation::Mov(Register::A, Register::E),
        0b01_100_000 => Operation::Mov(Register::B, Register::H),
        0b01_100_001 => Operation::Mov(Register::C, Register::H),
        0b01_100_010 => Operation::Mov(Register::D, Register::H),
        0b01_100_011 => Operation::Mov(Register::E, Register::H),
        0b01_100_100 => Operation::Mov(Register::H, Register::H),
        0b01_100_101 => Operation::Mov(Register::L, Register::H),
        0b01_100_111 => Operation::Mov(Register::A, Register::H),
        0b01_101_000 => Operation::Mov(Register::B, Register::L),
        0b01_101_001 => Operation::Mov(Register::C, Register::L),
        0b01_101_010 => Operation::Mov(Register::D, Register::L),
        0b01_101_011 => Operation::Mov(Register::E, Register::L),
        0b01_101_100 => Operation::Mov(Register::H, Register::L),
        0b01_101_101 => Operation::Mov(Register::L, Register::L),
        0b01_101_111 => Operation::Mov(Register::A, Register::L),
        0b01_111_000 => Operation::Mov(Register::B, Register::A),
        0b01_111_001 => Operation::Mov(Register::C, Register::A),
        0b01_111_010 => Operation::Mov(Register::D, Register::A),
        0b01_111_011 => Operation::Mov(Register::E, Register::A),
        0b01_111_100 => Operation::Mov(Register::H, Register::A),
        0b01_111_101 => Operation::Mov(Register::L, Register::A),
        0b01_111_111 => Operation::Mov(Register::A, Register::A),
        0b01_000_110 => Operation::MovFromMem(Register::B),
        0b01_001_110 => Operation::MovFromMem(Register::C),
        0b01_010_110 => Operation::MovFromMem(Register::D),
        0b01_011_110 => Operation::MovFromMem(Register::E),
        0b01_100_110 => Operation::MovFromMem(Register::H),
        0b01_101_110 => Operation::MovFromMem(Register::L),
        0b01_111_110 => Operation::MovFromMem(Register::A),
        0b01_110_000 => Operation::MovToMem(Register::B),
        0b01_110_001 => Operation::MovToMem(Register::C),
        0b01_110_010 => Operation::MovToMem(Register::D),
        0b01_110_011 => Operation::MovToMem(Register::E),
        0b01_110_100 => Operation::MovToMem(Register::H),
        0b01_110_101 => Operation::MovToMem(Register::L),
        0b01_110_111 => Operation::MovToMem(Register::A),
        0b00_000_110 => Operation::Mvi(Register::B),
        0b00_001_110 => Operation::Mvi(Register::C),
        0b00_010_110 => Operation::Mvi(Register::D),
        0b00_011_110 => Operation::Mvi(Register::E),
        0b00_100_110 => Operation::Mvi(Register::H),
        0b00_101_110 => Operation::Mvi(Register::L),
        0b00_111_110 => Operation::Mvi(Register::A),
        0b00_110_110 => Operation::MviMem,
        0b00_000_001 => Operation::Lxi(RegisterPair::BC),
        0b00_010_001 => Operation::Lxi(RegisterPair::DE),
        0b00_100_001 => Operation::Lxi(RegisterPair::HL),
        0b00_110_001 => Operation::Lxi(RegisterPair::SP),
        0b00_111_010 => Operation::Lda,
        0b00_110_010 => Operation::Sta,
        0b00_101_010 => Operation::Lhld,
        0b00_100_010 => Operation::Shld,
        0b00_001_010 => Operation::Ldax(RegisterPair::BC),
        0b00_011_010 => Operation::Ldax(RegisterPair::DE),
        0b00_000_010 => Operation::Stax(RegisterPair::BC),
        0b00_010_010 => Operation::Stax(RegisterPair::DE),
        0b11_101_011 => Operation::Xchg,
        0b10_000_000 => Operation::Add(Register::B),
        0b10_000_001 => Operation::Add(Register::C),
        0b10_000_010 => Operation::Add(Register::D),
        0b10_000_011 => Operation::Add(Register::E),
        0b10_000_100 => Operation::Add(Register::H),
        0b10_000_101 => Operation::Add(Register::L),
        0b10_000_111 => Operation::Add(Register::A),
        0b11_000_110 => Operation::Adi,
        0b11_001_110 => Operation::Aci,
        0b10_010_000 => Operation::Sub(Register::B),
        0b10_010_001 => Operation::Sub(Register::C),
        0b10_010_010 => Operation::Sub(Register::D),
        0b10_010_011 => Operation::Sub(Register::E),
        0b10_010_100 => Operation::Sub(Register::H),
        0b10_010_101 => Operation::Sub(Register::L),
        0b10_010_111 => Operation::Sub(Register::A),
        0b11_010_110 => Operation::Sui,
        0b11_011_110 => Operation::Sbi,
        0b00_000_100 => Operation::Inr(Register::B),
        0b00_001_100 => Operation::Inr(Register::C),
        0b00_010_100 => Operation::Inr(Register::D),
        0b00_011_100 => Operation::Inr(Register::E),
        0b00_100_100 => Operation::Inr(Register::H),
        0b00_101_100 => Operation::Inr(Register::L),
        0b00_111_100 => Operation::Inr(Register::A),
        0b00_000_101 => Operation::Dcr(Register::B),
        0b00_001_101 => Operation::Dcr(Register::C),
        0b00_010_101 => Operation::Dcr(Register::D),
        0b00_011_101 => Operation::Dcr(Register::E),
        0b00_100_101 => Operation::Dcr(Register::H),
        0b00_101_101 => Operation::Dcr(Register::L),
        0b00_111_101 => Operation::Dcr(Register::A),
        0b00_000_011 => Operation::Inx(RegisterPair::BC),
        0b00_010_011 => Operation::Inx(RegisterPair::DE),
        0b00_100_011 => Operation::Inx(RegisterPair::HL),
        0b00_110_011 => Operation::Inx(RegisterPair::SP),
        0b00_001_011 => Operation::Dcx(RegisterPair::BC),
        0b00_011_011 => Operation::Dcx(RegisterPair::DE),
        0b00_101_011 => Operation::Dcx(RegisterPair::HL),
        0b00_111_011 => Operation::Dcx(RegisterPair::SP),
        0b00_001_001 => Operation::Dad(RegisterPair::BC),
        0b00_011_001 => Operation::Dad(RegisterPair::DE),
        0b00_101_001 => Operation::Dad(RegisterPair::HL),
        0b00_111_001 => Operation::Dad(RegisterPair::SP),
        0b10_111_000 => Operation::Cmp(Register::B),
        0b10_111_001 => Operation::Cmp(Register::C),
        0b10_111_010 => Operation::Cmp(Register::D),
        0b10_111_011 => Operation::Cmp(Register::E),
        0b10_111_100 => Operation::Cmp(Register::H),
        0b10_111_101 => Operation::Cmp(Register::L),
        0b10_111_111 => Operation::Cmp(Register::A),
        0b10_100_000 => Operation::Ana(Register::B),
        0b10_100_001 => Operation::Ana(Register::C),
        0b10_100_010 => Operation::Ana(Register::D),
        0b10_100_011 => Operation::Ana(Register::E),
        0b10_100_100 => Operation::Ana(Register::H),
        0b10_100_101 => Operation::Ana(Register::L),
        0b10_100_111 => Operation::Ana(Register::A),
        0b11_100_110 => Operation::Ani,
        0b10_101_000 => Operation::Xra(Register::B),
        0b10_101_001 => Operation::Xra(Register::C),
        0b10_101_010 => Operation::Xra(Register::D),
        0b10_101_011 => Operation::Xra(Register::E),
        0b10_101_100 => Operation::Xra(Register::H),
        0b10_101_101 => Operation::Xra(Register::L),
        0b10_101_111 => Operation::Xra(Register::A),
        0b11_101_110 => Operation::Xri,
        0b10_110_000 => Operation::Ora(Register::B),
        0b10_110_001 => Operation::Ora(Register::C),
        0b10_110_010 => Operation::Ora(Register::D),
        0b10_110_011 => Operation::Ora(Register::E),
        0b10_110_100 => Operation::Ora(Register::H),
        0b10_110_101 => Operation::Ora(Register::L),
        0b10_110_111 => Operation::Ora(Register::A),
        0b11_110_110 => Operation::Ori,
        0b11_111_110 => Operation::Cpi,
        0b00_000_111 => Operation::Rlc,
        0b00_001_111 => Operation::Rrc,
        0b00_010_111 => Operation::Ral,
        0b00_011_111 => Operation::Rar,
        0b11_000_011 => Operation::Jmp,
        0b11_000_010 => Operation::Jcond((ConditionFlag::Zero, false)),
        0b11_001_010 => Operation::Jcond((ConditionFlag::Zero, true)),
        0b11_010_010 => Operation::Jcond((ConditionFlag::Carry, false)),
        0b11_011_010 => Operation::Jcond((ConditionFlag::Carry, true)),
        0b11_100_010 => Operation::Jcond((ConditionFlag::Parity, false)),
        0b11_101_010 => Operation::Jcond((ConditionFlag::Parity, true)),
        0b11_110_010 => Operation::Jcond((ConditionFlag::Sign, false)),
        0b11_111_010 => Operation::Jcond((ConditionFlag::Sign, true)),
        0b11_001_101 => Operation::Call,
        0b11_000_100 => Operation::Ccond((ConditionFlag::Zero, false)),
        0b11_001_100 => Operation::Ccond((ConditionFlag::Zero, true)),
        0b11_010_100 => Operation::Ccond((ConditionFlag::Carry, false)),
        0b11_011_100 => Operation::Ccond((ConditionFlag::Carry, true)),
        0b11_100_100 => Operation::Ccond((ConditionFlag::Parity, false)),
        0b11_101_100 => Operation::Ccond((ConditionFlag::Parity, true)),
        0b11_110_100 => Operation::Ccond((ConditionFlag::Sign, false)),
        0b11_111_100 => Operation::Ccond((ConditionFlag::Sign, true)),
        0b11_001_001 => Operation::Ret,
        0b11_000_000 => Operation::Rcond((ConditionFlag::Zero, false)),
        0b11_001_000 => Operation::Rcond((ConditionFlag::Zero, true)),
        0b11_010_000 => Operation::Rcond((ConditionFlag::Carry, false)),
        0b11_011_000 => Operation::Rcond((ConditionFlag::Carry, true)),
        0b11_100_000 => Operation::Rcond((ConditionFlag::Parity, false)),
        0b11_101_000 => Operation::Rcond((ConditionFlag::Parity, true)),
        0b11_110_000 => Operation::Rcond((ConditionFlag::Sign, false)),
        0b11_111_000 => Operation::Rcond((ConditionFlag::Sign, true)),
        0b11_000_101 => Operation::Push(RegisterPair::BC),
        0b11_010_101 => Operation::Push(RegisterPair::DE),
        0b11_100_101 => Operation::Push(RegisterPair::HL),
        0b11_110_101 => Operation::PushPsw,
        0b11_000_001 => Operation::Pop(RegisterPair::BC),
        0b11_010_001 => Operation::Pop(RegisterPair::DE),
        0b11_100_001 => Operation::Pop(RegisterPair::HL),
        0b11_110_001 => Operation::PopPsw,
        0b11_011_011 => Operation::In,
        0b11_010_011 => Operation::Out,
        0b11_111_011 => Operation::Ei,
        0b11_110_011 => Operation::Di,
        0b01_110_110 => Operation::Hlt,
        0b00_000_000 => Operation::Nop,
        _ => panic!("Unrecognized opcode: {:#010b}", op_code),
    }
}

#[allow(clippy::unusual_byte_groupings)]
#[cfg(test)]
mod tests {
    use crate::disassembler::{
        get_condition_from_bit_pattern, get_register_from_bit_pattern,
        get_register_pair_from_bit_pattern, Operation,
    };
    use crate::{Condition, Register, RegisterPair};
    use std::collections::HashMap;

    fn assert_operation_equals_expected(operation: &Operation, expected_operation: &Operation) {
        assert_eq!(
            operation, expected_operation,
            "Expected operation to be {:?}, but instead it was {:?}",
            expected_operation, operation
        );
    }

    fn get_all_combinations_for_op_codes<F, T>(
        base_op_code: u8,
        lowest_bit_offset: u8,
        bit_patterns: Vec<u8>,
        combination_function: F,
    ) -> HashMap<u8, T>
    where
        F: Fn(u8) -> T,
    {
        let mut combination_map = HashMap::with_capacity(bit_patterns.len());

        for bit_pattern in bit_patterns {
            let op_code = base_op_code | bit_pattern << lowest_bit_offset;
            let combination = combination_function(bit_pattern);
            combination_map.insert(op_code, combination);
        }

        combination_map
    }

    fn get_all_registers_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, Register> {
        let bit_patterns = vec![0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111];
        get_all_combinations_for_op_codes(
            base_op_code,
            lowest_bit_offset,
            bit_patterns,
            get_register_from_bit_pattern,
        )
    }

    fn get_all_conditions_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, Condition> {
        let bit_patterns = vec![0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
        get_all_combinations_for_op_codes(
            base_op_code,
            lowest_bit_offset,
            bit_patterns,
            get_condition_from_bit_pattern,
        )
    }

    fn get_all_register_pairs_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, RegisterPair> {
        get_all_register_pairs_for_op_codes_with_exclusions(
            base_op_code,
            lowest_bit_offset,
            Vec::new(),
        )
    }

    fn get_all_register_pairs_for_op_codes_with_exclusions(
        base_op_code: u8,
        lowest_bit_offset: u8,
        exclusions: Vec<u8>,
    ) -> HashMap<u8, RegisterPair> {
        let mut bit_patterns = vec![0b00, 0b01, 0b10, 0b11];
        bit_patterns.retain(|bp| !exclusions.contains(bp));
        get_all_combinations_for_op_codes(
            base_op_code,
            lowest_bit_offset,
            bit_patterns,
            get_register_pair_from_bit_pattern,
        )
    }

    #[test]
    fn disassembler_handles_mov() {
        let source_register_map = get_all_registers_for_op_codes(0b01_000_000, 0);

        for (interim_op_code, source_register) in source_register_map {
            let destination_register_map = get_all_registers_for_op_codes(interim_op_code, 3);

            for (op_code, destination_register) in destination_register_map {
                let operation = crate::disassembler::disassemble_op_code(op_code);
                assert_operation_equals_expected(
                    &operation,
                    &Operation::Mov(source_register, destination_register),
                );
            }
        }
    }

    #[test]
    fn disassembler_handles_mov_from_mem() {
        let register_map = get_all_registers_for_op_codes(0b01_000_110, 3);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::MovFromMem(register));
        }
    }

    #[test]
    fn disassembler_handles_mov_to_mem() {
        let register_map = get_all_registers_for_op_codes(0b01_110_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::MovToMem(register));
        }
    }

    #[test]
    fn disassembler_handles_mvi() {
        let register_map = get_all_registers_for_op_codes(0b00_000_110, 3);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Mvi(register));
        }
    }

    #[test]
    fn disassembler_handles_mvi_mem() {
        let operation = crate::disassembler::disassemble_op_code(0b00_110_110);
        assert_operation_equals_expected(&operation, &Operation::MviMem);
    }

    #[test]
    fn disassembler_handles_lxi() {
        let register_pair_map = get_all_register_pairs_for_op_codes(0b00_000_001, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Lxi(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_lda() {
        let operation = crate::disassembler::disassemble_op_code(0b00_111_010);
        assert_operation_equals_expected(&operation, &Operation::Lda);
    }

    #[test]
    fn disassembler_handles_sta() {
        let operation = crate::disassembler::disassemble_op_code(0b00_110_010);
        assert_operation_equals_expected(&operation, &Operation::Sta);
    }

    #[test]
    fn disassembler_handles_lhld() {
        let operation = crate::disassembler::disassemble_op_code(0b00_101_010);
        assert_operation_equals_expected(&operation, &Operation::Lhld);
    }

    #[test]
    fn disassembler_handles_shld() {
        let operation = crate::disassembler::disassemble_op_code(0b00_100_010);
        assert_operation_equals_expected(&operation, &Operation::Shld);
    }

    #[test]
    fn disassembler_handles_ldax() {
        let register_pair_map =
            get_all_register_pairs_for_op_codes_with_exclusions(0b00_001_010, 4, vec![0b10, 0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ldax(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_stax() {
        let register_pair_map =
            get_all_register_pairs_for_op_codes_with_exclusions(0b00_000_010, 4, vec![0b10, 0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Stax(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_xchg() {
        let operation = crate::disassembler::disassemble_op_code(0b11_101_011);
        assert_operation_equals_expected(&operation, &Operation::Xchg);
    }

    #[test]
    fn disassembler_handles_add() {
        let register_map = get_all_registers_for_op_codes(0b10_000_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Add(register));
        }
    }

    #[test]
    fn disassembler_handles_adi() {
        let operation = crate::disassembler::disassemble_op_code(0b11_000_110);
        assert_operation_equals_expected(&operation, &Operation::Adi);
    }

    #[test]
    fn disassembler_handles_aci() {
        let operation = crate::disassembler::disassemble_op_code(0b11_001_110);
        assert_operation_equals_expected(&operation, &Operation::Aci);
    }

    #[test]
    fn disassembler_handles_sub() {
        let register_map = get_all_registers_for_op_codes(0b10_010_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Sub(register));
        }
    }

    #[test]
    fn disassembler_handles_sui() {
        let operation = crate::disassembler::disassemble_op_code(0b11_010_110);
        assert_operation_equals_expected(&operation, &Operation::Sui);
    }

    #[test]
    fn disassembler_handles_sbi() {
        let operation = crate::disassembler::disassemble_op_code(0b11_011_110);
        assert_operation_equals_expected(&operation, &Operation::Sbi);
    }

    #[test]
    fn disassembler_handles_inr() {
        let register_map = get_all_registers_for_op_codes(0b00_000_100, 3);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Inr(register));
        }
    }

    #[test]
    fn disassembler_handles_dcr() {
        let register_map = get_all_registers_for_op_codes(0b00_000_101, 3);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Dcr(register));
        }
    }

    #[test]
    fn disassembler_handles_inx() {
        let register_pair_map = get_all_register_pairs_for_op_codes(0b00_000_011, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Inx(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_dcx() {
        let register_pair_map = get_all_register_pairs_for_op_codes(0b00_001_011, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Dcx(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_dad() {
        let register_pair_map = get_all_register_pairs_for_op_codes(0b00_001_001, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Dad(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_cmp() {
        let register_map = get_all_registers_for_op_codes(0b10_111_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Cmp(register));
        }
    }

    #[test]
    fn disassembler_handles_ana() {
        let register_map = get_all_registers_for_op_codes(0b10_100_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ana(register));
        }
    }

    #[test]
    fn disassembler_handles_ani() {
        let operation = crate::disassembler::disassemble_op_code(0b11_100_110);
        assert_operation_equals_expected(&operation, &Operation::Ani);
    }

    #[test]
    fn disassembler_handles_xra() {
        let register_map = get_all_registers_for_op_codes(0b10_101_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Xra(register));
        }
    }

    #[test]
    fn disassembler_handles_xri() {
        let operation = crate::disassembler::disassemble_op_code(0b11_101_110);
        assert_operation_equals_expected(&operation, &Operation::Xri);
    }

    #[test]
    fn disassembler_handles_ora() {
        let register_map = get_all_registers_for_op_codes(0b10_110_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ora(register));
        }
    }

    #[test]
    fn disassembler_handles_ori() {
        let operation = crate::disassembler::disassemble_op_code(0b11_110_110);
        assert_operation_equals_expected(&operation, &Operation::Ori);
    }

    #[test]
    fn disassembler_handles_cpi() {
        let operation = crate::disassembler::disassemble_op_code(0b11_111_110);
        assert_operation_equals_expected(&operation, &Operation::Cpi);
    }

    #[test]
    fn disassembler_handles_rlc() {
        let operation = crate::disassembler::disassemble_op_code(0b00_000_111);
        assert_operation_equals_expected(&operation, &Operation::Rlc);
    }

    #[test]
    fn disassembler_handles_rrc() {
        let operation = crate::disassembler::disassemble_op_code(0b00_001_111);
        assert_operation_equals_expected(&operation, &Operation::Rrc);
    }

    #[test]
    fn disassembler_handles_ral() {
        let operation = crate::disassembler::disassemble_op_code(0b00_010_111);
        assert_operation_equals_expected(&operation, &Operation::Ral);
    }

    #[test]
    fn disassembler_handles_rar() {
        let operation = crate::disassembler::disassemble_op_code(0b00_011_111);
        assert_operation_equals_expected(&operation, &Operation::Rar);
    }

    #[test]
    fn disassembler_handles_jmp() {
        let operation = crate::disassembler::disassemble_op_code(0b11_000_011);
        assert_operation_equals_expected(&operation, &Operation::Jmp);
    }

    #[test]
    fn disassembler_handles_jcond() {
        let condition_map = get_all_conditions_for_op_codes(0b11_000_010, 3);

        for (op_code, condition) in condition_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Jcond(condition));
        }
    }

    #[test]
    fn disassembler_handles_call() {
        let operation = crate::disassembler::disassemble_op_code(0b11_001_101);
        assert_operation_equals_expected(&operation, &Operation::Call);
    }

    #[test]
    fn disassembler_handles_ccond() {
        let condition_map = get_all_conditions_for_op_codes(0b11_000_100, 3);

        for (op_code, condition) in condition_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ccond(condition));
        }
    }

    #[test]
    fn disassembler_handles_ret() {
        let operation = crate::disassembler::disassemble_op_code(0b11_001_001);
        assert_operation_equals_expected(&operation, &Operation::Ret);
    }

    #[test]
    fn disassembler_handles_rcond() {
        let condition_map = get_all_conditions_for_op_codes(0b11_000_000, 3);

        for (op_code, condition) in condition_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Rcond(condition));
        }
    }

    #[test]
    fn disassembler_handles_push() {
        let register_pair_map =
            get_all_register_pairs_for_op_codes_with_exclusions(0b11_000_101, 4, vec![0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Push(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_push_psw() {
        let operation = crate::disassembler::disassemble_op_code(0b11_110_101);
        assert_operation_equals_expected(&operation, &Operation::PushPsw);
    }

    #[test]
    fn disassembler_handles_pop() {
        let register_pair_map =
            get_all_register_pairs_for_op_codes_with_exclusions(0b11_000_001, 4, vec![0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Pop(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_pop_psw() {
        let operation = crate::disassembler::disassemble_op_code(0b11_110_001);
        assert_operation_equals_expected(&operation, &Operation::PopPsw);
    }

    #[test]
    fn disassembler_handles_in() {
        let operation = crate::disassembler::disassemble_op_code(0b11_011_011);
        assert_operation_equals_expected(&operation, &Operation::In);
    }

    #[test]
    fn disassembler_handles_out() {
        let operation = crate::disassembler::disassemble_op_code(0b11_010_011);
        assert_operation_equals_expected(&operation, &Operation::Out);
    }

    #[test]
    fn disassembler_handles_ei() {
        let operation = crate::disassembler::disassemble_op_code(0b11_111_011);
        assert_operation_equals_expected(&operation, &Operation::Ei);
    }

    #[test]
    fn disassembler_handles_di() {
        let operation = crate::disassembler::disassemble_op_code(0b11_110_011);
        assert_operation_equals_expected(&operation, &Operation::Di);
    }

    #[test]
    fn disassembler_handles_hlt() {
        let operation = crate::disassembler::disassemble_op_code(0b01_110_110);
        assert_operation_equals_expected(&operation, &Operation::Hlt);
    }

    #[test]
    fn disassembler_handles_nop() {
        let operation = crate::disassembler::disassemble_op_code(0b00_000_000);
        assert_operation_equals_expected(&operation, &Operation::Nop);
    }
}
