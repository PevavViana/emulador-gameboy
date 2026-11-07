use crate::gameboy::cartridge::mbc::Mbc;
mod mbc;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mbc: Box<dyn Mbc>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        let mbc = mbc::mbc_from_cartridge_type(rom[0x0147]);
        let ram = vec![0; mbc::ram_size_from_code(rom[0x0149]) as usize];

        Self { rom, ram, mbc }
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.mbc.read_rom(&self.rom, address)
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        self.mbc.write_rom(&mut self.rom, address, value);
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.mbc.read_ram(&self.ram, address)
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.mbc.write_ram(&mut self.ram, address, value);
    }
}