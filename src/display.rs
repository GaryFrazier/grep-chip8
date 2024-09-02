pub fn clear_display(display_memory: &mut [bool; 0x800]) {
    for i in 0..display_memory.len() {
        display_memory[i] = false;
    }
}