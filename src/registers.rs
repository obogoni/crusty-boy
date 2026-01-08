use crate::utils;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub f: RegFlags,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Reg16 {
    BC,
    DE,
    SP,
    HL,
}

pub struct RegFlags {
    pub carry: bool,
    pub zero: bool,
    pub half_carry: bool,
    pub subtract: bool,
}

impl RegFlags {
    pub fn new() -> Self {
        RegFlags {
            carry: false,
            zero: false,
            half_carry: false,
            subtract: false,
        }
    }
}

pub enum Flag {
    Z,
    N,
    H,
    C,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
            f: RegFlags::new(),
        }
    }

    pub fn read_reg(&self, reg: Reg) -> u8 {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
            Reg::E => self.e,
            Reg::H => self.h,
            Reg::L => self.l,
            _ => unreachable!(),
        }
    }

    pub fn write_reg(&mut self, reg: Reg, value: u8) {
        match reg {
            Reg::A => self.a = value,
            Reg::B => self.b = value,
            Reg::C => self.c = value,
            Reg::D => self.d = value,
            Reg::E => self.e = value,
            Reg::H => self.h = value,
            Reg::L => self.l = value,
            _ => unreachable!(),
        }
    }

    pub fn read_reg16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::BC => self.read_bc(),
            Reg16::DE => self.read_de(),
            Reg16::HL => self.read_hl(),
            _ => panic!("Cannot read 8-bit register as 16-bit"),
        }
    }

    pub fn write_reg16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::BC => self.write_bc(value),
            Reg16::DE => self.write_de(value),
            Reg16::HL => self.write_hl(value),
            Reg16::SP => self.sp = value,
            _ => unreachable!(),
        }
    }

    pub fn write_bc(&mut self, value: u16) {
        (self.b, self.c) = utils::split_hi_lo(value);
    }

    pub fn read_bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn write_hl(&mut self, value: u16) {
        (self.h, self.l) = utils::split_hi_lo(value);
    }

    pub fn read_hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn write_de(&mut self, value: u16) {
        (self.d, self.e) = utils::split_hi_lo(value);
    }

    pub fn read_de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_flag(&mut self, flag: Flag, state: bool) {
        match flag {
            Flag::Z => {
                self.f.zero = state;
            }
            Flag::N => {
                self.f.subtract = state;
            }
            Flag::H => {
                self.f.half_carry = state;
            }
            Flag::C => {
                self.f.carry = state;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Registers;
    use crate::registers::RegFlags;
    use crate::utils;

    #[test]
    fn test_split_hi_lo() {
        let value: u16 = 0x1234;

        let (hi, lo) = utils::split_hi_lo(value);

        assert_eq!(hi, 0x12);
        assert_eq!(lo, 0x34);
    }

    #[test]
    fn test_write_bc() {
        let mut reg = Registers::new();

        reg.write_bc(0x1234);

        assert_eq!(reg.b, 0x12);
        assert_eq!(reg.c, 0x34);
    }

    #[test]
    fn test_read_bc() {
        let mut reg = Registers::new();

        reg.b = 0x12;
        reg.c = 0x34;

        let bc = reg.read_bc();

        assert_eq!(bc, 0x1234)
    }

    #[test]
    fn test_write_hl() {
        let mut reg = Registers::new();

        reg.write_hl(0x1234);

        assert_eq!(reg.h, 0x12);
        assert_eq!(reg.l, 0x34);
    }

    #[test]
    fn test_read_hl() {
        let mut reg = Registers::new();

        reg.h = 0x12;
        reg.l = 0x34;

        let hl = reg.read_hl();

        assert_eq!(hl, 0x1234)
    }

    #[test]
    fn test_write_de() {
        let mut reg = Registers::new();

        reg.write_de(0x1234);

        assert_eq!(reg.d, 0x12);
        assert_eq!(reg.e, 0x34);
    }

    #[test]
    fn test_read_de() {
        let mut reg = Registers::new();

        reg.d = 0x12;
        reg.e = 0x34;

        let de = reg.read_de();

        assert_eq!(de, 0x1234)
    }
}
