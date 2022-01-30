use core::arch::global_asm;

global_asm!(include_str!("asmutils.s"));

extern {
    fn asm_delay(t: isize);
}

pub fn delay(time: isize) {
  unsafe {
    asm_delay(time);
  }
}