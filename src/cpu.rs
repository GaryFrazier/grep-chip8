use crate::emulator::Emulator;
use crate::hex_util;

pub fn execute_next_instruction(emulator: &mut Emulator) {
    let instruction: u16 = get_next_instruction(emulator);
    call_instruction(emulator, instruction);
}

// gets next instruction from memory and increments pc
pub fn get_next_instruction(emulator: &mut Emulator) -> u16 {
    // all instructions are 2 bytes, msb first
    let instruction: u16 = ((emulator.ram[emulator.pc as usize] as u16) << 8) + emulator.ram[emulator.pc as usize + 1] as u16;
    emulator.pc += 2;
    return instruction;
}

// finds instruction based on the hex, then executes mapped function
pub fn call_instruction (emulator: &mut Emulator, instruction: u16) {
    match instruction {
        0x00E0 => cls(emulator),
        0x00EE => ret(emulator),
        _ if (instruction < 0x1000) => sys(), // 0nnn
        _ if (instruction >= 0x1000 && instruction < 0x2000) => jp(emulator, instruction), // 1nnn
        _ if (instruction >= 0x2000 && instruction < 0x3000) => call(emulator, instruction), // 2nnn
        _ if (instruction >= 0x3000 && instruction < 0x4000) => se(emulator, instruction), // 3xkk
        _ if (instruction >= 0x4000 && instruction < 0x5000) => sne(emulator, instruction), // 4xkk
        _ if (instruction >= 0x5000 && instruction < 0x6000) => sev(emulator, instruction), // 5xy0
        _ => {
            eprintln!("Error! Instruction not supported, please contact developer. Instruction code: {:#?}", instruction);
            std::process::exit(1);
        }
    }
}

////////////////////////////////// INSTRUCTIONS ///////////////////////////////////////////////

// 0nnn - SYS addr
// no op for emulators
pub fn sys() {
    return;
}

// 00E0 - CLS
// clears the display
pub fn cls(emulator: &mut Emulator) {
    crate::display::clear_display(&mut emulator.display_memory);
}

// 00EE - RET
// Return from a subroutine.
// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
pub fn ret(emulator: &mut Emulator) {
    emulator.pc = emulator.stack[emulator.sp as usize];
    emulator.sp -= 1;
}

// 1nnn - JP addr
// Jump to location nnn.
// The interpreter sets the program counter to nnn.
pub fn jp(emulator: &mut Emulator, instruction: u16) {
    emulator.pc = instruction & 0x0FFF;
}

// 2nnn - CALL addr
// Call subroutine at nnn.
// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
pub fn call(emulator: &mut Emulator, instruction: u16) {
    emulator.sp += 1;
    emulator.stack[emulator.sp as usize] = emulator.pc;
    emulator.pc = instruction & 0x0FFF;
}

// 3xkk - SE Vx, byte
// Skip next instruction if Vx = kk.
// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
pub fn se(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    if emulator.v[x] == ((instruction & 0xFF) as u8) {
        emulator.pc += 2;
    }
}

// 4xkk - SNE Vx, byte
// Skip next instruction if Vx != kk.
// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
pub fn sne(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    if emulator.v[x] != ((instruction & 0xFF) as u8) {
        emulator.pc += 2;
    }
}

// 5xy0 - SE Vx, Vy
// Skip next instruction if Vx = Vy.
// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
pub fn sev(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);
    if emulator.v[x] == emulator.v[y] {
        emulator.pc += 2;
    }
}