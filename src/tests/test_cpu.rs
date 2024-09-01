#[test]
fn sys_addr() {
    // arrange
    let mut test_ram: [u8; 0x1000] = [0; 0x1000];

    // act
    crate::cpu::sys_addr(&mut test_ram, 0);

    // assert
    assert!(true)
}