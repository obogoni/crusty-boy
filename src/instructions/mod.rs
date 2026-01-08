use crate::{cpu::Cpu, mmu::Mmu};

mod functions;
mod instruction_set;

pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub execute: fn(&mut Cpu, &mut Mmu) -> u8,
}

impl Instruction {
    pub const fn new(
        opcode: u8,
        mnemonic: &'static str,
        execute: fn(&mut Cpu, &mut Mmu) -> u8,
    ) -> Self {
        Instruction {
            opcode,
            mnemonic,
            execute,
        }
    }

    pub fn from_byte(byte: u8) -> Option<&'static Self> {
        for inst in instruction_set::INSTRUCTIONS.iter() {
            if inst.opcode == byte {
                return Some(inst);
            }
        }

        None
    }
}
