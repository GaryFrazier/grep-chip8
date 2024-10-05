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
        _ if (instruction & 0x1000 == 0x1000) => jp(emulator, instruction), // 1nnn
        _ if (instruction & 0x2000 == 0x2000) => call(emulator, instruction), // 2nnn
        _ if (instruction & 0x3000 == 0x3000) => se(emulator, instruction), // 3xkk
        _ if (instruction & 0x4000 == 0x4000) => sne(emulator, instruction), // 4xkk
        _ if (instruction & 0x5000 == 0x5000) => sev(emulator, instruction), // 5xy0
        _ if (instruction & 0x6000 == 0x6000) => ldx(emulator, instruction), // 6xkk
        _ if (instruction & 0x7000 == 0x7000) => addx(emulator, instruction), // 7xkk
        _ if (instruction & 0x8000 == 0x8000) => ldxy(emulator, instruction), // 8xy0
        _ if (instruction & 0x8001 == 0x8001) => orxy(emulator, instruction), // 8xy1
        _ if (instruction & 0x8002 == 0x8002) => andxy(emulator, instruction), // 8xy2
        _ if (instruction & 0x8003 == 0x8003) => xorxy(emulator, instruction), // 8xy3
        _ if (instruction & 0x8004 == 0x8004) => addxy(emulator, instruction), // 8xy4
        _ if (instruction & 0x8005 == 0x8005) => subxy(emulator, instruction), // 8xy5
        _ if (instruction & 0x8006 == 0x8006) => shrxy(emulator, instruction), // 8xy6
        _ if (instruction & 0x8007 == 0x8007) => subnxy(emulator, instruction), // 8xy7
        _ if (instruction & 0x800E == 0x800E) => shlxy(emulator, instruction), // 8xyE
        _ if (instruction & 0x9000 == 0x9000) => snexy(emulator, instruction), // 9xy0
        _ if (instruction & 0xA000 == 0xA000) => ldi(emulator, instruction), // Annn
        _ if (instruction & 0xB000 == 0xB000) => jpv(emulator, instruction), // Bnnn
        _ if (instruction & 0xC000 == 0xC000) => rnd(emulator, instruction), // Cxkk
        _ if (instruction & 0xD000 == 0xD000) => drw(emulator, instruction), // Dxyn
        _ if (instruction & 0xE09E == 0xE09E) => skp(emulator, instruction), // Ex9E
        _ if (instruction & 0xE0A1 == 0xE0A1) => sknp(emulator, instruction), // ExA1
        _ if (instruction & 0xF007 == 0xF007) => ldxdt(emulator, instruction), // Fx07
        _ if (instruction & 0xF00A == 0xF00A) => ldk(emulator, instruction), // Fx0A
        _ if (instruction & 0xF015 == 0xF015) => lddt(emulator, instruction), // Fx15
        _ if (instruction & 0xF018 == 0xF018) => ldst(emulator, instruction), // Fx18
        _ if (instruction & 0xF01E == 0xF01E) => addi(emulator, instruction), // Fx1E
        _ if (instruction & 0xF029 == 0xF029) => ldiv(emulator, instruction), // Fx29
        _ if (instruction & 0xF033 == 0xF033) => ldb(emulator, instruction), // Fx33
        _ if (instruction & 0xF055 == 0xF055) => ldii(emulator, instruction), // Fx55
        _ if (instruction & 0xF065 == 0xF065) => ldvi(emulator, instruction), // Fx65
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

// 6xkk - LD Vx, byte
// Set Vx = kk
// The interpreter puts the value kk into register Vx.
pub fn ldx(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    emulator.v[x] = (instruction & 0xFF) as u8;
}

// 7xkk - ADD Vx, byte
// Set Vx = Vx + kk.
// Adds the value kk to the value of register Vx, then stores the result in Vx. 
pub fn addx(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    emulator.v[x]+= (instruction & 0xFF) as u8;
}

// 8xy0 - LD Vx, Vy
// Set Vx = Vy.
// Stores the value of register Vy in register Vx.
pub fn ldxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);
    emulator.v[x] = emulator.v[y];
}

// 8xy1 - OR Vx, Vy
// Set Vx = Vx OR Vy.
// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
pub fn orxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);
    emulator.v[x] = emulator.v[x] | emulator.v[y];
}

// 8xy2 - AND Vx, Vy
// Set Vx = Vx AND Vy.
// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
pub fn andxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);
    emulator.v[x] = emulator.v[x] & emulator.v[y];
}

// 8xy3 - XOR Vx, Vy
// Set Vx = Vx XOR Vy.
// Performs a bitwise XOR on the values of Vx and Vy, then stores the result in Vx.
pub fn xorxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);
    emulator.v[x] = emulator.v[x] ^ emulator.v[y];
}

// 8xy4 - ADD Vx, Vy
// Set Vx = Vx + Vy, set VF = carry.
// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) 
// VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
pub fn addxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);
    let total: u16 = (emulator.v[x] as u16) + (emulator.v[y] as u16);

    emulator.v[0xF] = if total > 0xFF { 1 } else { 0 };
    emulator.v[x] = (total & 0xFF) as u8;
}

// 8xy5 - SUB Vx, Vy
// Set Vx = Vx - Vy, set VF = NOT borrow.
// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
pub fn subxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);

    emulator.v[0xF] = if emulator.v[x] > emulator.v[y] { 1 } else { 0 };
    emulator.v[x] = emulator.v[x].wrapping_sub(emulator.v[y]);
}

// 8xy6 - SHR Vx {, Vy}
// Set Vx = Vx SHR 1.
// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
pub fn shrxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);

    emulator.v[0xF] = emulator.v[x] & 0x1;
    emulator.v[x] = emulator.v[x] / 2;
}

// 8xy7 - SUBN Vx, Vy
// Set Vx = Vy - Vx, set VF = NOT borrow.
// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
pub fn subnxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);

    emulator.v[0xF] = if emulator.v[y] > emulator.v[x] { 1 } else { 0 };
    emulator.v[x] = emulator.v[y].wrapping_sub(emulator.v[x]);
}

// 8xyE - SHL Vx {, Vy}
// Set Vx = Vx SHL 1.
// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
pub fn shlxy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);

    emulator.v[0xF] = if  emulator.v[x] & 0x80 == 0x80 { 1 } else { 0 };
    emulator.v[x] = emulator.v[x].wrapping_mul(2);
}

// 9xy0 - SNE Vx, Vy
// Skip next instruction if Vx != Vy.
// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
pub fn snexy(emulator: &mut Emulator, instruction: u16) {
    let x = hex_util::get_nth_nibble(instruction, 3);
    let y = hex_util::get_nth_nibble(instruction, 2);

    if emulator.v[x] != emulator.v[y] {
        emulator.pc += 2;
    }
}

// DELETE
// _ if (instruction & 0xA000 == 0xA000) => ldi(emulator, instruction), // Annn
// _ if (instruction & 0xB000 == 0xB000) => jpv(emulator, instruction), // Bnnn
// _ if (instruction & 0xC000 == 0xC000) => rnd(emulator, instruction), // Cxkk
// _ if (instruction & 0xD000 == 0xD000) => drw(emulator, instruction), // Dxyn
// _ if (instruction & 0xE09E == 0xE09E) => skp(emulator, instruction), // Ex9E
// _ if (instruction & 0xE0A1 == 0xE0A1) => sknp(emulator, instruction), // ExA1
// _ if (instruction & 0xF007 == 0xF007) => ldxdt(emulator, instruction), // Fx07
// _ if (instruction & 0xF00A == 0xF00A) => ldk(emulator, instruction), // Fx0A
// _ if (instruction & 0xF015 == 0xF015) => lddt(emulator, instruction), // Fx15
// _ if (instruction & 0xF018 == 0xF018) => ldst(emulator, instruction), // Fx18
// _ if (instruction & 0xF01E == 0xF01E) => addi(emulator, instruction), // Fx1E
// _ if (instruction & 0xF029 == 0xF029) => ldiv(emulator, instruction), // Fx29
// _ if (instruction & 0xF033 == 0xF033) => ldb(emulator, instruction), // Fx33
// _ if (instruction & 0xF055 == 0xF055) => ldii(emulator, instruction), // Fx55
// _ if (instruction & 0xF065 == 0xF065) => ldvi(emulator, instruction), // Fx65

// Annn - LD I, addr
// Set I = nnn.
// The value of register I is set to nnn.
pub fn ldi(emulator: &mut Emulator, instruction: u16) {
    
}

// Bnnn - JP V0, addr
// Jump to location nnn + V0.
// The program counter is set to nnn plus the value of V0.
pub fn jpv(emulator: &mut Emulator, instruction: u16) {
    
}

// Cxkk - RND Vx, byte
// Set Vx = random byte AND kk.
// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. 
// The results are stored in Vx. See instruction 8xy2 for more information on AND.
pub fn rnd(emulator: &mut Emulator, instruction: u16) {
    
}

// Dxyn - DRW Vx, Vy, nibbler
// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
/* 
he interpreter reads n bytes from memory, starting at the address stored in I. 
These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). 
Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. 
If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
 See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
*/
pub fn drw(emulator: &mut Emulator, instruction: u16) {
    
}

// Ex9E - SKP Vx
// Skip next instruction if key with the value of Vx is pressed.
// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
pub fn skp(emulator: &mut Emulator, instruction: u16) {
    
}

// ExA1 - SKNP Vx
// Skip next instruction if key with the value of Vx is not pressed.
// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
pub fn sknp(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx07 - LD Vx, DT
// Set Vx = delay timer value.
// The value of DT is placed into Vx.
pub fn ldxdt(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx0A - LD Vx, K
// Wait for a key press, store the value of the key in Vx.
// All execution stops until a key is pressed, then the value of that key is stored in Vx.
pub fn ldk(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx15 - LD DT, Vx
// Set delay timer = Vx.
// DT is set equal to the value of Vx.
pub fn lddt(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx18 - LD ST, Vx
// Set sound timer = Vx.
// ST is set equal to the value of Vx.
pub fn ldst(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx1E - ADD I, Vx
// Set I = I + Vx.
// The values of I and Vx are added, and the results are stored in I.
pub fn addi(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx29 - LD F, Vx
// Set I = location of sprite for digit Vx.
// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. 
// See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
pub fn ldiv(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx33 - LD B, Vx
// Store BCD representation of Vx in memory locations I, I+1, and I+2.
// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
// the tens digit at location I+1, and the ones digit at location I+2.
pub fn ldb(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx55 - LD [I], Vx
// Store registers V0 through Vx in memory starting at location I.
// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
pub fn ldii(emulator: &mut Emulator, instruction: u16) {
    
}

// Fx65 - LD Vx, [I]
// Read registers V0 through Vx from memory starting at location I.
// The interpreter reads values from memory starting at location I into registers V0 through Vx.
pub fn ldvi(emulator: &mut Emulator, instruction: u16) {
    
}
