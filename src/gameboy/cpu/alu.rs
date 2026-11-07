use crate::gameboy::cpu::register::Flags;

pub struct Alu;

impl Alu {
    pub fn add(reg_a: u8, y: u8) -> (u8, Flags) {
        let (result, carry) = reg_a.overflowing_add(y);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, (reg_a & 0xF) + (y & 0xF) > 0xF);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn adc(reg_a: u8, y: u8, flags: Flags) -> (u8, Flags) {
        let carry_in = flags.get(Flags::C) as u8;
        let (result, carry1) = reg_a.overflowing_add(y);
        let (result, carry2) = result.overflowing_add(carry_in);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, (reg_a & 0xF) + (y & 0xF) + carry_in > 0xF);
        flags.set(Flags::C, carry1 || carry2);

        (result, flags)
    }

    pub fn sub(reg_a: u8, y: u8) -> (u8, Flags) {
        let (result, carry) = reg_a.overflowing_sub(y);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, true);
        flags.set(Flags::H, (reg_a & 0x0F) < (y & 0x0F));
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn sbc(reg_a: u8, y: u8, flags: Flags) -> (u8, Flags) {
        let carry_in = flags.get(Flags::C) as u8;
        let (result, carry1) = reg_a.overflowing_sub(y);
        let (result, carry2) = result.overflowing_sub(carry_in);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, true);
        flags.set(Flags::H, (reg_a & 0x0F) < (y & 0x0f) + carry_in);
        flags.set(Flags::C, carry1 || carry2);

        (result, flags)
    }

    pub fn cp(reg_a: u8, y: u8) -> Flags {
        let (_, flags) = Self::sub(reg_a, y);
        flags
    }

    pub fn inc_u8(x: u8, mut flags: Flags) -> (u8, Flags) {
        let result = x.wrapping_add(1);

        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, (x & 0x0F) + 1 > 0x0F);

        (result, flags)
    }

    pub fn dec_u8(x: u8, mut flags: Flags) -> (u8, Flags) {
        let result = x.wrapping_sub(1);

        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, true);
        flags.set(Flags::H, (x & 0x0F) < 1);

        (result, flags)
    }

    pub fn inc_u16(x: u16) -> u16 {
        x.wrapping_add(1)
    }

    pub fn dec_u16(x: u16) -> u16 {
        x.wrapping_sub(1)
    }

    pub fn add_sp(sp: u16, y: i8) -> (u16, Flags) {
        let offset = y as u16;
        let result = sp.wrapping_add(offset);
        let (_, carry) = (sp as u8).overflowing_add(offset as u8);

        let mut flags = Flags::default();

        flags.set(Flags::Z, false);
        flags.set(Flags::N, false);
        flags.set(Flags::H, (sp & 0x0F) + (offset & 0x0F) > 0x0F);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn add_hl(hl: u16, y: u16, mut flags: Flags) -> (u16, Flags) {
        let (result, carry) = hl.overflowing_add(y);

        flags.set(Flags::N, false);
        flags.set(Flags::H, (hl & 0x0FFF) + (y & 0x0FFF) > 0x0FFF);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn and(reg_a: u8, y: u8) -> (u8, Flags) {
        let result = reg_a & y;

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0x00);
        flags.set(Flags::N, false);
        flags.set(Flags::H, true);
        flags.set(Flags::C, false);

        (result, flags)
    }

    pub fn or(reg_a: u8, y: u8) -> (u8, Flags) {
        let result = reg_a | y;

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0x00);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, false);

        (result, flags)
    }

    pub fn xor(reg_a: u8, y: u8) -> (u8, Flags) {
        let result = reg_a ^ y;

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0x00);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, false);

        (result, flags)
    }

    pub fn cpl(reg_a: u8, mut flags: Flags) -> (u8, Flags) {
        let result = !reg_a;

        flags.set(Flags::N, true);
        flags.set(Flags::H, true);

        (result, flags)
    }

    pub fn bit(x: u8, bit_offset: u8, mut flags: Flags) -> Flags {
        let result = x & (1 << bit_offset) == 0;

        flags.set(Flags::Z, result);
        flags.set(Flags::N, false);
        flags.set(Flags::H, true);

        flags
    }

    pub fn res(x: u8, bit_offset: u8) -> u8 {
        x & !(1 << bit_offset)
    }

    pub fn set(x: u8, bit_offset: u8) -> u8 {
        x | (1 << bit_offset)
    }

    pub fn rl(x: u8, mut flags: Flags) -> (u8, Flags) {
        let new_carry = x & 0x80 != 0;
        let result = (x << 1) | (flags.get(Flags::C) as u8);

        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, new_carry);

        (result, flags)
    }

    pub fn rlc(x: u8) -> (u8, Flags) {
        let carry = x & 0x80 != 0;
        let result = (x << 1) | (carry as u8);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn rr(x: u8, mut flags: Flags) -> (u8, Flags) {
        let new_carry = x & 0x01 != 0;
        let result = (x >> 1) | ((flags.get(Flags::C) as u8) << 7);

        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, new_carry);

        (result, flags)
    }

    pub fn rrc(x: u8) -> (u8, Flags) {
        let carry = x & 0x01 != 0;
        let result = (x >> 1) | ((carry as u8) << 7);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn sla(x: u8) -> (u8, Flags) {
        let carry = x & 0x80 != 0;
        let result = x << 1;

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn sra(x: u8) -> (u8, Flags) {
        let bit_clone = x & 0x80 != 0;
        let carry = x & 0x01 != 0;
        let result = x >> 1 | ((bit_clone as u8) << 7);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn srl(x: u8) -> (u8, Flags) {
        let carry = x & 0x01 != 0;
        let result = x >> 1;

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, carry);

        (result, flags)
    }

    pub fn swap(x: u8) -> (u8, Flags) {
        let result = (x << 4) | (x >> 4);

        let mut flags = Flags::default();
        flags.set(Flags::Z, result == 0);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags.set(Flags::C, false);

        (result, flags)
    }

    pub fn ccf(mut flags: Flags) -> Flags {
        let carry = flags.get(Flags::C);

        flags.set(Flags::C, !carry);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags
    }

    pub fn scf(mut flags: Flags) -> Flags {
        flags.set(Flags::C, true);
        flags.set(Flags::N, false);
        flags.set(Flags::H, false);
        flags
    }

    pub fn daa(reg_a: u8, mut flags: Flags) -> (u8, Flags) {
        let n = flags.get(Flags::N);
        let mut correction: u8 = 0;
        let mut set_c = false;

        if flags.get(Flags::H) || (!n && (reg_a & 0x0F) > 0x09) {
            correction |= 0x06;
        }
        if flags.get(Flags::C) || (!n && reg_a > 0x99) {
            correction |= 0x60;
            set_c = true;
        }

        let result = if n {
            reg_a.wrapping_sub(correction)
        } else {
            reg_a.wrapping_add(correction)
        };

        flags.set(Flags::Z, result == 0);
        flags.set(Flags::H, false);
        flags.set(Flags::C, set_c);

        (result, flags)
    }
}
