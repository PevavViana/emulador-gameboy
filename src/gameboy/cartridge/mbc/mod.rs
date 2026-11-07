use crate::gameboy::cartridge::mbc::mbc0::Mbc0;

mod mbc0;

pub trait Mbc {
    fn read_rom(&self, rom: &[u8], address: u16) -> u8;
    fn read_ram(&self, ram: &[u8], address: u16) -> u8;
    fn write_rom(&mut self, rom: &mut[u8], address: u16, value: u8);
    fn write_ram(&mut self, ram: &mut[u8], address: u16, value: u8);
}

pub fn mbc_from_cartridge_type(byte: u8) -> Box<dyn Mbc> {
    match byte {
        0x00 => Box::new(Mbc0),
        _ => panic!("ERROR. Unknown cartridge type: {:02X}", byte)
    }
}

pub fn rom_size_from_code(byte: u8) -> u32 {
    match byte {
        0x00 => 0x8000,
        0x01 => 0x10000,
        0x02 => 0x20000,
        0x03 => 0x40000,
        0x04 => 0x80000,
        0x05 => 0x100000,
        0x06 => 0x200000,
        0x07 => 0x400000,
        0x08 => 0x800000,
        0x52 => 0x120000,
        0x53 => 0x140000,
        0x54 => 0x180000,
        _ => panic!("ERROR. Unknown ROM Size code: {:02X}", byte)
    }
}

pub fn ram_size_from_code(byte: u8) -> u32 {
    match byte {
        0x00 => 0x00,
        0x01 => 0x0800,
        0x02 => 0x2000,
        0x03 => 0x8000,
        0x04 => 0x20000,
        0x05 => 0x10000,
        _ => panic!("ERROR. Unknown RAM Size code: {:02X}", byte)
    }
}

