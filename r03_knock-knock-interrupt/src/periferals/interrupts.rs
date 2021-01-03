#![allow(dead_code)]
use core::ptr;
use crate::periferals::memmap::MMIO_BASE;
use crate::periferals::uart1;
use crate::periferals::uart1::AUX_MU_IIR;
use crate::tools::format_to;

global_asm!(include_str!("asm/asm_interrupts.s"));

extern {
    pub fn irq_init_vectors();
    pub fn irq_enable();
    pub fn irq_disable();
}

// Interrupts registers
const TIMER_CR: *mut u32                = (MMIO_BASE + 0x00000000) as *mut u32;
const TIMER_PRESCALER: *mut u32         = (MMIO_BASE + 0x00000008) as *mut u32;
const TIMER_LS32: *mut u32              = (MMIO_BASE + 0x0000001C) as *mut u32;
const TIMER_MS32: *mut u32              = (MMIO_BASE + 0x00000020) as *mut u32;

const INTRPT_VC_ROUT: *mut u32          = (MMIO_BASE + 0x0000000C) as *mut u32;
const INTRPT_PERFMON_SET: *mut u32      = (MMIO_BASE + 0x00000010) as *mut u32;
const INTRPT_PERFMON_CLR: *mut u32      = (MMIO_BASE + 0x00000014) as *mut u32;
const INTRPT_TIMER_C0: *mut u32         = (MMIO_BASE + 0x00000040) as *mut u32;
const INTRPT_TIMER_C1: *mut u32         = (MMIO_BASE + 0x00000044) as *mut u32;
const INTRPT_TIMER_C2: *mut u32         = (MMIO_BASE + 0x00000048) as *mut u32;
const INTRPT_TIMER_C3: *mut u32         = (MMIO_BASE + 0x0000004C) as *mut u32;
const INTRPT_MBOX_C0: *mut u32          = (MMIO_BASE + 0x00000050) as *mut u32;
const INTRPT_MBOX_C1: *mut u32          = (MMIO_BASE + 0x00000054) as *mut u32;
const INTRPT_MBOX_C2: *mut u32          = (MMIO_BASE + 0x00000058) as *mut u32;
const INTRPT_MBOX_C3: *mut u32          = (MMIO_BASE + 0x0000005C) as *mut u32;

const MBOX_MB0_C0_SET: *mut u32         = (MMIO_BASE + 0x00000080) as *mut u32;
const MBOX_MB1_C0_SET: *mut u32         = (MMIO_BASE + 0x00000084) as *mut u32;
const MBOX_MB2_C0_SET: *mut u32         = (MMIO_BASE + 0x00000088) as *mut u32;
const MBOX_MB3_C0_SET: *mut u32         = (MMIO_BASE + 0x0000008C) as *mut u32;
const MBOX_MB0_C1_SET: *mut u32         = (MMIO_BASE + 0x00000090) as *mut u32;
const MBOX_MB1_C1_SET: *mut u32         = (MMIO_BASE + 0x00000094) as *mut u32;
const MBOX_MB2_C1_SET: *mut u32         = (MMIO_BASE + 0x00000098) as *mut u32;
const MBOX_MB3_C1_SET: *mut u32         = (MMIO_BASE + 0x0000009C) as *mut u32;
const MBOX_MB0_C2_SET: *mut u32         = (MMIO_BASE + 0x000000A0) as *mut u32;
const MBOX_MB1_C2_SET: *mut u32         = (MMIO_BASE + 0x000000A4) as *mut u32;
const MBOX_MB2_C2_SET: *mut u32         = (MMIO_BASE + 0x000000A8) as *mut u32;
const MBOX_MB3_C2_SET: *mut u32         = (MMIO_BASE + 0x000000AC) as *mut u32;
const MBOX_MB0_C3_SET: *mut u32         = (MMIO_BASE + 0x000000B0) as *mut u32;
const MBOX_MB1_C3_SET: *mut u32         = (MMIO_BASE + 0x000000B4) as *mut u32;
const MBOX_MB2_C3_SET: *mut u32         = (MMIO_BASE + 0x000000B8) as *mut u32;
const MBOX_MB3_C3_SET: *mut u32         = (MMIO_BASE + 0x000000BC) as *mut u32;
const MBOX_MB0_C0_CLR: *mut u32         = (MMIO_BASE + 0x000000C0) as *mut u32;
const MBOX_MB1_C0_CLR: *mut u32         = (MMIO_BASE + 0x000000C4) as *mut u32;
const MBOX_MB2_C0_CLR: *mut u32         = (MMIO_BASE + 0x000000C8) as *mut u32;
const MBOX_MB3_C0_CLR: *mut u32         = (MMIO_BASE + 0x000000CC) as *mut u32;
const MBOX_MB0_C1_CLR: *mut u32         = (MMIO_BASE + 0x000000D0) as *mut u32;
const MBOX_MB1_C1_CLR: *mut u32         = (MMIO_BASE + 0x000000D4) as *mut u32;
const MBOX_MB2_C1_CLR: *mut u32         = (MMIO_BASE + 0x000000D8) as *mut u32;
const MBOX_MB3_C1_CLR: *mut u32         = (MMIO_BASE + 0x000000DC) as *mut u32;
const MBOX_MB0_C2_CLR: *mut u32         = (MMIO_BASE + 0x000000E0) as *mut u32;
const MBOX_MB1_C2_CLR: *mut u32         = (MMIO_BASE + 0x000000E4) as *mut u32;
const MBOX_MB2_C2_CLR: *mut u32         = (MMIO_BASE + 0x000000E8) as *mut u32;
const MBOX_MB3_C2_CLR: *mut u32         = (MMIO_BASE + 0x000000EC) as *mut u32;
const MBOX_MB0_C3_CLR: *mut u32         = (MMIO_BASE + 0x000000F0) as *mut u32;
const MBOX_MB1_C3_CLR: *mut u32         = (MMIO_BASE + 0x000000F4) as *mut u32;
const MBOX_MB2_C3_CLR: *mut u32         = (MMIO_BASE + 0x000000F8) as *mut u32;
const MBOX_MB3_C3_CLR: *mut u32         = (MMIO_BASE + 0x000000FC) as *mut u32;

const AXI_OUTSTAND_CNTR: *mut u32       = (MMIO_BASE + 0x0000002C) as *mut u32;
const INTRPT_AXI_OUTSTAND: *mut u32     = (MMIO_BASE + 0x00000030) as *mut u32;

const INTRPT_IRQ_CORE_C0: *mut u32      = (MMIO_BASE + 0x00000060) as *mut u32;
const INTRPT_IRQ_CORE_C1: *mut u32      = (MMIO_BASE + 0x00000064) as *mut u32;
const INTRPT_IRQ_CORE_C2: *mut u32      = (MMIO_BASE + 0x00000068) as *mut u32;
const INTRPT_IRQ_CORE_C3: *mut u32      = (MMIO_BASE + 0x0000006C) as *mut u32;
const INTRPT_FIQ_CORE_C0: *mut u32      = (MMIO_BASE + 0x00000070) as *mut u32;
const INTRPT_FIQ_CORE_C1: *mut u32      = (MMIO_BASE + 0x00000074) as *mut u32;
const INTRPT_FIQ_CORE_C2: *mut u32      = (MMIO_BASE + 0x00000078) as *mut u32;
const INTRPT_FIQ_CORE_C3: *mut u32      = (MMIO_BASE + 0x0000007C) as *mut u32;

const TIMER_LOCAL_CS: *mut u32          = (MMIO_BASE + 0x00000034) as *mut u32;
const TIMER_LOCAL_IRQ_CLR: *mut u32     = (MMIO_BASE + 0x00000038) as *mut u32;
const TIMER_LOCAL_IRQ_ROUT: *mut u32    = (MMIO_BASE + 0x00000024) as *mut u32;

// ARMC Registers
#[cfg(feature="raspi4")]
mod registers {
    use crate::periferals::memmap::MMIO_BASE;

    pub const IRQ0_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b200) as *mut u32;
    pub const IRQ0_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b204) as *mut u32;
    pub const IRQ0_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b208) as *mut u32;
    pub const IRQ0_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b210) as *mut u32;
    pub const IRQ0_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b214) as *mut u32;
    pub const IRQ0_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b218) as *mut u32;
    pub const IRQ0_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b220) as *mut u32;
    pub const IRQ0_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b224) as *mut u32;
    pub const IRQ0_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b228) as *mut u32;
    pub const IRQ0_STATUS0: *mut u32                 = (MMIO_BASE + 0x0000b230) as *mut u32;
    pub const IRQ0_STATUS1: *mut u32                 = (MMIO_BASE + 0x0000b234) as *mut u32;
    pub const IRQ0_STATUS2: *mut u32                 = (MMIO_BASE + 0x0000b238) as *mut u32;
    pub const IRQ1_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b240) as *mut u32;
    pub const IRQ1_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b244) as *mut u32;
    pub const IRQ1_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b248) as *mut u32;
    pub const IRQ1_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b250) as *mut u32;
    pub const IRQ1_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b254) as *mut u32;
    pub const IRQ1_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b258) as *mut u32;
    pub const IRQ1_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b260) as *mut u32;
    pub const IRQ1_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b264) as *mut u32;
    pub const IRQ1_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b268) as *mut u32;
    pub const IRQ2_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b280) as *mut u32;
    pub const IRQ2_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b284) as *mut u32;
    pub const IRQ2_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b288) as *mut u32;
    pub const IRQ2_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b290) as *mut u32;
    pub const IRQ2_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b294) as *mut u32;
    pub const IRQ2_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b298) as *mut u32;
    pub const IRQ2_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b2a0) as *mut u32;
    pub const IRQ2_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b2a4) as *mut u32;
    pub const IRQ2_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b2a8) as *mut u32;
    pub const IRQ3_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b2c0) as *mut u32;
    pub const IRQ3_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b2c4) as *mut u32;
    pub const IRQ3_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b2c8) as *mut u32;
    pub const IRQ3_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b2d0) as *mut u32;
    pub const IRQ3_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b2d4) as *mut u32;
    pub const IRQ3_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b2d8) as *mut u32;
    pub const IRQ3_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b2e0) as *mut u32;
    pub const IRQ3_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b2e4) as *mut u32;
    pub const IRQ3_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b2e8) as *mut u32;
    pub const FIQ0_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b300) as *mut u32;
    pub const FIQ0_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b304) as *mut u32;
    pub const FIQ0_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b308) as *mut u32;
    pub const FIQ0_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b310) as *mut u32;
    pub const FIQ0_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b314) as *mut u32;
    pub const FIQ0_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b318) as *mut u32;
    pub const FIQ0_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b320) as *mut u32;
    pub const FIQ0_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b324) as *mut u32;
    pub const FIQ0_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b328) as *mut u32;
    pub const FIQ1_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b340) as *mut u32;
    pub const FIQ1_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b344) as *mut u32;
    pub const FIQ1_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b348) as *mut u32;
    pub const FIQ1_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b350) as *mut u32;
    pub const FIQ1_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b354) as *mut u32;
    pub const FIQ1_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b358) as *mut u32;
    pub const FIQ1_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b360) as *mut u32;
    pub const FIQ1_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b364) as *mut u32;
    pub const FIQ1_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b368) as *mut u32;
    pub const FIQ2_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b380) as *mut u32;
    pub const FIQ2_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b384) as *mut u32;
    pub const FIQ2_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b388) as *mut u32;
    pub const FIQ2_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b390) as *mut u32;
    pub const FIQ2_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b394) as *mut u32;
    pub const FIQ2_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b398) as *mut u32;
    pub const FIQ2_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b3a0) as *mut u32;
    pub const FIQ2_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b3a4) as *mut u32;
    pub const FIQ2_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b3a8) as *mut u32;
    pub const FIQ3_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b3c0) as *mut u32;
    pub const FIQ3_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b3c4) as *mut u32;
    pub const FIQ3_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b3c8) as *mut u32;
    pub const FIQ3_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b3d0) as *mut u32;
    pub const FIQ3_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b3d4) as *mut u32;
    pub const FIQ3_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b3d8) as *mut u32;
    pub const FIQ3_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b3e0) as *mut u32;
    pub const FIQ3_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b3e4) as *mut u32;
    pub const FIQ3_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b3e8) as *mut u32;
    pub const SWIRQ_SET: *mut u32                    = (MMIO_BASE + 0x0000b3f0) as *mut u32;
    pub const SWIRQ_CLEAR: *mut u32                  = (MMIO_BASE + 0x0000b3f4) as *mut u32;
}

#[cfg(feature="raspi3")]
mod registers {
    use crate::periferals::memmap::MMIO_BASE;

    pub const IRQ0_BASIC_PENDING: *mut u32           = (MMIO_BASE + 0x0000b200) as *mut u32;
    pub const IRQ0_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b204) as *mut u32;
    pub const IRQ0_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b208) as *mut u32;
    pub const IRQ0_FIQ_CONTROL: *mut u32             = (MMIO_BASE + 0x0000b20C) as *mut u32;
    pub const IRQ0_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b210) as *mut u32;
    pub const IRQ0_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b214) as *mut u32;
    pub const IRQ0_SET_BASIC_IRQS: *mut u32          = (MMIO_BASE + 0x0000b218) as *mut u32;
    pub const IRQ0_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b21C) as *mut u32;
    pub const IRQ0_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b220) as *mut u32;
    pub const IRQ0_CLR_BASIC_IQS: *mut u32           = (MMIO_BASE + 0x0000b224) as *mut u32;
}

pub const AUX_IRQ: u32 = 1<<29;

pub fn enable_interrupt_controller(kind: u32) {
    unsafe {
        ptr::write_volatile(registers::IRQ0_SET_EN_0, kind);
    }
}

fn get_esr_el1() -> u64 {
    unsafe {
        let mut result: u64;
        asm!("mrs {}, esr_el1", out(reg)result);
        result
    }
}

fn get_elr_el1() -> u64 {
    unsafe {
        let mut result: u64;
        asm!("mrs {}, elr_el1", out(reg)result);
        result
    }
}

#[no_mangle]
pub extern "C" fn show_invalid_entry_message(kind: u32, esr: u64, address: u64) {
    let mut buf = [0u8; 256];

    let output = format_to::show(&mut buf,
                                 format_args!("Exception: type:{}, ESR:{:x?}, Address:{:x?} \n", kind, esr, address)).unwrap();
    uart1::puts(output);

    loop {}
}

#[no_mangle]
pub extern "C" fn handle_general_irq(kind: u32) {
    if kind != 6 {
        show_invalid_entry_message(kind, get_esr_el1(), get_elr_el1())
    }
    unsafe {
        let mut irq = ptr::read_volatile(registers::IRQ0_PENDING0);
        while irq != 0 {
            if (irq & AUX_IRQ) == AUX_IRQ {
                irq = irq & !AUX_IRQ;
                while (ptr::read_volatile(AUX_MU_IIR) & 4) == 4 {
                    let mut buf = [0u8; 256];
                    let kar = uart1::getc();
                    let output = format_to::show(&mut buf,
                                                 format_args!("Knock Knock: {:?}\n", kar as char)).unwrap();
                    uart1::puts(output);
                }
            }
        }
    }
}