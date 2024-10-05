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
    emulator.stack = [0xFE;0x10];

    let expected_pc: u16 = 0xFE;
    let expected_sp: u8 = 4;

    // act
    crate::cpu::ret(&mut emulator);

    // assert
    assert_eq!(expected_pc, emulator.pc);
    assert_eq!(expected_sp, emulator.sp);
}

#[test]
fn jp() {
    // arrange
    let mut emulator = Emulator::default();

    let expected_pc: u16 = 0x0123;
    let instruction: u16 = 0x1123;

    // act
    crate::cpu::jp(&mut emulator, instruction);

    // assert
    assert_eq!(emulator.pc, expected_pc);
}

#[test]
fn call() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.pc = 0x0456;

    let expected_sp: u8 = 1;
    let expected_stack_value = 0x0456; // current pc pushed onto stack
    let instruction: u16 = 0x2123;
    let expected_pc = 0x0123;

    // act
    crate::cpu::call(&mut emulator, instruction);

    // assert
    assert_eq!(emulator.pc, expected_pc);
    assert_eq!(emulator.sp, expected_sp);
    assert_eq!(emulator.stack[emulator.sp as usize], expected_stack_value);
}

#[test]
fn se() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0x56;

    let eq_instruction = 0x3456;
    let neq_instruction = 0x3457;

    crate::cpu::se(&mut emulator, eq_instruction);
    assert_eq!(emulator.pc, 2);

    // not equal, dont increment
    crate::cpu::se(&mut emulator, neq_instruction);
    assert_eq!(emulator.pc, 2);
}

#[test]
fn sne() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0x56;

    let eq_instruction = 0x4456;
    let neq_instruction = 0x4457;

    crate::cpu::sne(&mut emulator, eq_instruction);
    assert_eq!(emulator.pc, 0);

    // equal, dont increment
    crate::cpu::sne(&mut emulator, neq_instruction);
    assert_eq!(emulator.pc, 2);
}

#[test]
fn sev() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0x56;
    emulator.v[5] = 0x56;

    let eq_instruction = 0x5450;
    let neq_instruction = 0x5460;

    crate::cpu::sev(&mut emulator, eq_instruction);
    assert_eq!(emulator.pc, 2);

    // not equal, dont increment
    crate::cpu::sev(&mut emulator, neq_instruction);
    assert_eq!(emulator.pc, 2);
}