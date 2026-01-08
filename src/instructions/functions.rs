use crate::{
    cpu::Cpu,
    mmu::Mmu,
    registers::{Flag, Reg, Reg16, RegFlags},
    utils::has_half_carry,
};

pub fn ld(cpu: &mut Cpu, from: Reg, to: Reg) -> u8 {
    let value = cpu.registers.read_reg(from);
    cpu.registers.write_reg(to, value);

    4
}

pub fn ld_imm8(cpu: &mut Cpu, mmu: &mut Mmu, to: Reg) -> u8 {
    let value = cpu.fetch_immediate_byte(mmu);
    cpu.registers.write_reg(to, value);

    8
}

pub fn ld_reg_hl(cpu: &mut Cpu, mmu: &mut Mmu, from: Reg) -> u8 {
    let addr = cpu.registers.read_reg16(Reg16::HL);
    let value = cpu.registers.read_reg(from);
    mmu.write_byte(addr, value);

    8
}

pub fn ld_reg_addr(cpu: &mut Cpu, mmu: &mut Mmu, from: Reg, to: Reg16) -> u8 {
    let addr = cpu.registers.read_reg16(to);
    let value = cpu.registers.read_reg(from);
    mmu.write_byte(addr, value);

    8
}

pub fn ld_imm16(cpu: &mut Cpu, mmu: &mut Mmu, reg: Reg16) -> u8 {
    let value = mmu.read_word(cpu.registers.pc);
    cpu.registers.write_reg16(reg, value);

    12
}

pub fn ld_imm8_hl(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let value = cpu.fetch_immediate_byte(mmu);
    let addr = cpu.registers.read_reg16(Reg16::HL);
    mmu.write_byte(addr, value);

    12
}

pub fn ld_reg_imm_addr(cpu: &mut Cpu, mmu: &mut Mmu, from: Reg) -> u8 {
    let addr = cpu.fetch_immediate_word(mmu);
    let value = cpu.registers.read_reg(from);
    mmu.write_byte(addr, value);

    16
}

pub fn ld_addr_reg(cpu: &mut Cpu, mmu: &mut Mmu, reg_addr: Reg16, to: Reg) -> u8 {
    let addr = cpu.registers.read_reg16(reg_addr);
    let value = mmu.read_byte(addr);
    cpu.registers.write_reg(to, value);
    8
}

pub fn add(cpu: &mut Cpu, from: Reg) -> u8 {
    let a = cpu.registers.read_reg(Reg::A);
    let value = cpu.registers.read_reg(from);
    let (result, did_overflow) = a.overflowing_add(value);

    cpu.registers.write_reg(Reg::A, result);

    // Set flags
    cpu.registers.set_flag(Flag::Z, result == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, has_half_carry(a, value));
    cpu.registers.set_flag(Flag::C, did_overflow);

    4
}

pub fn adc(cpu: &mut Cpu, from: Reg) -> u8 {
    let a = cpu.registers.read_reg(Reg::A) as u16;
    let value = cpu.registers.read_reg(from) as u16;
    let carry = if cpu.registers.f.carry { 1 } else { 0 };

    let result: u16 = a + value + carry;

    cpu.registers.write_reg(Reg::A, result as u8);

    // Set flags
    cpu.registers.set_flag(Flag::Z, (result & 0xFF) == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers
        .set_flag(Flag::H, ((a & 0x0F) + (value & 0x0F) + carry) > 0x0F);
    cpu.registers.set_flag(Flag::C, result > 0xFF);

    4
}

pub fn add_imm8(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let a = cpu.registers.read_reg(Reg::A);
    let value = cpu.fetch_immediate_byte(mmu);
    let (result, did_overflow) = a.overflowing_add(value);

    cpu.registers.write_reg(Reg::A, result);

    // Set flags
    cpu.registers.set_flag(Flag::Z, result == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, has_half_carry(a, value));
    cpu.registers.set_flag(Flag::C, did_overflow);

    8
}
