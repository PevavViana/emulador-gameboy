use gameboy_emulator::gameboy::Gameboy;
use std::fs;

fn main() {
    let rom = fs::read("roms/Tetris_japan_en.gb").expect("ERROR. Failed to read ROM file");
    let gameboy = Gameboy::new(rom);

    gameboy.hex_dump(0, 255);
}
