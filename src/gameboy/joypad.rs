pub struct Joypad {
    directional_buttons: u8,
    action_buttons: u8,
    selection: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            directional_buttons: 0xFF,
            action_buttons: 0xFF,
            selection: 0xFF,
        }
    }

    pub fn read(&self) -> u8 {
        let mut buttons = 0x0F;

        if self.selection & 0x10 == 0 {
            buttons &= self.directional_buttons;
        }
        if self.selection & 0x20 == 0 {
            buttons &= self.action_buttons;
        }

        0xC0 | (self.selection & 0x30) | (!buttons & 0x0F)
    }

    pub fn write(&mut self, value: u8) {
        self.selection = value & 0x30;
    }
}
