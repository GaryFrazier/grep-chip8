#[test]
fn sys_addr() {
    // arrange
    let expected_display_mem: [bool; 0x800] = [false; 0x800];
    let mut test_display_mem: [bool; 0x800] = [true; 0x800];

    // act
    crate::display::clear_display(&mut test_display_mem);

    // assert
    assert!(test_display_mem.iter().eq(expected_display_mem.iter()));
}