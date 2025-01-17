use crate::input_output_registers::InputOutputRegisters;
use crate::memory_device::ReadWrite;

/// InternalMemory holds all memory banks for internal handling of the emulating job, not GPU or
/// cartridge related, just internal stuff to read and write during execution.
pub struct InternalMemory {
    /// working ram bank 0: 0xC000-0xCFFF
    wram0: [u8; 0x1FFF + 1],
    /// working ram bank n: 0xD000-0xDFFF
    wramn: [u8; 0x1FFF + 1],
    /// high ram (zero-page): 0xFF80-0xFFFE
    hram: [u8; 0x1FFF + 1],
    /// interrupt flag (request) register: 0xFF0F
    interrupt_flag: u8,
    /// interrupt flag enable: 0xFFFF
    interrupt_enable: u8,
    /// I/O registers.
    io_reg: InputOutputRegisters,
}

impl InternalMemory {
    pub fn new() -> InternalMemory {
        InternalMemory {
            wram0: [0; 0x1FFF + 1],
            wramn: [0; 0x1FFF + 1],
            hram: [0; 0x1FFF + 1],
            interrupt_flag: 0,
            interrupt_enable: 0,
            io_reg: InputOutputRegisters::new(),
        }
    }
}

impl ReadWrite for InternalMemory {
    fn contains(self: &Self, address: usize) -> bool {
        (0xC000..=0xCFFF).contains(&address)
            || (0xD000..=0xDFFF).contains(&address)
            || (0xFF80..=0xFFFE).contains(&address)
            || 0xFF0F == address
            || 0xFFFF == address
            || self.io_reg.contains(address)
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        if self.io_reg.contains(address) {
            return self.io_reg.read_byte(address);
        }

        match address {
            0xC000..=0xCFFF => Ok(self.wram0[address - 0xC000]),
            0xD000..=0xDFFF => Ok(self.wramn[address - 0xD000]),
            0xFF80..=0xFFFE => Ok(self.hram[address - 0xFF80]),
            0xFF0F => Ok(self.interrupt_flag),
            0xFFFF => Ok(self.interrupt_enable),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't read byte here",
            )),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        if self.io_reg.contains(address) {
            return self.io_reg.read_word(address);
        }

        let values = match address {
            0xC000..=0xCFFF => [
                self.wram0[address - 0xC000],
                self.wram0[address - 0xC000 + 1],
            ],
            0xD000..=0xDFFF => [
                self.wramn[address - 0xD000],
                self.wramn[address - 0xD000 + 1],
            ],
            0xFF80..=0xFFFE => [self.hram[address - 0xFF80], self.hram[address - 0xFF80 + 1]],
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "can't read byte here",
                ))
            }
        };

        Ok(u16::from_le_bytes(values))
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xC000..=0xCFFF => Ok(self.wram0[address - 0xC000] = value),
            0xD000..=0xDFFF => Ok(self.wramn[address - 0xD000] = value),
            0xFF80..=0xFFFE => Ok(self.hram[address - 0xFF80] = value),
            0xFF0F => Ok(self.interrupt_flag = value),
            0xFFFF => Ok(self.interrupt_enable = value),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
        todo!("implement this func")
    }
}
