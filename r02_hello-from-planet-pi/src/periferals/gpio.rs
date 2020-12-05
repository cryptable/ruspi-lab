use core::ptr;
use crate::periferals::utils;
use crate::periferals::memmap::MMIO_BASE;

pub const GPFSEL0: *mut u32   = (MMIO_BASE + 0x00200000) as *mut u32;
pub const GPFSEL1: *mut u32   = (MMIO_BASE + 0x00200004) as *mut u32;
pub const GPFSEL2: *mut u32   = (MMIO_BASE + 0x00200008) as *mut u32;
pub const GPFSEL3: *mut u32   = (MMIO_BASE + 0x0020000C) as *mut u32;
pub const GPFSEL4: *mut u32   = (MMIO_BASE + 0x00200010) as *mut u32;
pub const GPFSEL5: *mut u32   = (MMIO_BASE + 0x00200014) as *mut u32;
pub const GPSET0: *mut u32    = (MMIO_BASE + 0x0020001C) as *mut u32;
pub const GPSET1: *mut u32    = (MMIO_BASE + 0x00200020) as *mut u32;
pub const GPCLR0: *mut u32    = (MMIO_BASE + 0x00200028) as *mut u32;
pub const GPCLR1: *mut u32    = (MMIO_BASE + 0x0020002c) as *mut u32;
pub const GPLEV0: *mut u32    = (MMIO_BASE + 0x00200034) as *mut u32;
pub const GPLEV1: *mut u32    = (MMIO_BASE + 0x00200038) as *mut u32;
pub const GPEDS0: *mut u32    = (MMIO_BASE + 0x00200040) as *mut u32;
pub const GPEDS1: *mut u32    = (MMIO_BASE + 0x00200044) as *mut u32;
pub const GPHEN0: *mut u32    = (MMIO_BASE + 0x00200064) as *mut u32;
pub const GPHEN1: *mut u32    = (MMIO_BASE + 0x00200068) as *mut u32;
pub const GPPUD: *mut u32     = (MMIO_BASE + 0x00200094) as *mut u32;
pub const GPPUDCLK0: *mut u32 = (MMIO_BASE + 0x00200098) as *mut u32;
pub const GPPUDCLK1: *mut u32 = (MMIO_BASE + 0x0020009C) as *mut u32;

pub fn gpio_set_fun(pin: u8, func: u8) {
 
  if pin >= 28 {
    // Error
    return;
  }
  if func >= 8 {
    // Error
    return;
  }

  let shift = (pin % 10) * 3;
  if pin < 10 {
    // we clear the 3 bits using !(7<<shift) for the function selection
    // we set the pin to output using (1<<shift)
    unsafe {
      let r = (ptr::read_volatile(GPFSEL0) & !((7 as u32)<<shift)) | ((func as u32)<<shift);
      ptr::write_volatile(GPFSEL0, r);
    }
    return;
  }

  if pin < 20 {
    unsafe {
      let r = (ptr::read_volatile(GPFSEL1) & !((7 as u32)<<shift)) | ((func as u32)<<shift);
      ptr::write_volatile(GPFSEL1, r);    
    }
    return;
  }

  unsafe {
    let r = (ptr::read_volatile(GPFSEL2) & !((7 as u32)<<shift)) | ((func as u32)<<shift);
    ptr::write_volatile(GPFSEL2, r);
  }

  // Success
  return

}

pub fn gpio_set_output(pin: u8, high: bool) {
 
  if pin >= 28 {
    // Error
    return;
  }

  if high {
    unsafe {
      ptr::write_volatile(GPSET0, 1<<pin);
    }
    return;
  }

  unsafe {
    ptr::write_volatile(GPCLR0, 1<<pin);
  }
  // Success
  return

}

pub fn gpio_set_pullup_pulldown(pin: u8, func: u8) {

  if pin >= 28 {
    // Error
    return;
  }
  if func >= 4 {
    // Error
    return;
  }

  unsafe {
    ptr::write_volatile(GPPUD, func as u32);
  }
  utils::delay(150);
  unsafe {
    ptr::write_volatile(GPPUDCLK0, 1<<pin);
  }
  utils::delay(150);
  unsafe {
    ptr::write_volatile(GPPUD, 0x0);
    ptr::write_volatile(GPPUDCLK0, 0x0);
  }

  // Success
  return
}
