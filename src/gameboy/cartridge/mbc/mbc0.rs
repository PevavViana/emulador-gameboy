use crate::gameboy::cartridge::mbc::Mbc;

pub struct Mbc0;

impl Mbc for Mbc0 {
    fn read_rom(&self, rom: &[u8], address: u16) -> u8 {
        rom[usize::from(address)]
    }

    fn read_ram(&self, _ram: &[u8], _address: u16) -> u8 {
        0x00
    }

    fn write_rom(&mut self, _rom: &mut [u8], _address: u16, _value: u8) {}

    fn write_ram(&mut self, _ram: &mut [u8], _address: u16, _value: u8) {}
}
