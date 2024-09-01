// 0nnn - SYS addr
// no op for emulators
pub fn sys_addr(_ram: &mut [u8; 0x1000], _addr: u16) {
    return;
}