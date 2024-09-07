#[derive(Debug)]
pub struct Emulator {
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 0xF], 
    pub ram: [u8; 0x1000],
    pub display_memory: [bool; 0x800]
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            stack: [0; 0xF],
            ram: [0; 0x1000],
            display_memory: [false; 0x800]
        }
    }
}