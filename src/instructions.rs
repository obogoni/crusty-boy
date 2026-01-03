use crate::registers::{Register, Register16};

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Copy(Register, Register),
    CopyToAddress(Register, Register16),
    CopyToLoadedAddress(Register),
    Load8(Register),
    LoadFromAddress(Register16, Register),
    LoadFromLoadedAddress(Register),
    LoadToAddress(Register16),
    Load16(Register16)
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {

            //LD r8, r8 instructions
            0x40 => Some(Instruction::Copy(Register::B, Register::B)),
            0x41 => Some(Instruction::Copy(Register::B, Register::C)),
            0x42 => Some(Instruction::Copy(Register::B, Register::D)),
            0x43 => Some(Instruction::Copy(Register::B, Register::E)),
            0x44 => Some(Instruction::Copy(Register::B, Register::H)),
            0x45 => Some(Instruction::Copy(Register::B, Register::L)),
            0x47 => Some(Instruction::Copy(Register::B, Register::A)),
            0x48 => Some(Instruction::Copy(Register::C, Register::B)),
            0x49 => Some(Instruction::Copy(Register::C, Register::C)),
            0x4A => Some(Instruction::Copy(Register::C, Register::D)),
            0x4B => Some(Instruction::Copy(Register::C, Register::E)),
            0x4C => Some(Instruction::Copy(Register::C, Register::H)),
            0x4D => Some(Instruction::Copy(Register::C, Register::L)),
            0x4F => Some(Instruction::Copy(Register::C, Register::A)),
            0x50 => Some(Instruction::Copy(Register::D, Register::B)),
            0x51 => Some(Instruction::Copy(Register::D, Register::C)),
            0x52 => Some(Instruction::Copy(Register::D, Register::D)),
            0x53 => Some(Instruction::Copy(Register::D, Register::E)),
            0x54 => Some(Instruction::Copy(Register::D, Register::H)),
            0x55 => Some(Instruction::Copy(Register::D, Register::L)),
            0x57 => Some(Instruction::Copy(Register::D, Register::A)),
            0x58 => Some(Instruction::Copy(Register::E, Register::B)),
            0x59 => Some(Instruction::Copy(Register::E, Register::C)),
            0x5A => Some(Instruction::Copy(Register::E, Register::D)),
            0x5B => Some(Instruction::Copy(Register::E, Register::E)),
            0x5C => Some(Instruction::Copy(Register::E, Register::H)),
            0x5D => Some(Instruction::Copy(Register::E, Register::L)),
            0x5F => Some(Instruction::Copy(Register::E, Register::A)),
            0x60 => Some(Instruction::Copy(Register::H, Register::B)),
            0x61 => Some(Instruction::Copy(Register::H, Register::C)),
            0x62 => Some(Instruction::Copy(Register::H, Register::D)),
            0x63 => Some(Instruction::Copy(Register::H, Register::E)),
            0x64 => Some(Instruction::Copy(Register::H, Register::H)),
            0x65 => Some(Instruction::Copy(Register::H, Register::L)),
            0x67 => Some(Instruction::Copy(Register::H, Register::A)),
            0x68 => Some(Instruction::Copy(Register::L, Register::B)),
            0x69 => Some(Instruction::Copy(Register::L, Register::C)),
            0x6A => Some(Instruction::Copy(Register::L, Register::D)),
            0x6B => Some(Instruction::Copy(Register::L, Register::E)),
            0x6C => Some(Instruction::Copy(Register::L, Register::H)),
            0x6D => Some(Instruction::Copy(Register::L, Register::L)),
            0x6F => Some(Instruction::Copy(Register::L, Register::A)),
            0x78 => Some(Instruction::Copy(Register::A, Register::B)),
            0x79 => Some(Instruction::Copy(Register::A, Register::C)),
            0x7A => Some(Instruction::Copy(Register::A, Register::D)),
            0x7B => Some(Instruction::Copy(Register::A, Register::E)),
            0x7C => Some(Instruction::Copy(Register::A, Register::H)),
            0x7D => Some(Instruction::Copy(Register::A, Register::L)),
            0x7F => Some(Instruction::Copy(Register::A, Register::A)),

            //LD r8, n8 instructions
            0x06 => Some(Instruction::Load8(Register::B)),
            0x0E => Some(Instruction::Load8(Register::C)),
            0x16 => Some(Instruction::Load8(Register::D)),
            0x1E => Some(Instruction::Load8(Register::E)),
            0x26 => Some(Instruction::Load8(Register::H)),
            0x2E => Some(Instruction::Load8(Register::L)),
            0x3E => Some(Instruction::Load8(Register::A)),

            //LD r16, n16 instructions
            0x01 => Some(Instruction::Load16(Register16::BC)),
            0x11 => Some(Instruction::Load16(Register16::DE)),
            0x21 => Some(Instruction::Load16(Register16::HL)),
            0x31 => Some(Instruction::Load16(Register16::SP)),

             //LD [HL], r8 instructions
            0x70 => Some(Instruction::CopyToAddress(Register::B, Register16::HL)),
            0x71 => Some(Instruction::CopyToAddress(Register::C, Register16::HL)),
            0x72 => Some(Instruction::CopyToAddress(Register::D, Register16::HL)),
            0x73 => Some(Instruction::CopyToAddress(Register::E, Register16::HL)),
            0x74 => Some(Instruction::CopyToAddress(Register::H, Register16::HL)),
            0x75 => Some(Instruction::CopyToAddress(Register::L, Register16::HL)),
            0x77 => Some(Instruction::CopyToAddress(Register::A, Register16::HL)),

            //LD [HL], n8 instructions
            0x36 => Some(Instruction::LoadToAddress(Register16::HL)),

            //LD r8, [HL] instructions
            0x46 => Some(Instruction::LoadFromAddress(Register16::HL, Register::B)),
            0x4E => Some(Instruction::LoadFromAddress(Register16::HL, Register::C)),
            0x56 => Some(Instruction::LoadFromAddress(Register16::HL, Register::D)),
            0x5E => Some(Instruction::LoadFromAddress(Register16::HL, Register::E)),
            0x66 => Some(Instruction::LoadFromAddress(Register16::HL, Register::H)),
            0x6E => Some(Instruction::LoadFromAddress(Register16::HL, Register::L)),
            0x7E => Some(Instruction::LoadFromAddress(Register16::HL, Register::A)),

            //LD [r16], A
            0x02 => Some(Instruction::CopyToAddress(Register::A, Register16::BC)),
            0x12 => Some(Instruction::CopyToAddress(Register::A, Register16::DE)),

            //LD [n16], A
            0xEA => Some(Instruction::CopyToLoadedAddress(Register::A)),

            //LD A, [n16]
            0xFA => Some(Instruction::LoadFromLoadedAddress(Register::A)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::Instruction;
    use crate::registers::Register;

    #[test]
    fn test_from_byte_load8() {
        let inst = Instruction::from_byte(0x06);

        assert!(inst.is_some());
        assert_eq!(inst.unwrap(), Instruction::Load8(Register::B));
    }

    #[test]
    fn test_from_byte_load8_halt_inst() {
        let inst = Instruction::from_byte(0x76);

        assert!(inst.is_none())
    }
}
