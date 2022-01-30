use core::arch::global_asm;

global_asm!(include_str!("asm/asm_utils.s"));

extern {
    fn asm_delay(t: isize);
    fn asm_get_el() -> u32;
}

/// This will call an assembler function to delay a number steps (CPU clock steps, not ms).
pub fn delay(time: isize) {
  unsafe {
    asm_delay(time);
  }
}

/// Get the current exception level of the processor
pub fn get_exception_level() -> u32 {
    unsafe {
        return asm_get_el();
    }
}
