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
    pub f: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register16 {
    BC,
    DE,
    SP,
    HL
}

pub struct RegisterFlags;

impl RegisterFlags {
    const Z: u8 = 1 << 7; //Zero = 10000000
    const N: u8 = 1 << 6; //Subtract = 01000000
    const H: u8 = 1 << 5; //Half Carry = 001000000
    const C: u8 = 1 << 4; //Carry = 00010000
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
            f: 0,
        }
    }

    pub fn read_reg(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            _ => unreachable!()
        }
    }

    pub fn write_reg(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            _ => unreachable!()
        }
    }

    pub fn read_reg16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::BC => self.read_bc(),
            Register16::DE => self.read_de(),
            Register16::HL => self.read_hl(),
            _ => panic!("Cannot read 8-bit register as 16-bit"),
        }
    }

    pub fn write_bc(&mut self, value: u16) {
        (self.b, self.c) = Self::split_hi_lo(value);
    }

    pub fn read_bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn write_hl(&mut self, value: u16) {
        (self.h, self.l) = Self::split_hi_lo(value);
    }

    pub fn read_hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn write_de(&mut self, value: u16) {
        (self.d, self.e) = Self::split_hi_lo(value);
    }

    pub fn read_de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn write_af(&mut self, value: u16) {
        let (hi, lo) = Self::split_hi_lo(value);

        self.a = hi;
        self.f = lo & 0xF0;
    }

    pub fn read_af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.f])
    }

    fn split_hi_lo(value: u16) -> (u8, u8) {
        let hi = (value >> 8) as u8;
        let lo = value as u8;

        (hi, lo)
    }

    pub(crate) fn write_reg16(&mut self, reg: Register16, value: u16)  {
        match reg {
            Register16::BC => self.write_bc(value),
            Register16::DE => self.write_de(value),
            Register16::HL => self.write_hl(value),
            Register16::SP => self.sp = value,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::RegisterFlags;

    use super::Registers;

    #[test]
    fn test_split_hi_lo() {
        let value: u16 = 0x1234;

        let (hi, lo) = Registers::split_hi_lo(value);

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

    #[test]
    fn test_write_af() {
        //Adds a 1 to the second bit of the flags byte to make sure only the 4 right bits are set
        let value: u16 = 0b00000001_00010010;
        let mut reg = Registers::new();

        reg.write_af(value);

        assert_eq!(reg.a, 0b00000001);
        assert_eq!(reg.f, RegisterFlags::C);
    }

    #[test]
    fn test_read_af() {
        let mut reg = Registers::new();

        reg.a = 1;
        reg.f = RegisterFlags::Z;

        let af = reg.read_af();

        assert_eq!(af, 0b00000001_10000000);
    }
}
