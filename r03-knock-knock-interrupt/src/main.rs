#![no_std]
#![no_main]
#![feature(global_asm)]
use core::format_args;

global_asm!(include_str!("asm/start.s"));

mod panic;
mod periferals;
mod tools;

#[no_mangle]
pub extern "C" fn rmain() {

    let uart = periferals::uart1::Uart1::new();
    let mut buf = [0u8; 64];

    let output1 = tools::format_to::show(&mut buf,
                                        format_args!("Welcome to Raspberry Pi{}\n", 3)).unwrap();
    uart.puts(output1);
    let output2 = tools::format_to::show(&mut buf,
                                         format_args!("Exception Level: {}\n",
                                                      periferals::utils::get_exception_level())).unwrap();
    uart.puts(output2);

    loop {
        uart.putc(uart.getc());
    }
}