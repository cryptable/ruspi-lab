use core::ptr;
use crate::periferals::gpio;
use crate::periferals::memmap::MMIO_BASE;

/* Auxilary mini UART registers */
const AUX_IRQ: *mut u32         = (MMIO_BASE + 0x00215000) as *mut u32;
const AUX_ENABLES: *mut u32     = (MMIO_BASE + 0x00215004) as *mut u32;
const AUX_MU_IO: *mut u32       = (MMIO_BASE + 0x00215040) as *mut u32;
pub const AUX_MU_IIR: *mut u32      = (MMIO_BASE + 0x00215044) as *mut u32;
const AUX_MU_IER: *mut u32      = (MMIO_BASE + 0x00215048) as *mut u32;
const AUX_MU_LCR: *mut u32      = (MMIO_BASE + 0x0021504C) as *mut u32;
const AUX_MU_MCR: *mut u32      = (MMIO_BASE + 0x00215050) as *mut u32;
const AUX_MU_LSR: *mut u32      = (MMIO_BASE + 0x00215054) as *mut u32;
const AUX_MU_MSR: *mut u32      = (MMIO_BASE + 0x00215058) as *mut u32;
const AUX_MU_SCRATCH: *mut u32  = (MMIO_BASE + 0x0021505C) as *mut u32;
const AUX_MU_CNTL: *mut u32     = (MMIO_BASE + 0x00215060) as *mut u32;
const AUX_MU_STAT: *mut u32     = (MMIO_BASE + 0x00215064) as *mut u32;
const AUX_MU_BAUD: *mut u32     = (MMIO_BASE + 0x00215068) as *mut u32;


/// Creata and Initialize the UART
pub fn init_uart1() {

  gpio::gpio_set_fun(14, 2);
  gpio::gpio_set_fun(15, 2);
  gpio::gpio_set_pullup_pulldown(14, 0);
  gpio::gpio_set_pullup_pulldown(15, 0);

  unsafe {
    ptr::write_volatile(AUX_ENABLES, 0x1);
    ptr::write_volatile(AUX_MU_CNTL, 0x0);
    ptr::write_volatile(AUX_MU_LCR, 0x3);
    ptr::write_volatile(AUX_MU_MCR, 0x0);
    ptr::write_volatile(AUX_MU_IER, 0xd);     // No interrupts
    ptr::write_volatile(AUX_MU_IIR, 0x6);     // Clear FIFO registers
    ptr::write_volatile(AUX_MU_BAUD, 541);    // 270 for Raspberry Pi3 (see calculation in BCM2711) and 541 for a Pi4
    ptr::write_volatile(AUX_MU_CNTL, 0x3);
  }

}

/// Put a character into the UART1
pub fn putc(c: u8) {
  unsafe {
    let mut r = ptr::read_volatile(AUX_MU_LSR) & 0x20; // Transmitter empty?
    while r == 0x00 {
      r = ptr::read_volatile(AUX_MU_LSR) & 0x20;
    }
    ptr::write_volatile(AUX_MU_IO, c as u32);
  }
}

/// Get a character from the UART1
pub fn getc() -> u8 {
  unsafe {
    let mut r = ptr::read_volatile(AUX_MU_LSR) & 0x01;
    while r == 0x00 {
      r = ptr::read_volatile(AUX_MU_LSR) & 0x01;
    }
    let v = ptr::read_volatile(AUX_MU_IO);

    return v as u8;
  }
}

/// Send a string of characters to the UART1
pub fn puts(value: &str) {
  for c in value.chars() {
    if c == '\n' {
      putc('\r' as u8);
    }
    putc(c as u8);
  }
}
