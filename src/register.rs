pub struct Registers {
    pub a: u8,
    pub flags: CpuFlag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub program_counter: i32,
    pub stack_pointer: u16,
}

pub struct CpuFlag {
    pub carry: bool,
    pub half_carry: bool,
    pub negative: bool,
    pub zero: bool,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            flags: CpuFlag {
                carry: false,
                half_carry: false,
                negative: false,
                zero: false,
            },
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            program_counter: 0x0100,
            stack_pointer: 0xFFFE,
        }
    }

    pub fn set_bc(self: &mut Self, value: u16) {
        self.b = (value >> 8 as u16) as u8;
        self.c = value as u8;
    }

    pub fn bc(self: &Self) -> u16 {
        let ret = (self.b as u16) << 8;
        ret | self.c as u16
    }

    pub fn set_de(self: &mut Self, value: u16) {
        self.d = (value >> 8 as u16) as u8;
        self.e = value as u8;
    }

    pub fn de(self: &Self) -> u16 {
        let ret = (self.d as u16) << 8;
        ret | self.e as u16
    }

    pub fn set_hl(self: &mut Self, value: u16) {
        self.h = (value >> 8 as u16) as u8;
        self.l = value as u8;
    }

    pub fn hl(self: &Self) -> u16 {
        let ret = (self.h as u16) << 8;
        ret | self.l as u16
    }
}
