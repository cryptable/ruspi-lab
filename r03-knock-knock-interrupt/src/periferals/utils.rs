global_asm!(include_str!("asm/asmutils.s"));

extern {
    fn asm_delay(t: isize);
}

/// This will call an assembler function to delay a number steps (CPU clock steps, not ms).
pub fn delay(time: isize) {
  unsafe {
    asm_delay(time);
  }
}