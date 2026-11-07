pub struct Timer {
    div_counter: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    overflowed: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div_counter: 0xAB00,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
            overflowed: false,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div_counter >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("ERROR. Unknown address: {:04X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => {
                if self.monitored_bit() {
                    self.increment_tima();
                }

                self.div_counter = 0x0000
            }
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => {
                let was_monitored = self.monitored_bit();
                self.tac = value;
                let is_monitored = self.monitored_bit();

                if was_monitored && !is_monitored {
                    self.increment_tima();
                }
            }
            _ => panic!("ERROR. Unknown address: {:04X}", address),
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        if self.overflowed {
            self.tima = self.tma;
            self.overflowed = false;
        }
        let was_monitored = self.monitored_bit();
        self.div_counter = self.div_counter.wrapping_add(u16::from(cycles));
        let is_monitored = self.monitored_bit();

        if was_monitored && !is_monitored {
            self.increment_tima();
        }
    }

    fn monitored_bit(&self) -> bool {
        let enabled = self.tac & 0b100 != 0;
        let bit_position = match self.tac & 0b011 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!(),
        };

        enabled && (self.div_counter >> bit_position) & 1 == 1
    }

    fn increment_tima(&mut self) {
        self.tima = self.tima.wrapping_add(1);

        if self.tima == 0 {
            self.overflowed = true;
            // INTTERRUPTS VAI AQUI
        }
    }
}
