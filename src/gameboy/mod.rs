use crate::gameboy::{cpu::Cpu, mmu::Mmu};

mod cartridge;
mod cpu;
mod interrupts;
mod joypad;
mod mmu;
mod timer;

pub struct Gameboy {
    mmu: Mmu,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            mmu: Mmu::new(rom),
            cpu: Cpu,
        }
    }

    pub fn hex_dump(&self, start: u16, end: u16) {
        self.mmu.hex_dump(start, end);
    }
}
