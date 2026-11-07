use crate::gameboy::{cartridge::Cartridge, interrupts::Interrupts, joypad::Joypad, timer::Timer};

const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x007F;

pub struct Mmu {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    joypad: Joypad,
    timer: Timer,
    hram: [u8; HRAM_SIZE],
    interrupts: Interrupts,
}

impl Mmu {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            cartridge: Cartridge::new(rom),
            wram: [0; WRAM_SIZE],
            joypad: Joypad::new(),
            timer: Timer::new(),
            hram: [0; HRAM_SIZE],
            interrupts: Interrupts::new(),
        }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_rom(address),
            0xA000..=0xBFFF => self.cartridge.read_ram(address - 0xA000),
            0xC000..=0xDFFF => self.wram[usize::from(address) - 0xC000],
            0xE000..=0xFDFF => self.wram[usize::from(address) - 0xC000],
            0xFF00..=0xFF00 => self.joypad.read(),
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F..=0xFF0F => self.interrupts.read(address),
            0xFF80..=0xFFFE => self.hram[usize::from(address) - 0xFF80],
            0xFFFF..=0xFFFF => self.interrupts.read(address),
            _ => panic!("ERROR. Unknown address: {:04X}", address),
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),
            0xA000..=0xBFFF => self.cartridge.write_ram(address - 0xA000, value),
            0xC000..=0xDFFF => self.wram[usize::from(address) - WRAM_SIZE] = value,
            0xE000..=0xFDFF => self.wram[usize::from(address) - WRAM_SIZE] = value,
            0xFF00..=0xFF00 => self.joypad.write(value),
            0xFF04..=0xFF07 => self.timer.write(address, value),
            0xFF0F..=0xFF0F => self.interrupts.write(address, value),
            0xFF80..=0xFFFE => self.hram[usize::from(address) - HRAM_SIZE] = value,
            0xFFFF..=0xFFFF => self.interrupts.write(address, value),
            _ => panic!("ERROR. Unknown address: {:04X}", address),
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let less_significant = self.read_u8(address);
        let most_significant = self.read_u8(address.wrapping_add(1));

        (u16::from(most_significant) << 8) | u16::from(less_significant)
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let less_significant = value as u8;
        let most_significant = (value >> 8) as u8;

        self.write_u8(address, less_significant);
        self.write_u8(address.wrapping_add(1), most_significant)
    }

    pub fn hex_dump(&self, start: u16, end: u16) {
        for address in (start..=end).step_by(16) {
            print!("{:04X} ", address);

            for offset in 0..16 {
                let current = address.wrapping_add(offset);

                if current > end {
                    break;
                }

                print!("{:02X} ", self.read_u8(current));
            }

            println!();
        }
    }
}
