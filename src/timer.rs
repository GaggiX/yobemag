use crate::memory_device::ReadWrite;

pub struct Timer {
    /// FF07: Timer Control (TAC)
    ///  - Bit 2: Timer Stop
    ///    - 0: Stop
    ///    - 1: Start
    ///  - Bits 1-0: Input Clock Select (TIMA rate)
    ///    - 00: 4096 Hz
    ///    - 01: 262144 Hz
    ///    - 10: 65536 Hz
    ///    - 11: 16384 Hz
    tac: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { tac: 0 }
    }
}

impl ReadWrite for Timer {
    fn contains(self: &Self, address: usize) -> bool {
        0xFF07 == address
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF07 => Ok(self.tac),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn read_word(self: &Self, _address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xFF07 => Ok(self.tac = value),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn write_word(self: &mut Self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
