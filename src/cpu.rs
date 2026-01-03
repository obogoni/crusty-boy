use crate::{instructions::Instruction, memory::MemoryBus, registers::{Register, Register16, Registers}};
pub struct CPU {
    pub cycles: u16,
    pub registers: Registers,
}

impl CPU {

     pub fn new() -> Self {
        CPU {
            cycles: 0,
            registers: Registers::new(),
        }
    }

    pub fn step(&mut self, mem_bus: &mut MemoryBus) {

        let byte = mem_bus.fetch_byte(self.registers.pc);

        self.registers.pc += 1;

        let inst = Instruction::from_byte(byte);

        if inst.is_none() {
            log::error!("Unknown instruction: {:#X}", byte);
            return;
        }

        match inst {
            Some(instruction) => {
                match instruction {
                    Instruction::Copy(from, to) => {
                        self.copy(from, to);
                    },
                    Instruction::CopyToAddress(from, reg_addr) => {
                        self.copy_to_address(from, reg_addr, mem_bus);
                    }
                    Instruction::CopyToLoadedAddress(from) => {
                        self.copy_to_loaded_address(from, mem_bus);
                    },
                    Instruction::Load8(to) => {
                        self.load_byte(to, mem_bus);
                    },
                    Instruction::Load16(to) => {
                        self.load_word(to, mem_bus);
                    },
                    Instruction::LoadFromAddress(reg_addr, to) => {
                        self.load_from_address(reg_addr, to, mem_bus);
                    },
                    Instruction::LoadFromLoadedAddress(to) => {
                        self.load_from_loaded_address(to, mem_bus);
                    },
                    Instruction::LoadToAddress(reg_addr) => {
                        self.load_to_address(reg_addr, mem_bus);
                    },

                    _ => unreachable!(), // Other instructions not implemented yet
                }
            }
            None => {
                // Handle unknown instruction
            }
        }
    }

    fn copy(&mut self, from: Register, to: Register) -> u8 {
        self.registers.write_reg(to, self.registers.read_reg(from));

        4
    }

    fn copy_to_address(&mut self, from: Register, to: Register16, mem_bus: &mut MemoryBus) -> u8 {

        let addr = self.registers.read_reg16(to);
        let value = self.registers.read_reg(from);
        mem_bus.write_byte(addr, value);

        8
    }

    fn load_byte(&mut self, to: Register, mem_bus: &mut MemoryBus) -> u8 {
        let value = mem_bus.fetch_byte(self.registers.pc);
        self.registers.pc += 1;
        self.registers.write_reg(to, value);

        8
    }

    fn load_from_address(&mut self, reg_addr: Register16, to: Register, mem_bus: &mut MemoryBus) -> u8 {
        let addr = self.registers.read_reg16(reg_addr);
        let value = mem_bus.fetch_byte(addr);
        self.registers.write_reg(to, value);

        8
    }

    fn load_from_loaded_address(&mut self, to: Register, mem_bus: &mut MemoryBus) -> u8 {
        let addr = mem_bus.fetch_word(self.registers.pc);
        self.registers.pc += 2;
        let value = mem_bus.fetch_byte(addr);
        self.registers.write_reg(to, value);

        12
    }

    fn load_word(&mut self, reg: Register16, mem_bus: &mut MemoryBus) -> u8 {
        let value = mem_bus.fetch_word(self.registers.pc);
        self.registers.pc += 2;
        self.registers.write_reg16(reg, value);

        12
    }

    fn copy_to_loaded_address(&mut self, from: Register, mem_bus: &mut MemoryBus) -> u8 {
        let addr = mem_bus.fetch_word(self.registers.pc);
        self.registers.pc += 2;
        let value = self.registers.read_reg(from);
        mem_bus.write_byte(addr, value);

        12
    }

    fn load_to_address(&mut self, reg_addr: Register16, mem_bus: &mut MemoryBus) -> u8 {
        let value = mem_bus.fetch_byte(self.registers.pc);
        self.registers.pc += 1;
        let addr = self.registers.read_reg16(reg_addr);
        mem_bus.write_byte(addr, value);

        16
    }

}
