use crate::{
    instructions::Instruction,
    mmu::Mmu,
    registers::{Reg, Reg16, Registers},
};
pub struct Cpu {
    pub cycles: u16,
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            cycles: 0,
            registers: Registers::new(),
        }
    }

    pub fn fetch_immediate_byte(&mut self, mmu: &mut Mmu) -> u8 {
        let byte = mmu.read_byte(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    pub fn fetch_immediate_word(&mut self, mmu: &mut Mmu) -> u16 {
        let word = mmu.read_word(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    pub fn step(&mut self, mmu: &mut Mmu) {
        let byte = self.fetch_immediate_byte(mmu);
        let inst = Instruction::from_byte(byte).expect("Unkown instruction!");

        let cycles = (inst.execute)(self, mmu);
    }

    fn copy(&mut self, from: Reg, to: Reg) -> u8 {
        self.registers.write_reg(to, self.registers.read_reg(from));

        4
    }

    fn copy_to_address(&mut self, from: Reg, to: Reg16, mem_bus: &mut Mmu) -> u8 {
        let addr = self.registers.read_reg16(to);
        let value = self.registers.read_reg(from);
        mem_bus.write_byte(addr, value);

        8
    }

    fn load_byte(&mut self, to: Reg, mem_bus: &mut Mmu) -> u8 {
        let value = mem_bus.read_byte(self.registers.pc);
        self.registers.pc += 1;
        self.registers.write_reg(to, value);

        8
    }

    fn load_from_address(&mut self, reg_addr: Reg16, to: Reg, mem_bus: &mut Mmu) -> u8 {
        let addr = self.registers.read_reg16(reg_addr);
        let value = mem_bus.read_byte(addr);
        self.registers.write_reg(to, value);

        8
    }

    fn load_from_loaded_address(&mut self, to: Reg, mem_bus: &mut Mmu) -> u8 {
        let addr = mem_bus.read_word(self.registers.pc);
        self.registers.pc += 2;
        let value = mem_bus.read_byte(addr);
        self.registers.write_reg(to, value);

        12
    }

    fn load_word(&mut self, reg: Reg16, mem_bus: &mut Mmu) -> u8 {
        let value = mem_bus.read_word(self.registers.pc);
        self.registers.pc += 2;
        self.registers.write_reg16(reg, value);

        12
    }

    fn copy_to_loaded_address(&mut self, from: Reg, mem_bus: &mut Mmu) -> u8 {
        let addr = mem_bus.read_word(self.registers.pc);
        self.registers.pc += 2;
        let value = self.registers.read_reg(from);
        mem_bus.write_byte(addr, value);

        12
    }

    fn load_to_address(&mut self, reg_addr: Reg16, mem_bus: &mut Mmu) -> u8 {
        let value = mem_bus.read_byte(self.registers.pc);
        self.registers.pc += 1;
        let addr = self.registers.read_reg16(reg_addr);
        mem_bus.write_byte(addr, value);

        16
    }
}
