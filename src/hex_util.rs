pub fn get_nth_nibble(instruction: u16, n: usize) -> usize {
    if n > 4 || n <= 0 {
        println!("Invalid attempt to get nibble, returning 0");
        return 0;
    }
    
    // & the instruction to isolate the nibble, then bitshift to get the value of it
    return ((instruction & ((0xF << ((n - 1) * 4))))
        >> ((n - 1) * 4)).into();
}