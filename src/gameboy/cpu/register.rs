#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Flags(u8);

impl Flags {
    pub const Z: u8 = 0b1000_0000;
    pub const N: u8 = 0b0100_0000;
    pub const H: u8 = 0b0010_0000;
    pub const C: u8 = 0b0001_0000;

    pub fn get(&self, mask: u8) -> bool {
        (self.0 & mask) != 0
    }

    pub fn set(&mut self, mask: u8, value: bool) {
        if value {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

pub struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Register {
    // Constructor
    pub fn new() -> Self {
        Self {
            // Pair AF
            a: 0x01,
            f: Flags(0xB0),

            // Pair BC
            b: 0x00,
            c: 0x13,

            // Pair DE
            d: 0x00,
            e: 0xD8,

            // Pair HL
            h: 0x01,
            l: 0x4D,

            // Adress Registers
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn get_u8(&self, reg: R8) -> u8 {
        match reg {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    pub fn set_u8(&mut self, reg: R8, value: u8) {
        match reg {
            R8::A => self.a = value,
            R8::B => self.b = value,
            R8::C => self.c = value,
            R8::D => self.d = value,
            R8::E => self.e = value,
            R8::H => self.h = value,
            R8::L => self.l = value,
        }
    }

    pub fn get_u16(&self, reg: R16) -> u16 {
        match reg {
            R16::AF => (u16::from(self.a) << 8) | u16::from(self.f.0 & 0xF0),
            R16::BC => (u16::from(self.b) << 8) | u16::from(self.c),
            R16::DE => (u16::from(self.d) << 8) | u16::from(self.e),
            R16::HL => (u16::from(self.h) << 8) | u16::from(self.l),
            R16::SP => self.sp,
            R16::PC => self.pc,
        }
    }

    pub fn set_u16(&mut self, reg: R16, value: u16) {
        match reg {
            R16::AF => {
                self.a = (value >> 8) as u8;
                self.f = Flags((value & 0xF0) as u8);
            }
            R16::BC => {
                self.b = (value >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            R16::DE => {
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            R16::HL => {
                self.h = (value >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            R16::SP => self.sp = value,
            R16::PC => self.pc = value,
        }
    }

    pub fn get_flag(&self, mask: u8) -> bool {
        self.f.get(mask)
    }

    pub fn set_flag(&mut self, mask: u8, value: bool) {
        self.f.set(mask, value);
    }
}
