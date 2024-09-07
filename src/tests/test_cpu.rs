use crate::emulator::Emulator;

#[test]
fn get_next_instruction() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.pc = 2;
    emulator.ram = [0; 0x1000];

    emulator.ram[2] = 0xAB;
    emulator.ram[3] = 0xCD;

    let expected_pc: u16 = 4; // incremented by 2
    let expected_instruction: u16 = 0xABCD;

    // act
    let next_instruction = crate::cpu::get_next_instruction(&mut emulator);

    // assert
    assert_eq!(expected_pc, emulator.pc);
    assert_eq!(expected_instruction, next_instruction);
}

////////////////////////////// INSTRUCTIONS ////////////////////////////////////////

#[test]
fn sys() {
    // act
    crate::cpu::sys();

    // assert
    assert!(true)
}

#[test]
fn cls() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.display_memory = [true; 0x800];

    let expected_display_mem: [bool; 0x800] = [false; 0x800];

    // act
    crate::cpu::cls(&mut emulator);

    // assert
    assert!(emulator.display_memory.iter().eq(expected_display_mem.iter()));
}

#[test]
fn ret() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.sp = 5;
    emulator.stack = [0xFE;0xF];
    emulator.pc = 0;

    let expected_pc: u16 = 0xFE;
    let expected_sp: u8 = 4;

    // act
    crate::cpu::ret(&mut emulator);

    // assert
    assert_eq!(expected_pc, emulator.pc);
    assert_eq!(expected_sp, emulator.sp);
}