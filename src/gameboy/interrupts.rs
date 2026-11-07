pub enum InterruptSource {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub struct Interrupts {
    ie: u8,
    if_: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            ie: 0x00,
            if_: 0xE1,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF0F => self.if_,
            0xFFFF => self.ie,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF0F => self.if_ = value,
            0xFFFF => self.ie = value,
            _ => unreachable!(),
        }
    }

    pub fn request(&mut self, source: InterruptSource) {
        match source {
            InterruptSource::VBlank => self.if_ |= 0x01,
            InterruptSource::LcdStat => self.if_ |= 0x02,
            InterruptSource::Timer => self.if_ |= 0x04,
            InterruptSource::Serial => self.if_ |= 0x08,
            InterruptSource::Joypad => self.if_ |= 0x10,
        }
    }
}
