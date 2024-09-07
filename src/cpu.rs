use crate::emulator::Emulator;

pub fn execute_next_instruction(emulator: &mut Emulator) {
    let instruction: u16 = get_next_instruction(emulator);
    call_instruction(instruction, emulator);
}

// gets next instruction from memory and increments pc
pub fn get_next_instruction(emulator: &mut Emulator) -> u16 {
    // all instructions are 2 bytes, msb first
    let instruction: u16 = ((emulator.ram[emulator.pc as usize] as u16) << 8) + emulator.ram[emulator.pc as usize + 1] as u16;
    emulator.pc += 2;
    return instruction;
}

// finds instruction based on the hex, then executes mapped function
pub fn call_instruction (instruction: u16, emulator: &mut Emulator) {
    match instruction {
        0x00E0 => cls(emulator),
        0x00EE => ret(emulator),
        _ if (instruction < 0x1000) => sys(), // 0nnn
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