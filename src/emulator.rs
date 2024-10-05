#[derive(Debug)]
pub struct Emulator {
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 0x10], 
    pub ram: [u8; 0x1000],
    pub display_memory: [bool; 0x800],
    pub v: [u8; 0x10],
    pub i: u16,

    //	When these registers are non-zero, they are automatically decremented at a rate of 60Hz. See the section 2.5, Timers & Sound, for more information on these.
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            pc: 0,
            sp: 0,
            stack: [0; 0x10],
            ram: [0; 0x1000],
            display_memory: [false; 0x800],
            v: [0; 0x10],
            i: 0,
            delay_timer:0,
            sound_timer: 0,
        }
    }
}