use std::fmt;

fn concat_as_bytes(low: u8, high: u8) -> u16 {
    (u16::from(high) << 8) ^ u16::from(low)
}

pub enum Condition {
    NZ,
    Z,
    NC,
    C,
    PO,
    PE,
    P,
    M,
}

pub enum Operation {
    Mov,
    Mvi,
    Lxi,
    Lda,
    Sta,
    Lhld,
    Shld,
    Ldax,
    Stax,
    Xchg,
    Add,
    Adi,
    Adc,
    Aci,
    Sub,
    Sui,
    Sbb,
    Sbi,
    Inr,
    Dcr,
    Inx,
    Dcx,
    Dad,
    Daa,
    Ana,
    Ani,
    Xra,
    Xri,
    Ora,
    Ori,
    Cmp,
    Cpi,
    Rlc,
    Rrc,
    Ral,
    Rar,
    Cma,
    Cmc,
    Stc,
    Jmp,
    Jnz,
    Jz,
    Jnc,
    Jc,
    Jpo,
    Jpe,
    Jp,
    Jm,
    Call,
    Cnz,
    Cz,
    Cnc,
    Cc,
    Cpo,
    Cpe,
    Cp,
    Cm,
    Ret,
    Rnz,
    Rz,
    Rnc,
    Rc,
    Rpo,
    Rpe,
    Rp,
    Rm,
    Rst,
    Pchl,
    Push,
    PushPsw,
    Pop,
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
    fn name(&self) -> &str {
        match self {
            Operation::Mov => "MOV",
            Operation::Mvi => "MVI",
            Operation::Lxi => "LXI",
            Operation::Lda => "LDA",
            Operation::Sta => "STA",
            Operation::Lhld => "LHLD",
            Operation::Shld => "SHLD",
            Operation::Ldax => "LDAX",
            Operation::Stax => "STAX",
            Operation::Xchg => "XCHG",
            Operation::Add => "ADD",
            Operation::Adi => "ADI",
            Operation::Adc => "ADC",
            Operation::Aci => "ACI",
            Operation::Sub => "SUB",
            Operation::Sui => "SUI",
            Operation::Sbb => "SBB",
            Operation::Sbi => "SBI",
            Operation::Inr => "INR",
            Operation::Dcr => "DCR",
            Operation::Inx => "INX",
            Operation::Dcx => "DCX",
            Operation::Dad => "DAD",
            Operation::Daa => "DAA",
            Operation::Ana => "ANA",
            Operation::Ani => "ANI",
            Operation::Xra => "XRA",
            Operation::Xri => "XRI",
            Operation::Ora => "ORA",
            Operation::Ori => "ORI",
            Operation::Cmp => "CMP",
            Operation::Cpi => "CPI",
            Operation::Rlc => "RLC",
            Operation::Rrc => "RRC",
            Operation::Ral => "RAL",
            Operation::Rar => "RAR",
            Operation::Cma => "CMA",
            Operation::Cmc => "CMC",
            Operation::Stc => "STC",
            Operation::Jmp => "JMP",
            Operation::Jnz => "JNZ",
            Operation::Jz => "JZ",
            Operation::Jnc => "JNC",
            Operation::Jc => "JC",
            Operation::Jpo => "JPO",
            Operation::Jpe => "JPE",
            Operation::Jp => "JP",
            Operation::Jm => "JM",
            Operation::Call => "CALL",
            Operation::Cnz => "CNZ",
            Operation::Cz => "CZ",
            Operation::Cnc => "CNC",
            Operation::Cc => "CC",
            Operation::Cpo => "CPO",
            Operation::Cpe => "CPE",
            Operation::Cp => "CP",
            Operation::Cm => "CM",
            Operation::Ret => "RET",
            Operation::Rnz => "RNZ",
            Operation::Rz => "RZ",
            Operation::Rnc => "RNC",
            Operation::Rc => "RC",
            Operation::Rpo => "RPO",
            Operation::Rpe => "RPE",
            Operation::Rp => "RP",
            Operation::Rm => "RM",
            Operation::Rst => "RST",
            Operation::Pchl => "PCHL",
            Operation::Push => "PUSH",
            Operation::PushPsw => "PUSH PSW",
            Operation::Pop => "POP",
            Operation::PopPsw => "POP PSW",
            Operation::Xthl => "XTHL",
            Operation::Sphl => "SPHL",
            Operation::In => "IN",
            Operation::Out => "OUT",
            Operation::Ei => "EI",
            Operation::Di => "DI",
            Operation::Hlt => "HLT",
            Operation::Nop => "NOP",
        }
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub trait Location: fmt::Debug + fmt::Display {}

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Register {
    fn name(&self) -> &str {
        match self {
            Register::A => "A",
            Register::B => "B",
            Register::C => "C",
            Register::D => "D",
            Register::E => "E",
            Register::H => "H",
            Register::L => "L",
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Location for Register {}

#[derive(Debug)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

impl RegisterPair {
    fn pair(&self) -> (Option<Register>, Option<Register>) {
        match self {
            RegisterPair::BC => (Some(Register::B), Some(Register::C)),
            RegisterPair::DE => (Some(Register::D), Some(Register::E)),
            RegisterPair::HL => (Some(Register::H), Some(Register::L)),
            RegisterPair::SP => (None, None),
        }
    }

    fn name(&self) -> &str {
        match self {
            RegisterPair::BC => "BC",
            RegisterPair::DE => "DE",
            RegisterPair::HL => "HL",
            RegisterPair::SP => "SP",
        }
    }
}

impl fmt::Display for RegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Location for RegisterPair {}

#[derive(Debug)]
pub enum MemoryRegisterPair {
    M,
}

impl MemoryRegisterPair {
    fn register_pair(&self) -> RegisterPair {
        match self {
            MemoryRegisterPair::M => RegisterPair::HL,
        }
    }

    fn name(&self) -> &str {
        match self {
            MemoryRegisterPair::M => "M",
        }
    }
}

impl fmt::Display for MemoryRegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Location for MemoryRegisterPair {}

#[derive(Debug)]
pub struct MemoryLocation {
    address: u16,
}

impl MemoryLocation {
    pub fn new(low: u8, high: u8) -> Self {
        let address = concat_as_bytes(low, high);
        MemoryLocation { address }
    }
}

impl fmt::Display for MemoryLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${:04X}", self.address)
    }
}

impl Location for MemoryLocation {}

#[derive(Debug)]
pub struct ProgramAddress {
    address: u16,
}

impl ProgramAddress {
    pub fn new(low: u8, high: u8) -> Self {
        let address = concat_as_bytes(low, high);
        ProgramAddress { address }
    }
}

impl fmt::Display for ProgramAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:04X}", self.address)
    }
}

impl Location for ProgramAddress {}

#[derive(Debug)]
pub struct Port {
    port: u8,
}

impl Port {
    pub fn new(port: u8) -> Self {
        Port { port }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.port)
    }
}

impl Location for Port {}

#[derive(Debug)]
pub struct RestartNumber {
    restart_number: u8,
}

impl RestartNumber {
    pub fn new(restart_number: u8) -> Self {
        RestartNumber { restart_number }
    }
}

impl fmt::Display for RestartNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.restart_number)
    }
}

impl Location for RestartNumber {}

#[derive(Debug)]
pub struct Instruction<'a> {
    pub operation: Operation,
    to: Option<Box<dyn Location + 'a>>,
    from: Option<Box<dyn Location + 'a>>,
    content: Option<u8>,
    content_high: Option<u8>,
}

impl<'a> Instruction<'a> {
    fn new(
        operation: Operation,
        to: Option<Box<dyn Location + 'a>>,
        from: Option<Box<dyn Location + 'a>>,
        content: Option<u8>,
        content_high: Option<u8>,
    ) -> Self {
        Instruction {
            operation,
            to,
            from,
            content,
            content_high,
        }
    }

    pub fn new_no_args(operation: Operation) -> Self {
        Instruction::new(operation, None, None, None, None)
    }

    // From: loading from the provided location
    // To: storing at the provided location
    // If the operation happens in place, it is to and not from

    pub fn new_to(operation: Operation, to: Box<dyn Location + 'a>) -> Self {
        Instruction::new(operation, Some(to), None, None, None)
    }

    pub fn new_from(operation: Operation, from: Box<dyn Location + 'a>) -> Self {
        Instruction::new(operation, None, Some(from), None, None)
    }

    pub fn new_content(operation: Operation, content: u8) -> Self {
        Instruction::new(operation, None, None, Some(content), None)
    }

    pub fn new_to_from(operation: Operation, to: Box<dyn Location + 'a>, from: Box<dyn Location + 'a>) -> Self {
        Instruction::new(operation, Some(to), Some(from), None, None)
    }

    pub fn new_to_content(operation: Operation, to: Box<dyn Location + 'a>, content: u8) -> Self {
        Instruction::new(operation, Some(to), None, Some(content), None)
    }

    pub fn new_to_content16(operation: Operation, to: Box<dyn Location + 'a>, content: u8, content_high: u8) -> Self {
        Instruction::new(operation, Some(to), None, Some(content), Some(content_high))
    }
}

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<6}", self.operation.name())?;

        if let Some(to) = &self.to {
            write!(f, "{}", to)?;

            if let Some(from) = &self.from {
                write!(f, ",{}", from)?;
            } else if let Some(content) = self.content {
                if let Some(content_high) = self.content_high {
                    write!(f, ",&{:04X}", concat_as_bytes(content, content_high))?;
                } else {
                    write!(f, ",&{:02X}", content)?;
                }
            }
        } else if let Some(from) = &self.from {
            write!(f, "{}", from)?;
        } else if let Some(content) = self.content {
            write!(f, "&{:02X}", content)?;
        }

        Ok(())
    }
}
