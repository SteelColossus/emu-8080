use crate::{Condition, ConditionFlag, Register};
#[cfg(test)]
use mutagen::mutate;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Nop,
    Hlt,
    Di,
    Ei,
    Add(Register),
    Sub(Register),
    Mvi(Register),
    Mov(Register, Register),
    Jmp,
    Jcond(Condition),
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
        _ => panic!("Invalid bit pattern of {:#b}", bit_pattern),
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
        _ => panic!("Invalid bit pattern of {:#b}", bit_pattern),
    }
}

#[cfg_attr(test, mutate)]
fn disassemble_op_code(op_code: u8) -> Operation {
    match op_code {
        0b10_000_000 => Operation::Add(Register::B),
        0b10_000_001 => Operation::Add(Register::C),
        0b10_000_010 => Operation::Add(Register::D),
        0b10_000_011 => Operation::Add(Register::E),
        0b10_000_100 => Operation::Add(Register::H),
        0b10_000_101 => Operation::Add(Register::L),
        0b10_000_111 => Operation::Add(Register::A),
        0b10_010_000 => Operation::Sub(Register::B),
        0b10_010_001 => Operation::Sub(Register::C),
        0b10_010_010 => Operation::Sub(Register::D),
        0b10_010_011 => Operation::Sub(Register::E),
        0b10_010_100 => Operation::Sub(Register::H),
        0b10_010_101 => Operation::Sub(Register::L),
        0b10_010_111 => Operation::Sub(Register::A),
        0b00_000_110 => Operation::Mvi(Register::B),
        0b00_001_110 => Operation::Mvi(Register::C),
        0b00_010_110 => Operation::Mvi(Register::D),
        0b00_011_110 => Operation::Mvi(Register::E),
        0b00_100_110 => Operation::Mvi(Register::H),
        0b00_101_110 => Operation::Mvi(Register::L),
        0b00_111_110 => Operation::Mvi(Register::A),
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
        0b11_000_011 => Operation::Jmp,
        0b11_000_010 => Operation::Jcond((ConditionFlag::Zero, false)),
        0b11_001_010 => Operation::Jcond((ConditionFlag::Zero, true)),
        0b11_010_010 => Operation::Jcond((ConditionFlag::Carry, false)),
        0b11_011_010 => Operation::Jcond((ConditionFlag::Carry, true)),
        0b11_100_010 => Operation::Jcond((ConditionFlag::Parity, false)),
        0b11_101_010 => Operation::Jcond((ConditionFlag::Parity, true)),
        0b11_110_010 => Operation::Jcond((ConditionFlag::Sign, false)),
        0b11_111_010 => Operation::Jcond((ConditionFlag::Sign, true)),
        0b00_000_000 => Operation::Nop,
        0b01_110_110 => Operation::Hlt,
        0b11_110_011 => Operation::Di,
        0b11_111_011 => Operation::Ei,
        _ => panic!("Unrecognized opcode: {:#010b}", op_code),
    }
}

#[cfg(test)]
mod tests {
    use crate::disassembler::{
        get_condition_from_bit_pattern, get_register_from_bit_pattern, Operation,
    };
    use crate::{Condition, Register};
    use std::collections::HashMap;

    fn assert_operation_equals_expected(operation: &Operation, expected_operation: &Operation) {
        assert_eq!(
            operation, expected_operation,
            "Expected operation to be {:?}, but instead it was {:?}",
            expected_operation, operation
        );
    }

    fn get_all_registers_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, Register> {
        let bit_patterns = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111];
        let mut register_map = HashMap::with_capacity(bit_patterns.len());

        for bit_pattern in bit_patterns {
            let op_code = base_op_code | bit_pattern << lowest_bit_offset;
            let register = get_register_from_bit_pattern(bit_pattern);
            register_map.insert(op_code, register);
        }

        register_map
    }

    fn get_all_conditions_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, Condition> {
        let bit_patterns = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
        let mut condition_map = HashMap::with_capacity(bit_patterns.len());

        for bit_pattern in bit_patterns {
            let op_code = base_op_code | bit_pattern << lowest_bit_offset;
            let condition = get_condition_from_bit_pattern(bit_pattern);
            condition_map.insert(op_code, condition);
        }

        condition_map
    }

    #[test]
    fn disassembler_handles_nop() {
        let operation = crate::disassembler::disassemble_op_code(0b00_000_000);
        assert_operation_equals_expected(&operation, &Operation::Nop);
    }

    #[test]
    fn disassembler_handles_hlt() {
        let operation = crate::disassembler::disassemble_op_code(0b01_110_110);
        assert_operation_equals_expected(&operation, &Operation::Hlt);
    }

    #[test]
    fn disassembler_handles_di() {
        let operation = crate::disassembler::disassemble_op_code(0b11_110_011);
        assert_operation_equals_expected(&operation, &Operation::Di);
    }

    #[test]
    fn disassembler_handles_ei() {
        let operation = crate::disassembler::disassemble_op_code(0b11_111_011);
        assert_operation_equals_expected(&operation, &Operation::Ei);
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
    fn disassembler_handles_sub() {
        let register_map = get_all_registers_for_op_codes(0b10_010_000, 0);

        for (op_code, register) in register_map {
            let operation = crate::disassembler::disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Sub(register));
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
}
