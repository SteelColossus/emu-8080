use emu_8080::{
    Condition, Instruction, MemoryLocation, MemoryRegisterPair, Operation, Port, ProgramAddress, Register,
    RegisterPair, RestartNumber,
};

fn jcond<'a>(condition: Condition, low: u8, high: u8) -> Instruction<'a> {
    let operation = match condition {
        Condition::NZ => Operation::Jnz,
        Condition::Z => Operation::Jz,
        Condition::NC => Operation::Jnc,
        Condition::C => Operation::Jc,
        Condition::PO => Operation::Jpo,
        Condition::PE => Operation::Jpe,
        Condition::P => Operation::Jp,
        Condition::M => Operation::Jm,
    };

    Instruction::new_from(operation, Box::new(ProgramAddress::new(low, high)))
}

fn ccond<'a>(condition: Condition, low: u8, high: u8) -> Instruction<'a> {
    let operation = match condition {
        Condition::NZ => Operation::Cnz,
        Condition::Z => Operation::Cz,
        Condition::NC => Operation::Cnc,
        Condition::C => Operation::Cc,
        Condition::PO => Operation::Cpo,
        Condition::PE => Operation::Cpe,
        Condition::P => Operation::Cp,
        Condition::M => Operation::Cm,
    };

    Instruction::new_from(operation, Box::new(ProgramAddress::new(low, high)))
}

fn rcond<'a>(condition: Condition) -> Instruction<'a> {
    let operation = match condition {
        Condition::NZ => Operation::Rnz,
        Condition::Z => Operation::Rz,
        Condition::NC => Operation::Rnc,
        Condition::C => Operation::Rc,
        Condition::PO => Operation::Rpo,
        Condition::PE => Operation::Rpe,
        Condition::P => Operation::Rp,
        Condition::M => Operation::Rm,
    };

    Instruction::new_no_args(operation)
}

fn next_arg<'a>(iter: &mut std::iter::Enumerate<std::slice::Iter<'a, u8>>) -> &'a u8 {
    iter.next().expect("Expected an argument byte but there was none").1
}

pub fn disassemble(hex_bytes: &[u8]) {
    // Minimum number of bytes
    let mut instructions: Vec<Instruction> = Vec::with_capacity(hex_bytes.len() / 3);
    let mut iter = hex_bytes.iter().enumerate();

    while let Some((i, op_code)) = iter.next() {
        let instruction = match op_code {
            0b01_111_111 => Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(Register::A)),
            0b01_000_111 => Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(Register::B)),
            0b01_001_111 => Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(Register::C)),
            0b01_010_111 => Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(Register::D)),
            0b01_011_111 => Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(Register::E)),
            0b01_100_111 => Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(Register::H)),
            0b01_101_111 => Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(Register::L)),
            0b01_111_000 => Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(Register::A)),
            0b01_000_000 => Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(Register::B)),
            0b01_001_000 => Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(Register::C)),
            0b01_010_000 => Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(Register::D)),
            0b01_011_000 => Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(Register::E)),
            0b01_100_000 => Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(Register::H)),
            0b01_101_000 => Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(Register::L)),
            0b01_111_001 => Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(Register::A)),
            0b01_000_001 => Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(Register::B)),
            0b01_001_001 => Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(Register::C)),
            0b01_010_001 => Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(Register::D)),
            0b01_011_001 => Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(Register::E)),
            0b01_100_001 => Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(Register::H)),
            0b01_101_001 => Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(Register::L)),
            0b01_111_010 => Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(Register::A)),
            0b01_000_010 => Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(Register::B)),
            0b01_001_010 => Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(Register::C)),
            0b01_010_010 => Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(Register::D)),
            0b01_011_010 => Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(Register::E)),
            0b01_100_010 => Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(Register::H)),
            0b01_101_010 => Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(Register::L)),
            0b01_111_011 => Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(Register::A)),
            0b01_000_011 => Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(Register::B)),
            0b01_001_011 => Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(Register::C)),
            0b01_010_011 => Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(Register::D)),
            0b01_011_011 => Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(Register::E)),
            0b01_100_011 => Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(Register::H)),
            0b01_101_011 => Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(Register::L)),
            0b01_111_100 => Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(Register::A)),
            0b01_000_100 => Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(Register::B)),
            0b01_001_100 => Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(Register::C)),
            0b01_010_100 => Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(Register::D)),
            0b01_011_100 => Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(Register::E)),
            0b01_100_100 => Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(Register::H)),
            0b01_101_100 => Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(Register::L)),
            0b01_111_101 => Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(Register::A)),
            0b01_000_101 => Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(Register::B)),
            0b01_001_101 => Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(Register::C)),
            0b01_010_101 => Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(Register::D)),
            0b01_011_101 => Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(Register::E)),
            0b01_100_101 => Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(Register::H)),
            0b01_101_101 => Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(Register::L)),
            0b01_111_110 => {
                Instruction::new_to_from(Operation::Mov, Box::new(Register::A), Box::new(MemoryRegisterPair::M))
            }
            0b01_000_110 => {
                Instruction::new_to_from(Operation::Mov, Box::new(Register::B), Box::new(MemoryRegisterPair::M))
            }
            0b01_001_110 => {
                Instruction::new_to_from(Operation::Mov, Box::new(Register::C), Box::new(MemoryRegisterPair::M))
            }
            0b01_010_110 => {
                Instruction::new_to_from(Operation::Mov, Box::new(Register::D), Box::new(MemoryRegisterPair::M))
            }
            0b01_011_110 => {
                Instruction::new_to_from(Operation::Mov, Box::new(Register::E), Box::new(MemoryRegisterPair::M))
            }
            0b01_100_110 => {
                Instruction::new_to_from(Operation::Mov, Box::new(Register::H), Box::new(MemoryRegisterPair::M))
            }
            0b01_101_110 => {
                Instruction::new_to_from(Operation::Mov, Box::new(Register::L), Box::new(MemoryRegisterPair::M))
            }
            0b01_110_111 => {
                Instruction::new_to_from(Operation::Mov, Box::new(MemoryRegisterPair::M), Box::new(Register::A))
            }
            0b01_110_000 => {
                Instruction::new_to_from(Operation::Mov, Box::new(MemoryRegisterPair::M), Box::new(Register::B))
            }
            0b01_110_001 => {
                Instruction::new_to_from(Operation::Mov, Box::new(MemoryRegisterPair::M), Box::new(Register::C))
            }
            0b01_110_010 => {
                Instruction::new_to_from(Operation::Mov, Box::new(MemoryRegisterPair::M), Box::new(Register::D))
            }
            0b01_110_011 => {
                Instruction::new_to_from(Operation::Mov, Box::new(MemoryRegisterPair::M), Box::new(Register::E))
            }
            0b01_110_100 => {
                Instruction::new_to_from(Operation::Mov, Box::new(MemoryRegisterPair::M), Box::new(Register::H))
            }
            0b01_110_101 => {
                Instruction::new_to_from(Operation::Mov, Box::new(MemoryRegisterPair::M), Box::new(Register::L))
            }
            0b00_111_110 => Instruction::new_to_content(Operation::Mvi, Box::new(Register::A), *next_arg(&mut iter)),
            0b00_000_110 => Instruction::new_to_content(Operation::Mvi, Box::new(Register::B), *next_arg(&mut iter)),
            0b00_001_110 => Instruction::new_to_content(Operation::Mvi, Box::new(Register::C), *next_arg(&mut iter)),
            0b00_010_110 => Instruction::new_to_content(Operation::Mvi, Box::new(Register::D), *next_arg(&mut iter)),
            0b00_011_110 => Instruction::new_to_content(Operation::Mvi, Box::new(Register::E), *next_arg(&mut iter)),
            0b00_100_110 => Instruction::new_to_content(Operation::Mvi, Box::new(Register::H), *next_arg(&mut iter)),
            0b00_101_110 => Instruction::new_to_content(Operation::Mvi, Box::new(Register::L), *next_arg(&mut iter)),
            0b00_110_110 => {
                Instruction::new_to_content(Operation::Mvi, Box::new(MemoryRegisterPair::M), *next_arg(&mut iter))
            }
            0b00_000_001 => Instruction::new_to_content16(
                Operation::Lxi,
                Box::new(RegisterPair::BC),
                *next_arg(&mut iter),
                *next_arg(&mut iter),
            ),
            0b00_010_001 => Instruction::new_to_content16(
                Operation::Lxi,
                Box::new(RegisterPair::DE),
                *next_arg(&mut iter),
                *next_arg(&mut iter),
            ),
            0b00_100_001 => Instruction::new_to_content16(
                Operation::Lxi,
                Box::new(RegisterPair::HL),
                *next_arg(&mut iter),
                *next_arg(&mut iter),
            ),
            0b00_110_001 => Instruction::new_to_content16(
                Operation::Lxi,
                Box::new(RegisterPair::SP),
                *next_arg(&mut iter),
                *next_arg(&mut iter),
            ),
            0b00_111_010 => Instruction::new_from(
                Operation::Lda,
                Box::new(MemoryLocation::new(*next_arg(&mut iter), *next_arg(&mut iter))),
            ),
            0b00_110_010 => Instruction::new_to(
                Operation::Sta,
                Box::new(MemoryLocation::new(*next_arg(&mut iter), *next_arg(&mut iter))),
            ),
            0b00_101_010 => Instruction::new_from(
                Operation::Lhld,
                Box::new(MemoryLocation::new(*next_arg(&mut iter), *next_arg(&mut iter))),
            ),
            0b00_100_010 => Instruction::new_to(
                Operation::Shld,
                Box::new(MemoryLocation::new(*next_arg(&mut iter), *next_arg(&mut iter))),
            ),
            0b00_001_010 => Instruction::new_from(Operation::Ldax, Box::new(RegisterPair::BC)),
            0b00_011_010 => Instruction::new_from(Operation::Ldax, Box::new(RegisterPair::DE)),
            0b00_000_010 => Instruction::new_to(Operation::Stax, Box::new(RegisterPair::BC)),
            0b00_010_010 => Instruction::new_to(Operation::Stax, Box::new(RegisterPair::DE)),
            0b11_101_011 => Instruction::new_no_args(Operation::Xchg),
            0b10_000_111 => Instruction::new_from(Operation::Add, Box::new(Register::A)),
            0b10_000_000 => Instruction::new_from(Operation::Add, Box::new(Register::B)),
            0b10_000_001 => Instruction::new_from(Operation::Add, Box::new(Register::C)),
            0b10_000_010 => Instruction::new_from(Operation::Add, Box::new(Register::D)),
            0b10_000_011 => Instruction::new_from(Operation::Add, Box::new(Register::E)),
            0b10_000_100 => Instruction::new_from(Operation::Add, Box::new(Register::H)),
            0b10_000_101 => Instruction::new_from(Operation::Add, Box::new(Register::L)),
            0b10_000_110 => Instruction::new_from(Operation::Add, Box::new(MemoryRegisterPair::M)),
            0b11_000_110 => Instruction::new_content(Operation::Adi, *next_arg(&mut iter)),
            0b10_001_111 => Instruction::new_from(Operation::Adc, Box::new(Register::A)),
            0b10_001_000 => Instruction::new_from(Operation::Adc, Box::new(Register::B)),
            0b10_001_001 => Instruction::new_from(Operation::Adc, Box::new(Register::C)),
            0b10_001_010 => Instruction::new_from(Operation::Adc, Box::new(Register::D)),
            0b10_001_011 => Instruction::new_from(Operation::Adc, Box::new(Register::E)),
            0b10_001_100 => Instruction::new_from(Operation::Adc, Box::new(Register::H)),
            0b10_001_101 => Instruction::new_from(Operation::Adc, Box::new(Register::L)),
            0b10_001_110 => Instruction::new_from(Operation::Adc, Box::new(MemoryRegisterPair::M)),
            0b11_001_110 => Instruction::new_content(Operation::Aci, *next_arg(&mut iter)),
            0b10_010_111 => Instruction::new_from(Operation::Sub, Box::new(Register::A)),
            0b10_010_000 => Instruction::new_from(Operation::Sub, Box::new(Register::B)),
            0b10_010_001 => Instruction::new_from(Operation::Sub, Box::new(Register::C)),
            0b10_010_010 => Instruction::new_from(Operation::Sub, Box::new(Register::D)),
            0b10_010_011 => Instruction::new_from(Operation::Sub, Box::new(Register::E)),
            0b10_010_100 => Instruction::new_from(Operation::Sub, Box::new(Register::H)),
            0b10_010_101 => Instruction::new_from(Operation::Sub, Box::new(Register::L)),
            0b10_010_110 => Instruction::new_from(Operation::Sub, Box::new(MemoryRegisterPair::M)),
            0b11_010_110 => Instruction::new_content(Operation::Sui, *next_arg(&mut iter)),
            0b10_011_111 => Instruction::new_from(Operation::Sbb, Box::new(Register::A)),
            0b10_011_000 => Instruction::new_from(Operation::Sbb, Box::new(Register::B)),
            0b10_011_001 => Instruction::new_from(Operation::Sbb, Box::new(Register::C)),
            0b10_011_010 => Instruction::new_from(Operation::Sbb, Box::new(Register::D)),
            0b10_011_011 => Instruction::new_from(Operation::Sbb, Box::new(Register::E)),
            0b10_011_100 => Instruction::new_from(Operation::Sbb, Box::new(Register::H)),
            0b10_011_101 => Instruction::new_from(Operation::Sbb, Box::new(Register::L)),
            0b10_011_110 => Instruction::new_from(Operation::Sbb, Box::new(MemoryRegisterPair::M)),
            0b11_011_110 => Instruction::new_content(Operation::Sbi, *next_arg(&mut iter)),
            0b00_111_100 => Instruction::new_to(Operation::Inr, Box::new(Register::A)),
            0b00_000_100 => Instruction::new_to(Operation::Inr, Box::new(Register::B)),
            0b00_001_100 => Instruction::new_to(Operation::Inr, Box::new(Register::C)),
            0b00_010_100 => Instruction::new_to(Operation::Inr, Box::new(Register::D)),
            0b00_011_100 => Instruction::new_to(Operation::Inr, Box::new(Register::E)),
            0b00_100_100 => Instruction::new_to(Operation::Inr, Box::new(Register::H)),
            0b00_101_100 => Instruction::new_to(Operation::Inr, Box::new(Register::L)),
            0b00_110_100 => Instruction::new_to(Operation::Inr, Box::new(MemoryRegisterPair::M)),
            0b00_111_101 => Instruction::new_to(Operation::Dcr, Box::new(Register::A)),
            0b00_000_101 => Instruction::new_to(Operation::Dcr, Box::new(Register::B)),
            0b00_001_101 => Instruction::new_to(Operation::Dcr, Box::new(Register::C)),
            0b00_010_101 => Instruction::new_to(Operation::Dcr, Box::new(Register::D)),
            0b00_011_101 => Instruction::new_to(Operation::Dcr, Box::new(Register::E)),
            0b00_100_101 => Instruction::new_to(Operation::Dcr, Box::new(Register::H)),
            0b00_101_101 => Instruction::new_to(Operation::Dcr, Box::new(Register::L)),
            0b00_110_101 => Instruction::new_to(Operation::Dcr, Box::new(MemoryRegisterPair::M)),
            0b00_000_011 => Instruction::new_to(Operation::Inx, Box::new(RegisterPair::BC)),
            0b00_010_011 => Instruction::new_to(Operation::Inx, Box::new(RegisterPair::DE)),
            0b00_100_011 => Instruction::new_to(Operation::Inx, Box::new(RegisterPair::HL)),
            0b00_110_011 => Instruction::new_to(Operation::Inx, Box::new(RegisterPair::SP)),
            0b00_001_011 => Instruction::new_to(Operation::Dcx, Box::new(RegisterPair::BC)),
            0b00_011_011 => Instruction::new_to(Operation::Dcx, Box::new(RegisterPair::DE)),
            0b00_101_011 => Instruction::new_to(Operation::Dcx, Box::new(RegisterPair::HL)),
            0b00_111_011 => Instruction::new_to(Operation::Dcx, Box::new(RegisterPair::SP)),
            0b00_001_001 => Instruction::new_from(Operation::Dad, Box::new(RegisterPair::BC)),
            0b00_011_001 => Instruction::new_from(Operation::Dad, Box::new(RegisterPair::DE)),
            0b00_101_001 => Instruction::new_from(Operation::Dad, Box::new(RegisterPair::HL)),
            0b00_111_001 => Instruction::new_from(Operation::Dad, Box::new(RegisterPair::SP)),
            0b00_100_111 => Instruction::new_no_args(Operation::Daa),
            0b10_100_111 => Instruction::new_from(Operation::Ana, Box::new(Register::A)),
            0b10_100_000 => Instruction::new_from(Operation::Ana, Box::new(Register::B)),
            0b10_100_001 => Instruction::new_from(Operation::Ana, Box::new(Register::C)),
            0b10_100_010 => Instruction::new_from(Operation::Ana, Box::new(Register::D)),
            0b10_100_011 => Instruction::new_from(Operation::Ana, Box::new(Register::E)),
            0b10_100_100 => Instruction::new_from(Operation::Ana, Box::new(Register::H)),
            0b10_100_101 => Instruction::new_from(Operation::Ana, Box::new(Register::L)),
            0b10_100_110 => Instruction::new_from(Operation::Ana, Box::new(MemoryRegisterPair::M)),
            0b11_100_110 => Instruction::new_content(Operation::Ani, *next_arg(&mut iter)),
            0b10_101_111 => Instruction::new_from(Operation::Xra, Box::new(Register::A)),
            0b10_101_000 => Instruction::new_from(Operation::Xra, Box::new(Register::B)),
            0b10_101_001 => Instruction::new_from(Operation::Xra, Box::new(Register::C)),
            0b10_101_010 => Instruction::new_from(Operation::Xra, Box::new(Register::D)),
            0b10_101_011 => Instruction::new_from(Operation::Xra, Box::new(Register::E)),
            0b10_101_100 => Instruction::new_from(Operation::Xra, Box::new(Register::H)),
            0b10_101_101 => Instruction::new_from(Operation::Xra, Box::new(Register::L)),
            0b10_101_110 => Instruction::new_from(Operation::Xra, Box::new(MemoryRegisterPair::M)),
            0b11_101_110 => Instruction::new_content(Operation::Xri, *next_arg(&mut iter)),
            0b10_110_111 => Instruction::new_from(Operation::Ora, Box::new(Register::A)),
            0b10_110_000 => Instruction::new_from(Operation::Ora, Box::new(Register::B)),
            0b10_110_001 => Instruction::new_from(Operation::Ora, Box::new(Register::C)),
            0b10_110_010 => Instruction::new_from(Operation::Ora, Box::new(Register::D)),
            0b10_110_011 => Instruction::new_from(Operation::Ora, Box::new(Register::E)),
            0b10_110_100 => Instruction::new_from(Operation::Ora, Box::new(Register::H)),
            0b10_110_101 => Instruction::new_from(Operation::Ora, Box::new(Register::L)),
            0b10_110_110 => Instruction::new_from(Operation::Ora, Box::new(MemoryRegisterPair::M)),
            0b11_110_110 => Instruction::new_content(Operation::Ori, *next_arg(&mut iter)),
            0b10_111_111 => Instruction::new_from(Operation::Cmp, Box::new(Register::A)),
            0b10_111_000 => Instruction::new_from(Operation::Cmp, Box::new(Register::B)),
            0b10_111_001 => Instruction::new_from(Operation::Cmp, Box::new(Register::C)),
            0b10_111_010 => Instruction::new_from(Operation::Cmp, Box::new(Register::D)),
            0b10_111_011 => Instruction::new_from(Operation::Cmp, Box::new(Register::E)),
            0b10_111_100 => Instruction::new_from(Operation::Cmp, Box::new(Register::H)),
            0b10_111_101 => Instruction::new_from(Operation::Cmp, Box::new(Register::L)),
            0b10_111_110 => Instruction::new_from(Operation::Cmp, Box::new(MemoryRegisterPair::M)),
            0b11_111_110 => Instruction::new_content(Operation::Cpi, *next_arg(&mut iter)),
            0b00_000_111 => Instruction::new_no_args(Operation::Rlc),
            0b00_001_111 => Instruction::new_no_args(Operation::Rrc),
            0b00_010_111 => Instruction::new_no_args(Operation::Ral),
            0b00_011_111 => Instruction::new_no_args(Operation::Rar),
            0b00_101_111 => Instruction::new_no_args(Operation::Cma),
            0b00_111_111 => Instruction::new_no_args(Operation::Cmc),
            0b00_110_111 => Instruction::new_no_args(Operation::Stc),
            0b11_000_011 => Instruction::new_from(
                Operation::Jmp,
                Box::new(ProgramAddress::new(*next_arg(&mut iter), *next_arg(&mut iter))),
            ),
            0b11_000_010 => jcond(Condition::NZ, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_001_010 => jcond(Condition::Z, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_010_010 => jcond(Condition::NC, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_011_010 => jcond(Condition::C, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_100_010 => jcond(Condition::PO, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_101_010 => jcond(Condition::PE, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_110_010 => jcond(Condition::P, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_111_010 => jcond(Condition::M, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_001_101 => Instruction::new_from(
                Operation::Call,
                Box::new(ProgramAddress::new(*next_arg(&mut iter), *next_arg(&mut iter))),
            ),
            0b11_000_100 => ccond(Condition::NZ, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_001_100 => ccond(Condition::Z, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_010_100 => ccond(Condition::NC, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_011_100 => ccond(Condition::C, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_100_100 => ccond(Condition::PO, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_101_100 => ccond(Condition::PE, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_110_100 => ccond(Condition::P, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_111_100 => ccond(Condition::M, *next_arg(&mut iter), *next_arg(&mut iter)),
            0b11_001_001 => Instruction::new_no_args(Operation::Ret),
            0b11_000_000 => rcond(Condition::NZ),
            0b11_001_000 => rcond(Condition::Z),
            0b11_010_000 => rcond(Condition::NC),
            0b11_011_000 => rcond(Condition::C),
            0b11_100_000 => rcond(Condition::PO),
            0b11_101_000 => rcond(Condition::PE),
            0b11_110_000 => rcond(Condition::P),
            0b11_111_000 => rcond(Condition::M),
            0b11_000_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(0))),
            0b11_001_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(1))),
            0b11_010_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(2))),
            0b11_011_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(3))),
            0b11_100_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(4))),
            0b11_101_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(5))),
            0b11_110_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(6))),
            0b11_111_111 => Instruction::new_from(Operation::Rst, Box::new(RestartNumber::new(7))),
            0b11_101_001 => Instruction::new_no_args(Operation::Pchl),
            0b11_000_101 => Instruction::new_from(Operation::Push, Box::new(RegisterPair::BC)),
            0b11_010_101 => Instruction::new_from(Operation::Push, Box::new(RegisterPair::DE)),
            0b11_100_101 => Instruction::new_from(Operation::Push, Box::new(RegisterPair::HL)),
            0b11_110_101 => Instruction::new_no_args(Operation::PushPsw),
            0b11_000_001 => Instruction::new_to(Operation::Pop, Box::new(RegisterPair::BC)),
            0b11_010_001 => Instruction::new_to(Operation::Pop, Box::new(RegisterPair::DE)),
            0b11_100_001 => Instruction::new_to(Operation::Pop, Box::new(RegisterPair::HL)),
            0b11_110_001 => Instruction::new_no_args(Operation::PopPsw),
            0b11_100_011 => Instruction::new_no_args(Operation::Xthl),
            0b11_111_001 => Instruction::new_no_args(Operation::Sphl),
            0b11_011_011 => Instruction::new_from(Operation::In, Box::new(Port::new(*next_arg(&mut iter)))),
            0b11_010_011 => Instruction::new_to(Operation::Out, Box::new(Port::new(*next_arg(&mut iter)))),
            0b11_111_011 => Instruction::new_no_args(Operation::Ei),
            0b11_110_011 => Instruction::new_no_args(Operation::Di),
            0b01_110_110 => Instruction::new_no_args(Operation::Hlt),
            0b00_000_000 => Instruction::new_no_args(Operation::Nop),
            _ => panic!("Unrecognized opcode!"),
        };

        println!("{:04X}: {:02X}    {}", i, op_code, instruction);

        instructions.push(instruction);
    }
}
