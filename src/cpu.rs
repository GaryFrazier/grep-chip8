pub fn execute_next_instruction(ram: &mut [u8; 0x1000] ,pc: &mut u16) {
    let instruction: u16 = get_next_instruction(ram, pc);
    
    // get next
}

// gets next instruction from memory and increments pc
pub fn get_next_instruction(ram: &mut [u8; 0x1000] ,pc: &mut u16) -> u16 {
    // all instructions are 2 bytes, msb first
    let instruction: u16 = ((ram[*pc as usize] as u16) << 8) + ram[*pc as usize + 1] as u16;
    *pc += 2;
    return instruction;
}
////////////////////////////////// INSTRUCTIONS ///////////////////////////////////////////////

// 0nnn - SYS addr
// no op for emulators TODO delete params
pub fn sys() {
    return;
}

// 00E0 - CLS
// clears the display
pub fn cls(display_memory: &mut [bool; 0x800]) {
    crate::display::clear_display(display_memory);
}

// 00EE - RET
// Return from a subroutine.
// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
pub fn ret(stack: &[u16; 0xF], sp: &mut u8, pc: &mut u16) {
    *pc = stack[*sp as usize];
    *sp = *sp - 1;
}