use crate::{
    instructions::{Instruction, functions::*},
    registers::{Reg, Reg16},
    utils,
};

pub static INSTRUCTIONS: [Instruction; 256] = [
    Instruction::new(0x00, "NOP", |_, _| 4),
    Instruction::new(0x01, "LD BC, n16", |cpu, mmu| ld_imm16(cpu, mmu, Reg16::BC)),
    Instruction::new(0x02, "LD [BC], A", |cpu, mmu| {
        ld_reg_addr(cpu, mmu, Reg::A, Reg16::BC)
    }),
    Instruction::new(0x03, "INC BC", |_, _| todo!("INC BC")),
    Instruction::new(0x04, "INC B", |_, _| todo!("INC B")),
    Instruction::new(0x05, "DEC B", |_, _| todo!("DEC B")),
    Instruction::new(0x06, "LD B, n8", |cpu, mmu| ld_imm8(cpu, mmu, Reg::B)),
    Instruction::new(0x07, "RLCA", |_, _| todo!("RLCA")),
    Instruction::new(0x08, "LD [n16], SP", |cpu, mmu| {
        let addr = cpu.fetch_immediate_word(mmu);
        let sp = cpu.registers.sp;
        let (hi, lo) = utils::split_hi_lo(sp);

        mmu.write_byte(addr, lo);
        mmu.write_byte(addr + 1, hi);
        20
    }),
    Instruction::new(0x09, "ADD HL, BC", |_, _| todo!("ADD HL, BC")),
    Instruction::new(0x0A, "LD A, [BC]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::BC, Reg::A)
    }),
    Instruction::new(0x0B, "DEC BC", |_, _| todo!("DEC BC")),
    Instruction::new(0x0C, "INC C", |_, _| todo!("INC C")),
    Instruction::new(0x0D, "DEC C", |_, _| todo!("DEC C")),
    Instruction::new(0x0E, "LD C, n8", |cpu, mmu| ld_imm8(cpu, mmu, Reg::C)),
    Instruction::new(0x0F, "RRCA", |_, _| todo!("RRCA")),
    Instruction::new(0x10, "STOP", |_, _| todo!("STOP")),
    Instruction::new(0x11, "LD DE, n16", |cpu, mmu| ld_imm16(cpu, mmu, Reg16::DE)),
    Instruction::new(0x12, "LD [DE], A", |cpu, mmu| {
        ld_reg_addr(cpu, mmu, Reg::A, Reg16::DE)
    }),
    Instruction::new(0x13, "INC DE", |_, _| todo!("INC DE")),
    Instruction::new(0x14, "INC D", |_, _| todo!("INC D")),
    Instruction::new(0x15, "DEC D", |_, _| todo!("DEC D")),
    Instruction::new(0x16, "LD D, n8", |cpu, mmu| ld_imm8(cpu, mmu, Reg::D)),
    Instruction::new(0x17, "RLA", |_, _| todo!("RLA")),
    Instruction::new(0x18, "JR n8", |_, _| todo!("JR n8")),
    Instruction::new(0x19, "ADD HL, DE", |_, _| todo!("ADD HL, DE")),
    Instruction::new(0x1A, "LD A, [DE]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::DE, Reg::A)
    }),
    Instruction::new(0x1B, "DEC DE", |_, _| todo!("DEC DE")),
    Instruction::new(0x1C, "INC E", |_, _| todo!("INC E")),
    Instruction::new(0x1D, "DEC E", |_, _| todo!("DEC E")),
    Instruction::new(0x1E, "LD E, n8", |cpu, mmu| ld_imm8(cpu, mmu, Reg::E)),
    Instruction::new(0x1F, "RRA", |_, _| todo!("RRA")),
    Instruction::new(0x20, "JR NZ, n8", |_, _| todo!("JR NZ, n8")),
    Instruction::new(0x21, "LD HL, n16", |cpu, mmu| ld_imm16(cpu, mmu, Reg16::HL)),
    Instruction::new(0x22, "LD [HL+], A", |cpu, mmu| {
        ld_reg_hl(cpu, mmu, Reg::A);
        let hl = cpu.registers.read_hl();
        cpu.registers.write_hl(hl.wrapping_add(1));
        8
    }),
    Instruction::new(0x23, "INC HL", |_, _| todo!("INC HL")),
    Instruction::new(0x24, "INC H", |_, _| todo!("INC H")),
    Instruction::new(0x25, "DEC H", |_, _| todo!("DEC H")),
    Instruction::new(0x26, "LD H, n8", |cpu, mmu| ld_imm8(cpu, mmu, Reg::H)),
    Instruction::new(0x27, "DA2A", |_, _| todo!("DAA")),
    Instruction::new(0x28, "JR Z, n8", |_, _| todo!("JR Z, n8")),
    Instruction::new(0x29, "ADD HL, HL", |_, _| todo!("ADD HL, HL")),
    Instruction::new(0x2A, "LD A, [HL+]", |cpu, mmu| {
        let hl = cpu.registers.read_hl();
        let value = mmu.read_byte(hl);
        cpu.registers.a = value;
        cpu.registers.write_hl(hl.wrapping_add(1));
        8
    }),
    Instruction::new(0x2B, "DEC HL", |_, _| todo!("DEC HL")),
    Instruction::new(0x2C, "INC L", |_, _| todo!("INC L")),
    Instruction::new(0x2D, "DEC L", |_, _| todo!("DEC L")),
    Instruction::new(0x2E, "LD L, n8", |cpu, mmu| ld_imm8(cpu, mmu, Reg::L)),
    Instruction::new(0x2F, "CPL", |_, _| todo!("CPL")),
    Instruction::new(0x30, "JR NC, n8", |_, _| todo!("JR NC, n8")),
    Instruction::new(0x31, "LD SP, n16", |cpu, mmu| ld_imm16(cpu, mmu, Reg16::SP)),
    Instruction::new(0x32, "LD [HL-], A", |cpu, mmu| {
        ld_reg_hl(cpu, mmu, Reg::A);
        let hl = cpu.registers.read_hl();
        cpu.registers.write_hl(hl.wrapping_sub(1));
        8
    }),
    Instruction::new(0x33, "INC SP", |_, _| todo!("INC SP")),
    Instruction::new(0x34, "INC [HL]", |_, _| todo!("INC [HL]")),
    Instruction::new(0x35, "DEC [HL]", |_, _| todo!("DEC [HL]")),
    Instruction::new(0x36, "LD [HL], n8", |cpu, mmu| ld_imm8_hl(cpu, mmu)),
    Instruction::new(0x37, "SCF", |_, _| todo!("SCF")),
    Instruction::new(0x38, "JR C, n8", |_, _| todo!("JR C, n8")),
    Instruction::new(0x39, "ADD HL, SP", |_, _| todo!("ADD HL, SP")),
    Instruction::new(0x3A, "LD A, [HL-]", |cpu, mmu| {
        let hl = cpu.registers.read_hl();
        let value = mmu.read_byte(hl);
        cpu.registers.a = value;
        cpu.registers.write_hl(hl.wrapping_sub(1));
        8
    }),
    Instruction::new(0x3B, "DEC SP", |_, _| todo!("DEC SP")),
    Instruction::new(0x3C, "INC A", |_, _| todo!("INC A")),
    Instruction::new(0x3D, "DEC A", |_, _| todo!("DEC A")),
    Instruction::new(0x3E, "LD A, n8", |cpu, mmu| ld_imm8(cpu, mmu, Reg::A)),
    Instruction::new(0x3F, "CCF", |_, _| todo!("CCF")),
    Instruction::new(0x40, "LD B, B", |cpu, _| ld(cpu, Reg::B, Reg::B)),
    Instruction::new(0x41, "LD B, C", |cpu, _| ld(cpu, Reg::B, Reg::C)),
    Instruction::new(0x42, "LD B, D", |cpu, _| ld(cpu, Reg::B, Reg::D)),
    Instruction::new(0x43, "LD B, E", |cpu, _| ld(cpu, Reg::B, Reg::E)),
    Instruction::new(0x44, "LD B, H", |cpu, _| ld(cpu, Reg::B, Reg::H)),
    Instruction::new(0x45, "LD B, L", |cpu, _| ld(cpu, Reg::B, Reg::L)),
    Instruction::new(0x46, "LD B, [HL]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::HL, Reg::B)
    }),
    Instruction::new(0x47, "LD B, A", |cpu, _| ld(cpu, Reg::B, Reg::A)),
    Instruction::new(0x48, "LD C, B", |cpu, _| ld(cpu, Reg::C, Reg::B)),
    Instruction::new(0x49, "LD C, C", |cpu, _| ld(cpu, Reg::C, Reg::C)),
    Instruction::new(0x4A, "LD C, D", |cpu, _| ld(cpu, Reg::C, Reg::D)),
    Instruction::new(0x4B, "LD C, E", |cpu, _| ld(cpu, Reg::C, Reg::E)),
    Instruction::new(0x4C, "LD C, H", |cpu, _| ld(cpu, Reg::C, Reg::H)),
    Instruction::new(0x4D, "LD C, L", |cpu, _| ld(cpu, Reg::C, Reg::L)),
    Instruction::new(0x4E, "LD C, [HL]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::HL, Reg::C)
    }),
    Instruction::new(0x4F, "LD C, A", |cpu, _| ld(cpu, Reg::C, Reg::A)),
    Instruction::new(0x50, "LD D, B", |cpu, _| ld(cpu, Reg::D, Reg::B)),
    Instruction::new(0x51, "LD D, C", |cpu, _| ld(cpu, Reg::D, Reg::C)),
    Instruction::new(0x52, "LD D, D", |cpu, _| ld(cpu, Reg::D, Reg::D)),
    Instruction::new(0x53, "LD D, E", |cpu, _| ld(cpu, Reg::D, Reg::E)),
    Instruction::new(0x54, "LD D, H", |cpu, _| ld(cpu, Reg::D, Reg::H)),
    Instruction::new(0x55, "LD D, L", |cpu, _| ld(cpu, Reg::D, Reg::L)),
    Instruction::new(0x56, "LD D, [HL]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::HL, Reg::D)
    }),
    Instruction::new(0x57, "LD D, A", |cpu, _| ld(cpu, Reg::D, Reg::A)),
    Instruction::new(0x58, "LD E, B", |cpu, _| ld(cpu, Reg::E, Reg::B)),
    Instruction::new(0x59, "LD E, C", |cpu, _| ld(cpu, Reg::E, Reg::C)),
    Instruction::new(0x5A, "LD E, D", |cpu, _| ld(cpu, Reg::E, Reg::D)),
    Instruction::new(0x5B, "LD E, E", |cpu, _| ld(cpu, Reg::E, Reg::E)),
    Instruction::new(0x5C, "LD E, H", |cpu, _| ld(cpu, Reg::E, Reg::H)),
    Instruction::new(0x5D, "LD E, L", |cpu, _| ld(cpu, Reg::E, Reg::L)),
    Instruction::new(0x5E, "LD E, [HL]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::HL, Reg::E)
    }),
    Instruction::new(0x5F, "LD E, A", |cpu, _| ld(cpu, Reg::E, Reg::A)),
    Instruction::new(0x60, "LD H, B", |cpu, _| ld(cpu, Reg::H, Reg::B)),
    Instruction::new(0x61, "LD H, C", |cpu, _| ld(cpu, Reg::H, Reg::C)),
    Instruction::new(0x62, "LD H, D", |cpu, _| ld(cpu, Reg::H, Reg::D)),
    Instruction::new(0x63, "LD H, E", |cpu, _| ld(cpu, Reg::H, Reg::E)),
    Instruction::new(0x64, "LD H, H", |cpu, _| ld(cpu, Reg::H, Reg::H)),
    Instruction::new(0x65, "LD H, L", |cpu, _| ld(cpu, Reg::H, Reg::L)),
    Instruction::new(0x66, "LD H, [HL]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::HL, Reg::H)
    }),
    Instruction::new(0x67, "LD H, A", |cpu, _| ld(cpu, Reg::H, Reg::A)),
    Instruction::new(0x68, "LD L, B", |cpu, _| ld(cpu, Reg::L, Reg::B)),
    Instruction::new(0x69, "LD L, C", |cpu, _| ld(cpu, Reg::L, Reg::C)),
    Instruction::new(0x6A, "LD L, D", |cpu, _| ld(cpu, Reg::L, Reg::D)),
    Instruction::new(0x6B, "LD L, E", |cpu, _| ld(cpu, Reg::L, Reg::E)),
    Instruction::new(0x6C, "LD L, H", |cpu, _| ld(cpu, Reg::L, Reg::H)),
    Instruction::new(0x6D, "LD L, L", |cpu, _| ld(cpu, Reg::L, Reg::L)),
    Instruction::new(0x6E, "LD L, [HL]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::HL, Reg::L)
    }),
    Instruction::new(0x6F, "LD L, A", |cpu, _| ld(cpu, Reg::L, Reg::A)),
    Instruction::new(0x70, "LD [HL], B", |cpu, mmu| ld_reg_hl(cpu, mmu, Reg::B)),
    Instruction::new(0x71, "LD [HL], C", |cpu, mmu| ld_reg_hl(cpu, mmu, Reg::C)),
    Instruction::new(0x72, "LD [HL], D", |cpu, mmu| ld_reg_hl(cpu, mmu, Reg::D)),
    Instruction::new(0x73, "LD [HL], E", |cpu, mmu| ld_reg_hl(cpu, mmu, Reg::E)),
    Instruction::new(0x74, "LD [HL], H", |cpu, mmu| ld_reg_hl(cpu, mmu, Reg::H)),
    Instruction::new(0x75, "LD [HL], L", |cpu, mmu| ld_reg_hl(cpu, mmu, Reg::L)),
    Instruction::new(0x76, "HALT", |_, _| todo!("HALT")),
    Instruction::new(0x77, "LD [HL], A", |cpu, mmu| ld_reg_hl(cpu, mmu, Reg::A)),
    Instruction::new(0x78, "LD A, B", |cpu, _| ld(cpu, Reg::A, Reg::B)),
    Instruction::new(0x79, "LD A, C", |cpu, _| ld(cpu, Reg::A, Reg::C)),
    Instruction::new(0x7A, "LD A, D", |cpu, _| ld(cpu, Reg::A, Reg::D)),
    Instruction::new(0x7B, "LD A, E", |cpu, _| ld(cpu, Reg::A, Reg::E)),
    Instruction::new(0x7C, "LD A, H", |cpu, _| ld(cpu, Reg::A, Reg::H)),
    Instruction::new(0x7D, "LD A, L", |cpu, _| ld(cpu, Reg::A, Reg::L)),
    Instruction::new(0x7E, "LD A, [HL]", |cpu, mmu| {
        ld_addr_reg(cpu, mmu, Reg16::HL, Reg::A)
    }),
    Instruction::new(0x7F, "LD A, A", |cpu, _| ld(cpu, Reg::A, Reg::A)),
    Instruction::new(0x80, "ADD A, B", |cpu, _| add(cpu, Reg::B)),
    Instruction::new(0x81, "ADD A, C", |cpu, _| add(cpu, Reg::C)),
    Instruction::new(0x82, "ADD A, D", |cpu, _| add(cpu, Reg::D)),
    Instruction::new(0x83, "ADD A, E", |cpu, _| add(cpu, Reg::E)),
    Instruction::new(0x84, "ADD A, H", |cpu, _| add(cpu, Reg::H)),
    Instruction::new(0x85, "ADD A, L", |cpu, _| add(cpu, Reg::L)),
    Instruction::new(0x86, "ADD A, [HL]", |_, _| todo!("ADD A, [HL]")),
    Instruction::new(0x87, "ADD A, A", |cpu, _| add(cpu, Reg::A)),
    Instruction::new(0x88, "ADC A, B", |cpu, _| adc(cpu, Reg::B)),
    Instruction::new(0x89, "ADC A, C", |cpu, _| adc(cpu, Reg::C)),
    Instruction::new(0x8A, "ADC A, D", |cpu, _| adc(cpu, Reg::D)),
    Instruction::new(0x8B, "ADC A, E", |cpu, _| adc(cpu, Reg::E)),
    Instruction::new(0x8C, "ADC A, H", |cpu, _| adc(cpu, Reg::H)),
    Instruction::new(0x8D, "ADC A, L", |cpu, _| adc(cpu, Reg::L)),
    Instruction::new(0x8E, "ADC A, [HL]", |_, _| todo!("ADC A, [HL]")),
    Instruction::new(0x8F, "ADC A, A", |_, _| todo!("ADC A, A")),
    Instruction::new(0x90, "SUB A, B", |_, _| todo!("SUB A, B")),
    Instruction::new(0x91, "SUB A, C", |_, _| todo!("SUB A, C")),
    Instruction::new(0x92, "SUB A, D", |_, _| todo!("SUB A, D")),
    Instruction::new(0x93, "SUB A, E", |_, _| todo!("SUB A, E")),
    Instruction::new(0x94, "SUB A, H", |_, _| todo!("SUB A, H")),
    Instruction::new(0x95, "SUB A, L", |_, _| todo!("SUB A, L")),
    Instruction::new(0x96, "SUB A, [HL]", |_, _| todo!("SUB A, [HL]")),
    Instruction::new(0x97, "SUB A, A", |_, _| todo!("SUB A, A")),
    Instruction::new(0x98, "SBC A, B", |_, _| todo!("SBC A, B")),
    Instruction::new(0x99, "SBC A, C", |_, _| todo!("SBC A, C")),
    Instruction::new(0x9A, "SBC A, D", |_, _| todo!("SBC A, D")),
    Instruction::new(0x9B, "SBC A, E", |_, _| todo!("SBC A, E")),
    Instruction::new(0x9C, "SBC A, H", |_, _| todo!("SBC A, H")),
    Instruction::new(0x9D, "SBC A, L", |_, _| todo!("SBC A, L")),
    Instruction::new(0x9E, "SBC A, [HL]", |_, _| todo!("SBC A, [HL]")),
    Instruction::new(0x9F, "SBC A, A", |_, _| todo!("SBC A, A")),
    Instruction::new(0xA0, "AND A, B", |_, _| todo!("AND A, B")),
    Instruction::new(0xA1, "AND A, C", |_, _| todo!("AND A, C")),
    Instruction::new(0xA2, "AND A, D", |_, _| todo!("AND A, D")),
    Instruction::new(0xA3, "AND A, E", |_, _| todo!("AND A, E")),
    Instruction::new(0xA4, "AND A, H", |_, _| todo!("AND A, H")),
    Instruction::new(0xA5, "AND A, L", |_, _| todo!("AND A, L")),
    Instruction::new(0xA6, "AND A, [HL]", |_, _| todo!("AND A, [HL]")),
    Instruction::new(0xA7, "AND A, A", |_, _| todo!("AND A, A")),
    Instruction::new(0xA8, "XOR A, B", |_, _| todo!("XOR A, B")),
    Instruction::new(0xA9, "XOR A, C", |_, _| todo!("XOR A, C")),
    Instruction::new(0xAA, "XOR A, D", |_, _| todo!("XOR A, D")),
    Instruction::new(0xAB, "XOR A, E", |_, _| todo!("XOR A, E")),
    Instruction::new(0xAC, "XOR A, H", |_, _| todo!("XOR A, H")),
    Instruction::new(0xAD, "XOR A, L", |_, _| todo!("XOR A, L")),
    Instruction::new(0xAE, "XOR A, [HL]", |_, _| todo!("XOR A, [HL]")),
    Instruction::new(0xAF, "XOR A, A", |_, _| todo!("XOR A, A")),
    Instruction::new(0xB0, "OR A, B", |_, _| todo!("OR A, B")),
    Instruction::new(0xB1, "OR A, C", |_, _| todo!("OR A, C")),
    Instruction::new(0xB2, "OR A, D", |_, _| todo!("OR A, D")),
    Instruction::new(0xB3, "OR A, E", |_, _| todo!("OR A, E")),
    Instruction::new(0xB4, "OR A, H", |_, _| todo!("OR A, H")),
    Instruction::new(0xB5, "OR A, L", |_, _| todo!("OR A, L")),
    Instruction::new(0xB6, "OR A, [HL]", |_, _| todo!("OR A, [HL]")),
    Instruction::new(0xB7, "OR A, A", |_, _| todo!("OR A, A")),
    Instruction::new(0xB8, "CP A, B", |_, _| todo!("CP A, B")),
    Instruction::new(0xB9, "CP A, C", |_, _| todo!("CP A, C")),
    Instruction::new(0xBA, "CP A, D", |_, _| todo!("CP A, D")),
    Instruction::new(0xBB, "CP A, E", |_, _| todo!("CP A, E")),
    Instruction::new(0xBC, "CP A, H", |_, _| todo!("CP A, H")),
    Instruction::new(0xBD, "CP A, L", |_, _| todo!("CP A, L")),
    Instruction::new(0xBE, "CP A, [HL]", |_, _| todo!("CP A, [HL]")),
    Instruction::new(0xBF, "CP A, A", |_, _| todo!("CP A, A")),
    Instruction::new(0xC0, "RET NZ", |_, _| todo!("RET NZ")),
    Instruction::new(0xC1, "POP BC", |_, _| todo!("POP BC")),
    Instruction::new(0xC2, "JP NZ, n16", |_, _| todo!("JP NZ, n16")),
    Instruction::new(0xC3, "JP n16", |_, _| todo!("JP n16")),
    Instruction::new(0xC4, "CALL NZ, n16", |_, _| todo!("CALL NZ, n16")),
    Instruction::new(0xC5, "PUSH BC", |_, _| todo!("PUSH BC")),
    Instruction::new(0xC6, "ADD A, n8", |cpu, mmu| add_imm8(cpu, mmu)),
    Instruction::new(0xC7, "RST 00h", |_, _| todo!("RST 00h")),
    Instruction::new(0xC8, "RET Z", |_, _| todo!("RET Z")),
    Instruction::new(0xC9, "RET", |_, _| todo!("RET")),
    Instruction::new(0xCA, "JP Z, n16", |_, _| todo!("JP Z, n16")),
    Instruction::new(0xCB, "PREFIX CB", |_, _| todo!("PREFIX CB")),
    Instruction::new(0xCC, "CALL Z, n16", |_, _| todo!("CALL Z, n16")),
    Instruction::new(0xCD, "CALL n16", |_, _| todo!("CALL n16")),
    Instruction::new(0xCE, "ADC A, n8", |_, _| todo!("ADC A, n8")),
    Instruction::new(0xCF, "RST 08h", |_, _| todo!("RST 08h")),
    Instruction::new(0xD0, "RET NC", |_, _| todo!("RET NC")),
    Instruction::new(0xD1, "POP DE", |_, _| todo!("POP DE")),
    Instruction::new(0xD2, "JP NC, n16", |_, _| todo!("JP NC, n16")),
    Instruction::new(0xD3, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xD4, "CALL NC, n16", |_, _| todo!("CALL NC, n16")),
    Instruction::new(0xD5, "PUSH DE", |_, _| todo!("PUSH DE")),
    Instruction::new(0xD6, "SUB A, n8", |_, _| todo!("SUB A, n8")),
    Instruction::new(0xD7, "RST 10h", |_, _| todo!("RST 10h")),
    Instruction::new(0xD8, "RET C", |_, _| todo!("RET C")),
    Instruction::new(0xD9, "RETI", |_, _| todo!("RETI")),
    Instruction::new(0xDA, "JP C, n16", |_, _| todo!("JP C, n16")),
    Instruction::new(0xDB, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xDC, "CALL C, n16", |_, _| todo!("CALL C, n16")),
    Instruction::new(0xDD, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xDE, "SBC A, n8", |_, _| todo!("SBC A, n8")),
    Instruction::new(0xDF, "RST 18h", |_, _| todo!("RST 18h")),
    Instruction::new(0xE0, "LDH [n8], A", |cpu, mmu| {
        let value = cpu.registers.a;
        let offset = cpu.fetch_immediate_byte(mmu);
        let addr = 0xFF00u16.wrapping_add(offset as u16);
        mmu.write_byte(addr, value);
        12
    }),
    Instruction::new(0xE1, "POP HL", |_, _| todo!("POP HL")),
    Instruction::new(0xE2, "LDH [C], A", |cpu, mmu| {
        let offset = cpu.registers.c;
        let addr = 0xFF00u16.wrapping_add(offset as u16);
        let value = cpu.registers.a;
        mmu.write_byte(addr, value);
        8
    }),
    Instruction::new(0xE3, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xE4, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xE5, "PUSH HL", |_, _| todo!("PUSH HL")),
    Instruction::new(0xE6, "AND A, n8", |_, _| todo!("AND A, n8")),
    Instruction::new(0xE7, "RST 20h", |_, _| todo!("RST 20h")),
    Instruction::new(0xE8, "ADD SP, n8", |_, _| todo!("ADD SP, n8")),
    Instruction::new(0xE9, "JP HL", |_, _| todo!("JP HL")),
    Instruction::new(0xEA, "LD [n16], A", |cpu, mmu| {
        ld_reg_imm_addr(cpu, mmu, Reg::A)
    }),
    Instruction::new(0xEB, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xEC, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xED, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xEE, "XOR A, n8", |_, _| todo!("XOR A, n8")),
    Instruction::new(0xEF, "RST 28h", |_, _| todo!("RST 28h")),
    Instruction::new(0xF0, "LDH A, [n8]", |cpu, mmu| {
        let addr = cpu.fetch_immediate_byte(mmu);
        let addr = 0xFF00u16.wrapping_add(addr as u16);
        let vaue = mmu.read_byte(addr);
        cpu.registers.a = vaue;
        12
    }),
    Instruction::new(0xF1, "POP AF", |_, _| todo!("POP AF")),
    Instruction::new(0xF2, "LDH A, [C]", |cpu, mmu| {
        let offset = cpu.registers.c;
        let addr = 0xFF00u16.wrapping_add(offset as u16);
        let value = mmu.read_byte(addr);
        cpu.registers.a = value;

        8
    }),
    Instruction::new(0xF3, "DI", |_, _| todo!("DI")),
    Instruction::new(0xF4, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xF5, "PUSH AF", |_, _| todo!("PUSH AF")),
    Instruction::new(0xF6, "OR A, n8", |_, _| todo!("OR A, n8")),
    Instruction::new(0xF7, "RST 30h", |_, _| todo!("RST 30h")),
    Instruction::new(0xF8, "LD HL, SP+n8", |_, _| todo!("LD HL, SP+n8")),
    Instruction::new(0xF9, "LD SP, HL", |_, _| todo!("LD SP, HL")),
    Instruction::new(0xFA, "LD A, [n16]", |cpu, mmu| {
        let addr = cpu.fetch_immediate_word(mmu);
        let value = mmu.read_byte(addr);
        cpu.registers.a = value;
        16
    }),
    Instruction::new(0xFB, "EI", |_, _| todo!("EI")),
    Instruction::new(0xFC, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xFD, "INVALID", |_, _| todo!("INVALID")),
    Instruction::new(0xFE, "CP A, n8", |_, _| todo!("CP A, n8")),
    Instruction::new(0xFF, "RST 38h", |_, _| todo!("RST 38h")),
];
