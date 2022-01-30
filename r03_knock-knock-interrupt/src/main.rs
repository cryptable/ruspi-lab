#![no_std]
#![no_main]
use core::format_args;
use core::arch::global_asm;

global_asm!(include_str!("asm/start.s"));

mod panic;
mod periferals;
mod tools;

#[no_mangle]
pub extern "C" fn rmain() {

    periferals::uart1::init_uart1();

    let mut buf = [0u8; 64];
    let output1 = tools::format_to::show(&mut buf,
                                         format_args!("Welcome to Raspberry Pi {}\n", 3)).unwrap();
    periferals::uart1::puts(output1);

    unsafe {
        periferals::interrupts::irq_init_vectors();
        periferals::interrupts::enable_interrupt_controller(periferals::interrupts::AUX_IRQ);
        periferals::interrupts::irq_enable();
    }

    let output2 = tools::format_to::show(&mut buf,
                                         format_args!("Exception Level: {}\n",
                                                      periferals::utils::get_exception_level())).unwrap();
    periferals::uart1::puts(output2);

    loop {
//        periferals::uart1::putc(periferals::uart1::getc());
    }
}