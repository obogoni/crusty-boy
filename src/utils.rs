pub fn split_hi_lo(value: u16) -> (u8, u8) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;

    (hi, lo)
}

pub fn has_half_carry(a: u8, b: u8) -> bool {
    ((a & 0x0F) + (b & 0x0F)) > 0x0F
}

#[cfg(test)]
mod tests {
    use crate::utils::split_hi_lo;

    #[test]
    fn test_split_hi_lo() {
        let value: u16 = 0x1234;

        let (hi, lo) = split_hi_lo(value);

        assert_eq!(hi, 0x12);
        assert_eq!(lo, 0x34);
    }
}
