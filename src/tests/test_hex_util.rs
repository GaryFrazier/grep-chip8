#[test]
fn get_nth_nibble() {
    // arrange
    let instruction = 0x6543;

    // act
    let invalid = crate::hex_util::get_nth_nibble(instruction, 0);
    let first = crate::hex_util::get_nth_nibble(instruction, 1);
    let second = crate::hex_util::get_nth_nibble(instruction, 2);
    let third = crate::hex_util::get_nth_nibble(instruction, 3);
    let fourth = crate::hex_util::get_nth_nibble(instruction, 4);

    // assert
    assert_eq!(invalid, 0);
    assert_eq!(first, 3);
    assert_eq!(second, 4);
    assert_eq!(third, 5);
    assert_eq!(fourth, 6);
}