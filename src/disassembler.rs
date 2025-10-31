use crate::{ConditionFlag, Operation, Register, RegisterPair};
// #[cfg(test)]
// use mutagen::mutate;

#[allow(clippy::unusual_byte_groupings)]
// #[cfg_attr(test, mutate)]
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
        0b10_000_110 => Operation::AddMem,
        0b11_000_110 => Operation::Adi,
        0b10_001_000 => Operation::Adc(Register::B),
        0b10_001_001 => Operation::Adc(Register::C),
        0b10_001_010 => Operation::Adc(Register::D),
        0b10_001_011 => Operation::Adc(Register::E),
        0b10_001_100 => Operation::Adc(Register::H),
        0b10_001_101 => Operation::Adc(Register::L),
        0b10_001_111 => Operation::Adc(Register::A),
        0b10_001_110 => Operation::AdcMem,
        0b11_001_110 => Operation::Aci,
        0b10_010_000 => Operation::Sub(Register::B),
        0b10_010_001 => Operation::Sub(Register::C),
        0b10_010_010 => Operation::Sub(Register::D),
        0b10_010_011 => Operation::Sub(Register::E),
        0b10_010_100 => Operation::Sub(Register::H),
        0b10_010_101 => Operation::Sub(Register::L),
        0b10_010_111 => Operation::Sub(Register::A),
        0b10_010_110 => Operation::SubMem,
        0b11_010_110 => Operation::Sui,
        0b10_011_000 => Operation::Sbb(Register::B),
        0b10_011_001 => Operation::Sbb(Register::C),
        0b10_011_010 => Operation::Sbb(Register::D),
        0b10_011_011 => Operation::Sbb(Register::E),
        0b10_011_100 => Operation::Sbb(Register::H),
        0b10_011_101 => Operation::Sbb(Register::L),
        0b10_011_111 => Operation::Sbb(Register::A),
        0b10_011_110 => Operation::SbbMem,
        0b11_011_110 => Operation::Sbi,
        0b00_000_100 => Operation::Inr(Register::B),
        0b00_001_100 => Operation::Inr(Register::C),
        0b00_010_100 => Operation::Inr(Register::D),
        0b00_011_100 => Operation::Inr(Register::E),
        0b00_100_100 => Operation::Inr(Register::H),
        0b00_101_100 => Operation::Inr(Register::L),
        0b00_111_100 => Operation::Inr(Register::A),
        0b00_110_100 => Operation::InrMem,
        0b00_000_101 => Operation::Dcr(Register::B),
        0b00_001_101 => Operation::Dcr(Register::C),
        0b00_010_101 => Operation::Dcr(Register::D),
        0b00_011_101 => Operation::Dcr(Register::E),
        0b00_100_101 => Operation::Dcr(Register::H),
        0b00_101_101 => Operation::Dcr(Register::L),
        0b00_111_101 => Operation::Dcr(Register::A),
        0b00_110_101 => Operation::DcrMem,
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
        0b00_100_111 => Operation::Daa,
        0b10_100_000 => Operation::Ana(Register::B),
        0b10_100_001 => Operation::Ana(Register::C),
        0b10_100_010 => Operation::Ana(Register::D),
        0b10_100_011 => Operation::Ana(Register::E),
        0b10_100_100 => Operation::Ana(Register::H),
        0b10_100_101 => Operation::Ana(Register::L),
        0b10_100_111 => Operation::Ana(Register::A),
        0b10_100_110 => Operation::AnaMem,
        0b11_100_110 => Operation::Ani,
        0b10_101_000 => Operation::Xra(Register::B),
        0b10_101_001 => Operation::Xra(Register::C),
        0b10_101_010 => Operation::Xra(Register::D),
        0b10_101_011 => Operation::Xra(Register::E),
        0b10_101_100 => Operation::Xra(Register::H),
        0b10_101_101 => Operation::Xra(Register::L),
        0b10_101_111 => Operation::Xra(Register::A),
        0b10_101_110 => Operation::XraMem,
        0b11_101_110 => Operation::Xri,
        0b10_110_000 => Operation::Ora(Register::B),
        0b10_110_001 => Operation::Ora(Register::C),
        0b10_110_010 => Operation::Ora(Register::D),
        0b10_110_011 => Operation::Ora(Register::E),
        0b10_110_100 => Operation::Ora(Register::H),
        0b10_110_101 => Operation::Ora(Register::L),
        0b10_110_111 => Operation::Ora(Register::A),
        0b10_110_110 => Operation::OraMem,
        0b11_110_110 => Operation::Ori,
        0b10_111_000 => Operation::Cmp(Register::B),
        0b10_111_001 => Operation::Cmp(Register::C),
        0b10_111_010 => Operation::Cmp(Register::D),
        0b10_111_011 => Operation::Cmp(Register::E),
        0b10_111_100 => Operation::Cmp(Register::H),
        0b10_111_101 => Operation::Cmp(Register::L),
        0b10_111_111 => Operation::Cmp(Register::A),
        0b10_111_110 => Operation::CmpMem,
        0b11_111_110 => Operation::Cpi,
        0b00_000_111 => Operation::Rlc,
        0b00_001_111 => Operation::Rrc,
        0b00_010_111 => Operation::Ral,
        0b00_011_111 => Operation::Rar,
        0b00_101_111 => Operation::Cma,
        0b00_111_111 => Operation::Cmc,
        0b00_110_111 => Operation::Stc,
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
        0b11_000_111 => Operation::Rst(0),
        0b11_001_111 => Operation::Rst(1),
        0b11_010_111 => Operation::Rst(2),
        0b11_011_111 => Operation::Rst(3),
        0b11_100_111 => Operation::Rst(4),
        0b11_101_111 => Operation::Rst(5),
        0b11_110_111 => Operation::Rst(6),
        0b11_111_111 => Operation::Rst(7),
        0b11_101_001 => Operation::Pchl,
        0b11_000_101 => Operation::Push(RegisterPair::BC),
        0b11_010_101 => Operation::Push(RegisterPair::DE),
        0b11_100_101 => Operation::Push(RegisterPair::HL),
        0b11_110_101 => Operation::PushPsw,
        0b11_000_001 => Operation::Pop(RegisterPair::BC),
        0b11_010_001 => Operation::Pop(RegisterPair::DE),
        0b11_100_001 => Operation::Pop(RegisterPair::HL),
        0b11_110_001 => Operation::PopPsw,
        0b11_100_011 => Operation::Xthl,
        0b11_111_001 => Operation::Sphl,
        0b11_011_011 => Operation::In,
        0b11_010_011 => Operation::Out,
        0b11_111_011 => Operation::Ei,
        0b11_110_011 => Operation::Di,
        0b01_110_110 => Operation::Hlt,
        0b00_000_000 => Operation::Nop,
        _ => panic!("Unrecognized opcode: {op_code:#010b}"),
    }
}

#[allow(clippy::unusual_byte_groupings)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Condition;
    use std::collections::HashMap;

    fn register_from_bit_pattern(bit_pattern: u8) -> Register {
        match bit_pattern {
            0b000 => Register::B,
            0b001 => Register::C,
            0b010 => Register::D,
            0b011 => Register::E,
            0b100 => Register::H,
            0b101 => Register::L,
            0b111 => Register::A,
            _ => panic!("Invalid bit pattern of {bit_pattern:#b} for register"),
        }
    }

    fn condition_from_bit_pattern(bit_pattern: u8) -> Condition {
        match bit_pattern {
            0b000 => (ConditionFlag::Zero, false),
            0b001 => (ConditionFlag::Zero, true),
            0b010 => (ConditionFlag::Carry, false),
            0b011 => (ConditionFlag::Carry, true),
            0b100 => (ConditionFlag::Parity, false),
            0b101 => (ConditionFlag::Parity, true),
            0b110 => (ConditionFlag::Sign, false),
            0b111 => (ConditionFlag::Sign, true),
            _ => panic!("Invalid bit pattern of {bit_pattern:#b} for condition"),
        }
    }

    fn register_pair_from_bit_pattern(bit_pattern: u8) -> RegisterPair {
        match bit_pattern {
            0b00 => RegisterPair::BC,
            0b01 => RegisterPair::DE,
            0b10 => RegisterPair::HL,
            0b11 => RegisterPair::SP,
            _ => panic!("Invalid bit pattern of {bit_pattern:#b} for register pair"),
        }
    }

    fn assert_operation_equals_expected(operation: &Operation, expected_operation: &Operation) {
        assert_eq!(
            operation, expected_operation,
            "Expected operation to be {expected_operation:?}, but instead it was {operation:?}",
        );
    }

    fn all_combinations_for_op_codes<F, T>(
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

    fn all_registers_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, Register> {
        let bit_patterns = vec![0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111];
        all_combinations_for_op_codes(
            base_op_code,
            lowest_bit_offset,
            bit_patterns,
            register_from_bit_pattern,
        )
    }

    fn all_conditions_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, Condition> {
        let bit_patterns = vec![0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111];
        all_combinations_for_op_codes(
            base_op_code,
            lowest_bit_offset,
            bit_patterns,
            condition_from_bit_pattern,
        )
    }

    fn all_register_pairs_for_op_codes(
        base_op_code: u8,
        lowest_bit_offset: u8,
    ) -> HashMap<u8, RegisterPair> {
        all_register_pairs_for_op_codes_with_exclusions(base_op_code, lowest_bit_offset, Vec::new())
    }

    fn all_register_pairs_for_op_codes_with_exclusions(
        base_op_code: u8,
        lowest_bit_offset: u8,
        exclusions: Vec<u8>,
    ) -> HashMap<u8, RegisterPair> {
        let mut bit_patterns = vec![0b00, 0b01, 0b10, 0b11];
        bit_patterns.retain(|bp| !exclusions.contains(bp));
        all_combinations_for_op_codes(
            base_op_code,
            lowest_bit_offset,
            bit_patterns,
            register_pair_from_bit_pattern,
        )
    }

    #[test]
    #[should_panic(expected = "Unrecognized opcode: 0b00001000")]
    fn disassembler_panics_on_unsupported_op_code() {
        disassemble_op_code(0b00_001_000);
    }

    #[test]
    fn disassembler_handles_mov() {
        let source_register_map = all_registers_for_op_codes(0b01_000_000, 0);

        for (interim_op_code, source_register) in source_register_map {
            let destination_register_map = all_registers_for_op_codes(interim_op_code, 3);

            for (op_code, destination_register) in destination_register_map {
                let operation = disassemble_op_code(op_code);
                assert_operation_equals_expected(
                    &operation,
                    &Operation::Mov(source_register, destination_register),
                );
            }
        }
    }

    #[test]
    fn disassembler_handles_mov_from_mem() {
        let register_map = all_registers_for_op_codes(0b01_000_110, 3);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::MovFromMem(register));
        }
    }

    #[test]
    fn disassembler_handles_mov_to_mem() {
        let register_map = all_registers_for_op_codes(0b01_110_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::MovToMem(register));
        }
    }

    #[test]
    fn disassembler_handles_mvi() {
        let register_map = all_registers_for_op_codes(0b00_000_110, 3);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Mvi(register));
        }
    }

    #[test]
    fn disassembler_handles_mvi_mem() {
        let operation = disassemble_op_code(0b00_110_110);
        assert_operation_equals_expected(&operation, &Operation::MviMem);
    }

    #[test]
    fn disassembler_handles_lxi() {
        let register_pair_map = all_register_pairs_for_op_codes(0b00_000_001, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Lxi(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_lda() {
        let operation = disassemble_op_code(0b00_111_010);
        assert_operation_equals_expected(&operation, &Operation::Lda);
    }

    #[test]
    fn disassembler_handles_sta() {
        let operation = disassemble_op_code(0b00_110_010);
        assert_operation_equals_expected(&operation, &Operation::Sta);
    }

    #[test]
    fn disassembler_handles_lhld() {
        let operation = disassemble_op_code(0b00_101_010);
        assert_operation_equals_expected(&operation, &Operation::Lhld);
    }

    #[test]
    fn disassembler_handles_shld() {
        let operation = disassemble_op_code(0b00_100_010);
        assert_operation_equals_expected(&operation, &Operation::Shld);
    }

    #[test]
    fn disassembler_handles_ldax() {
        let register_pair_map =
            all_register_pairs_for_op_codes_with_exclusions(0b00_001_010, 4, vec![0b10, 0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ldax(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_stax() {
        let register_pair_map =
            all_register_pairs_for_op_codes_with_exclusions(0b00_000_010, 4, vec![0b10, 0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Stax(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_xchg() {
        let operation = disassemble_op_code(0b11_101_011);
        assert_operation_equals_expected(&operation, &Operation::Xchg);
    }

    #[test]
    fn disassembler_handles_add() {
        let register_map = all_registers_for_op_codes(0b10_000_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Add(register));
        }
    }

    #[test]
    fn disassembler_handles_add_mem() {
        let operation = disassemble_op_code(0b10_000_110);
        assert_operation_equals_expected(&operation, &Operation::AddMem);
    }

    #[test]
    fn disassembler_handles_adi() {
        let operation = disassemble_op_code(0b11_000_110);
        assert_operation_equals_expected(&operation, &Operation::Adi);
    }

    #[test]
    fn disassembler_handles_adc() {
        let register_map = all_registers_for_op_codes(0b10_001_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Adc(register));
        }
    }

    #[test]
    fn disassembler_handles_adc_mem() {
        let operation = disassemble_op_code(0b10_001_110);
        assert_operation_equals_expected(&operation, &Operation::AdcMem);
    }

    #[test]
    fn disassembler_handles_aci() {
        let operation = disassemble_op_code(0b11_001_110);
        assert_operation_equals_expected(&operation, &Operation::Aci);
    }

    #[test]
    fn disassembler_handles_sub() {
        let register_map = all_registers_for_op_codes(0b10_010_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Sub(register));
        }
    }

    #[test]
    fn disassembler_handles_sub_mem() {
        let operation = disassemble_op_code(0b10_010_110);
        assert_operation_equals_expected(&operation, &Operation::SubMem);
    }

    #[test]
    fn disassembler_handles_sui() {
        let operation = disassemble_op_code(0b11_010_110);
        assert_operation_equals_expected(&operation, &Operation::Sui);
    }

    #[test]
    fn disassembler_handles_sbb() {
        let register_map = all_registers_for_op_codes(0b10_011_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Sbb(register));
        }
    }

    #[test]
    fn disassembler_handles_sbb_mem() {
        let operation = disassemble_op_code(0b10_011_110);
        assert_operation_equals_expected(&operation, &Operation::SbbMem);
    }

    #[test]
    fn disassembler_handles_sbi() {
        let operation = disassemble_op_code(0b11_011_110);
        assert_operation_equals_expected(&operation, &Operation::Sbi);
    }

    #[test]
    fn disassembler_handles_inr() {
        let register_map = all_registers_for_op_codes(0b00_000_100, 3);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Inr(register));
        }
    }

    #[test]
    fn disassembler_handles_inr_mem() {
        let operation = disassemble_op_code(0b00_110_100);
        assert_operation_equals_expected(&operation, &Operation::InrMem);
    }

    #[test]
    fn disassembler_handles_dcr() {
        let register_map = all_registers_for_op_codes(0b00_000_101, 3);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Dcr(register));
        }
    }

    #[test]
    fn disassembler_handles_dcr_mem() {
        let operation = disassemble_op_code(0b00_110_101);
        assert_operation_equals_expected(&operation, &Operation::DcrMem);
    }

    #[test]
    fn disassembler_handles_inx() {
        let register_pair_map = all_register_pairs_for_op_codes(0b00_000_011, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Inx(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_dcx() {
        let register_pair_map = all_register_pairs_for_op_codes(0b00_001_011, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Dcx(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_dad() {
        let register_pair_map = all_register_pairs_for_op_codes(0b00_001_001, 4);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Dad(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_daa() {
        let operation = disassemble_op_code(0b00_100_111);
        assert_operation_equals_expected(&operation, &Operation::Daa);
    }

    #[test]
    fn disassembler_handles_ana() {
        let register_map = all_registers_for_op_codes(0b10_100_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ana(register));
        }
    }

    #[test]
    fn disassembler_handles_ana_mem() {
        let operation = disassemble_op_code(0b10_100_110);
        assert_operation_equals_expected(&operation, &Operation::AnaMem);
    }

    #[test]
    fn disassembler_handles_ani() {
        let operation = disassemble_op_code(0b11_100_110);
        assert_operation_equals_expected(&operation, &Operation::Ani);
    }

    #[test]
    fn disassembler_handles_xra() {
        let register_map = all_registers_for_op_codes(0b10_101_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Xra(register));
        }
    }

    #[test]
    fn disassembler_handles_xra_mem() {
        let operation = disassemble_op_code(0b10_101_110);
        assert_operation_equals_expected(&operation, &Operation::XraMem);
    }

    #[test]
    fn disassembler_handles_xri() {
        let operation = disassemble_op_code(0b11_101_110);
        assert_operation_equals_expected(&operation, &Operation::Xri);
    }

    #[test]
    fn disassembler_handles_ora() {
        let register_map = all_registers_for_op_codes(0b10_110_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ora(register));
        }
    }

    #[test]
    fn disassembler_handles_ora_mem() {
        let operation = disassemble_op_code(0b10_110_110);
        assert_operation_equals_expected(&operation, &Operation::OraMem);
    }

    #[test]
    fn disassembler_handles_ori() {
        let operation = disassemble_op_code(0b11_110_110);
        assert_operation_equals_expected(&operation, &Operation::Ori);
    }

    #[test]
    fn disassembler_handles_cmp() {
        let register_map = all_registers_for_op_codes(0b10_111_000, 0);

        for (op_code, register) in register_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Cmp(register));
        }
    }

    #[test]
    fn disassembler_handles_cmp_mem() {
        let operation = disassemble_op_code(0b10_111_110);
        assert_operation_equals_expected(&operation, &Operation::CmpMem);
    }

    #[test]
    fn disassembler_handles_cpi() {
        let operation = disassemble_op_code(0b11_111_110);
        assert_operation_equals_expected(&operation, &Operation::Cpi);
    }

    #[test]
    fn disassembler_handles_rlc() {
        let operation = disassemble_op_code(0b00_000_111);
        assert_operation_equals_expected(&operation, &Operation::Rlc);
    }

    #[test]
    fn disassembler_handles_rrc() {
        let operation = disassemble_op_code(0b00_001_111);
        assert_operation_equals_expected(&operation, &Operation::Rrc);
    }

    #[test]
    fn disassembler_handles_ral() {
        let operation = disassemble_op_code(0b00_010_111);
        assert_operation_equals_expected(&operation, &Operation::Ral);
    }

    #[test]
    fn disassembler_handles_rar() {
        let operation = disassemble_op_code(0b00_011_111);
        assert_operation_equals_expected(&operation, &Operation::Rar);
    }

    #[test]
    fn disassembler_handles_cma() {
        let operation = disassemble_op_code(0b00_101_111);
        assert_operation_equals_expected(&operation, &Operation::Cma);
    }

    #[test]
    fn disassembler_handles_cmc() {
        let operation = disassemble_op_code(0b00_111_111);
        assert_operation_equals_expected(&operation, &Operation::Cmc);
    }

    #[test]
    fn disassembler_handles_stc() {
        let operation = disassemble_op_code(0b00_110_111);
        assert_operation_equals_expected(&operation, &Operation::Stc);
    }

    #[test]
    fn disassembler_handles_jmp() {
        let operation = disassemble_op_code(0b11_000_011);
        assert_operation_equals_expected(&operation, &Operation::Jmp);
    }

    #[test]
    fn disassembler_handles_jcond() {
        let condition_map = all_conditions_for_op_codes(0b11_000_010, 3);

        for (op_code, condition) in condition_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Jcond(condition));
        }
    }

    #[test]
    fn disassembler_handles_call() {
        let operation = disassemble_op_code(0b11_001_101);
        assert_operation_equals_expected(&operation, &Operation::Call);
    }

    #[test]
    fn disassembler_handles_ccond() {
        let condition_map = all_conditions_for_op_codes(0b11_000_100, 3);

        for (op_code, condition) in condition_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Ccond(condition));
        }
    }

    #[test]
    fn disassembler_handles_ret() {
        let operation = disassemble_op_code(0b11_001_001);
        assert_operation_equals_expected(&operation, &Operation::Ret);
    }

    #[test]
    fn disassembler_handles_rcond() {
        let condition_map = all_conditions_for_op_codes(0b11_000_000, 3);

        for (op_code, condition) in condition_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Rcond(condition));
        }
    }

    #[test]
    fn disassembler_handles_rst() {
        let reset_index_map = all_combinations_for_op_codes(
            0b11_000_111,
            3,
            vec![0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111],
            |bit_pattern| bit_pattern,
        );

        for (op_code, reset_index) in reset_index_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Rst(reset_index));
        }
    }

    #[test]
    fn disassembler_handles_pchl() {
        let operation = disassemble_op_code(0b11_101_001);
        assert_operation_equals_expected(&operation, &Operation::Pchl);
    }

    #[test]
    fn disassembler_handles_push() {
        let register_pair_map =
            all_register_pairs_for_op_codes_with_exclusions(0b11_000_101, 4, vec![0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Push(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_push_psw() {
        let operation = disassemble_op_code(0b11_110_101);
        assert_operation_equals_expected(&operation, &Operation::PushPsw);
    }

    #[test]
    fn disassembler_handles_pop() {
        let register_pair_map =
            all_register_pairs_for_op_codes_with_exclusions(0b11_000_001, 4, vec![0b11]);

        for (op_code, register_pair) in register_pair_map {
            let operation = disassemble_op_code(op_code);
            assert_operation_equals_expected(&operation, &Operation::Pop(register_pair));
        }
    }

    #[test]
    fn disassembler_handles_pop_psw() {
        let operation = disassemble_op_code(0b11_110_001);
        assert_operation_equals_expected(&operation, &Operation::PopPsw);
    }

    #[test]
    fn disassembler_handles_xthl() {
        let operation = disassemble_op_code(0b11_100_011);
        assert_operation_equals_expected(&operation, &Operation::Xthl);
    }

    #[test]
    fn disassembler_handles_sphl() {
        let operation = disassemble_op_code(0b11_111_001);
        assert_operation_equals_expected(&operation, &Operation::Sphl);
    }

    #[test]
    fn disassembler_handles_in() {
        let operation = disassemble_op_code(0b11_011_011);
        assert_operation_equals_expected(&operation, &Operation::In);
    }

    #[test]
    fn disassembler_handles_out() {
        let operation = disassemble_op_code(0b11_010_011);
        assert_operation_equals_expected(&operation, &Operation::Out);
    }

    #[test]
    fn disassembler_handles_ei() {
        let operation = disassemble_op_code(0b11_111_011);
        assert_operation_equals_expected(&operation, &Operation::Ei);
    }

    #[test]
    fn disassembler_handles_di() {
        let operation = disassemble_op_code(0b11_110_011);
        assert_operation_equals_expected(&operation, &Operation::Di);
    }

    #[test]
    fn disassembler_handles_hlt() {
        let operation = disassemble_op_code(0b01_110_110);
        assert_operation_equals_expected(&operation, &Operation::Hlt);
    }

    #[test]
    fn disassembler_handles_nop() {
        let operation = disassemble_op_code(0b00_000_000);
        assert_operation_equals_expected(&operation, &Operation::Nop);
    }
}
