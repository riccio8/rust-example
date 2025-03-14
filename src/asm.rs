use std::arch::asm;


fn main() {
    unsafe {
        asm!("nop"); // Assembly inline
    }
}
