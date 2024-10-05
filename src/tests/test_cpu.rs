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

#[test]
fn ldx() {
    // arrange
    let mut emulator = Emulator::default();

    // act
    let instruction = 0x7450;

    // assert
    crate::cpu::ldx(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0x50);
}

#[test]
fn addx() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0x1;

    // act
    let instruction = 0x7450;

    // assert
    crate::cpu::addx(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0x51);
}

#[test]
fn ldxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[5] = 0x2;

    // act
    let instruction = 0x8450;

    // assert
    crate::cpu::ldxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], emulator.v[5]);
}

#[test]
fn orxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0x8;
    emulator.v[5] = 0x4;

    // act
    let instruction = 0x8451;

    // assert
    crate::cpu::orxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0xC);
}

#[test]
fn andxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0xC;
    emulator.v[5] = 0x4;

    // act
    let instruction = 0x8452;

    // assert
    crate::cpu::andxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0x4);
}

#[test]
fn xorxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0xC;
    emulator.v[5] = 0x4;

    // act
    let instruction = 0x8453;

    // assert
    crate::cpu::xorxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0x8);
}

#[test]
fn addxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0xFF;
    emulator.v[5] = 0x3;

    // act
    let instruction = 0x8454;

    // assert
    crate::cpu::addxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0x2);
    assert_eq!(emulator.v[0xF], 0x1);
}

#[test]
fn subxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0xFF;
    emulator.v[5] = 0x3;

    // act
    let instruction = 0x8455;

    // assert
    crate::cpu::subxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0xFC);
    assert_eq!(emulator.v[0xF], 0x1);
}


#[test]
fn shrxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0xFF;

    // act
    let instruction = 0x8456;

    // assert
    crate::cpu::shrxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0x7F);
    assert_eq!(emulator.v[0xF], 0x1);
}

#[test]
fn subnxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0x3;
    emulator.v[5] = 0xFF;

    // act
    let instruction = 0x8457;

    // assert
    crate::cpu::subnxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0xFC);
    assert_eq!(emulator.v[0xF], 0x1);
}


#[test]
fn shlxy() {
    // arrange
    let mut emulator = Emulator::default();
    emulator.v[4] = 0xF0;

    // act
    let instruction = 0x845E;

    // assert
    crate::cpu::shlxy(&mut emulator, instruction);
    assert_eq!(emulator.v[4], 0xE0);
    assert_eq!(emulator.v[0xF], 0x1);
}
