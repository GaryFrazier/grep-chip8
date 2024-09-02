#[test]
fn get_next_instruction() {
    // arrange
    let mut ram: [u8; 0x1000] = [0; 0x1000];
    ram[2] = 0xAB;
    ram[3] = 0xCD;

    let mut pc: u16 = 2;

    let expected_pc: u16 = 4; // incremented by 2
    let expected_instruction: u16 = 0xABCD;

    // act
    let next_instruction = crate::cpu::get_next_instruction(&mut ram, &mut pc);

    // assert
    assert_eq!(expected_pc, pc);
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
    let expected_display_mem: [bool; 0x800] = [false; 0x800];
    let mut test_display_mem: [bool; 0x800] = [true; 0x800];

    // act
    crate::cpu::cls(&mut test_display_mem);

    // assert
    assert!(test_display_mem.iter().eq(expected_display_mem.iter()));
}

#[test]
fn ret() {
    // arrange
    let stack: [u16; 0xF] = [0xFE; 0xF];
    let mut sp: u8 = 5;
    let mut pc: u16 = 0;

    let expected_pc: u16 = 0xFE;
    let expected_sp: u8 = 4;

    // act
    crate::cpu::ret(&stack, &mut sp, &mut pc);

    // assert
    assert_eq!(expected_pc, pc);
    assert_eq!(expected_sp, sp);
}