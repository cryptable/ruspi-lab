use core::ptr;
use crate::periferals::memmap::MMIO_BASE;
use crate::periferals::uart1;
use crate::periferals::uart1::AUX_MU_IIR;
use crate::tools::format_to;

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
const IRQ0_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b200) as *mut u32;
const IRQ0_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b204) as *mut u32;
const IRQ0_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b208) as *mut u32;
const IRQ0_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b210) as *mut u32;
const IRQ0_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b214) as *mut u32;
const IRQ0_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b218) as *mut u32;
const IRQ0_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b220) as *mut u32;
const IRQ0_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b224) as *mut u32;
const IRQ0_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b228) as *mut u32;
const IRQ0_STATUS0: *mut u32                 = (MMIO_BASE + 0x0000b230) as *mut u32;
const IRQ0_STATUS1: *mut u32                 = (MMIO_BASE + 0x0000b234) as *mut u32;
const IRQ0_STATUS2: *mut u32                 = (MMIO_BASE + 0x0000b238) as *mut u32;
const IRQ1_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b240) as *mut u32;
const IRQ1_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b244) as *mut u32;
const IRQ1_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b248) as *mut u32;
const IRQ1_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b250) as *mut u32;
const IRQ1_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b254) as *mut u32;
const IRQ1_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b258) as *mut u32;
const IRQ1_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b260) as *mut u32;
const IRQ1_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b264) as *mut u32;
const IRQ1_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b268) as *mut u32;
const IRQ2_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b280) as *mut u32;
const IRQ2_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b284) as *mut u32;
const IRQ2_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b288) as *mut u32;
const IRQ2_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b290) as *mut u32;
const IRQ2_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b294) as *mut u32;
const IRQ2_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b298) as *mut u32;
const IRQ2_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b2a0) as *mut u32;
const IRQ2_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b2a4) as *mut u32;
const IRQ2_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b2a8) as *mut u32;
const IRQ3_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b2c0) as *mut u32;
const IRQ3_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b2c4) as *mut u32;
const IRQ3_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b2c8) as *mut u32;
const IRQ3_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b2d0) as *mut u32;
const IRQ3_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b2d4) as *mut u32;
const IRQ3_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b2d8) as *mut u32;
const IRQ3_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b2e0) as *mut u32;
const IRQ3_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b2e4) as *mut u32;
const IRQ3_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b2e8) as *mut u32;
const FIQ0_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b300) as *mut u32;
const FIQ0_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b304) as *mut u32;
const FIQ0_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b308) as *mut u32;
const FIQ0_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b310) as *mut u32;
const FIQ0_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b314) as *mut u32;
const FIQ0_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b318) as *mut u32;
const FIQ0_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b320) as *mut u32;
const FIQ0_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b324) as *mut u32;
const FIQ0_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b328) as *mut u32;
const FIQ1_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b340) as *mut u32;
const FIQ1_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b344) as *mut u32;
const FIQ1_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b348) as *mut u32;
const FIQ1_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b350) as *mut u32;
const FIQ1_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b354) as *mut u32;
const FIQ1_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b358) as *mut u32;
const FIQ1_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b360) as *mut u32;
const FIQ1_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b364) as *mut u32;
const FIQ1_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b368) as *mut u32;
const FIQ2_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b380) as *mut u32;
const FIQ2_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b384) as *mut u32;
const FIQ2_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b388) as *mut u32;
const FIQ2_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b390) as *mut u32;
const FIQ2_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b394) as *mut u32;
const FIQ2_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b398) as *mut u32;
const FIQ2_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b3a0) as *mut u32;
const FIQ2_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b3a4) as *mut u32;
const FIQ2_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b3a8) as *mut u32;
const FIQ3_PENDING0: *mut u32                = (MMIO_BASE + 0x0000b3c0) as *mut u32;
const FIQ3_PENDING1: *mut u32                = (MMIO_BASE + 0x0000b3c4) as *mut u32;
const FIQ3_PENDING2: *mut u32                = (MMIO_BASE + 0x0000b3c8) as *mut u32;
const FIQ3_SET_EN_0: *mut u32                = (MMIO_BASE + 0x0000b3d0) as *mut u32;
const FIQ3_SET_EN_1: *mut u32                = (MMIO_BASE + 0x0000b3d4) as *mut u32;
const FIQ3_SET_EN_2: *mut u32                = (MMIO_BASE + 0x0000b3d8) as *mut u32;
const FIQ3_CLR_EN_0: *mut u32                = (MMIO_BASE + 0x0000b3e0) as *mut u32;
const FIQ3_CLR_EN_1: *mut u32                = (MMIO_BASE + 0x0000b3e4) as *mut u32;
const FIQ3_CLR_EN_2: *mut u32                = (MMIO_BASE + 0x0000b3e8) as *mut u32;
const SWIRQ_SET: *mut u32                    = (MMIO_BASE + 0x0000b3f0) as *mut u32;
const SWIRQ_CLEAR: *mut u32                  = (MMIO_BASE + 0x0000b3f4) as *mut u32;


pub const FRAME_SIZE: u32               = 256;

global_asm!(r#"
.macro kernel_entry
    sub sp, sp, 256
    stp x0, x1, [sp, #16 * 0]
    stp x2, x3, [sp, #16 * 1]
    stp	x4, x5, [sp, #16 * 2]
    stp	x6, x7, [sp, #16 * 3]
    stp	x8, x9, [sp, #16 * 4]
    stp	x10, x11, [sp, #16 * 5]
    stp	x12, x13, [sp, #16 * 6]
    stp	x14, x15, [sp, #16 * 7]
    stp	x16, x17, [sp, #16 * 8]
    stp	x18, x19, [sp, #16 * 9]
    stp	x20, x21, [sp, #16 * 10]
    stp	x22, x23, [sp, #16 * 11]
    stp	x24, x25, [sp, #16 * 12]
    stp	x26, x27, [sp, #16 * 13]
    stp	x28, x29, [sp, #16 * 14]
    str	x30, [sp, #16 * 15]
.endm

.macro kernel_exit
    ldp x0, x1, [sp, #16 * 0]
    ldp x2, x3, [sp, #16 * 1]
    ldp	x4, x5, [sp, #16 * 2]
    ldp	x6, x7, [sp, #16 * 3]
    ldp	x8, x9, [sp, #16 * 4]
    ldp	x10, x11, [sp, #16 * 5]
    ldp	x12, x13, [sp, #16 * 6]
    ldp	x14, x15, [sp, #16 * 7]
    ldp	x16, x17, [sp, #16 * 8]
    ldp	x18, x19, [sp, #16 * 9]
    ldp	x20, x21, [sp, #16 * 10]
    ldp	x22, x23, [sp, #16 * 11]
    ldp	x24, x25, [sp, #16 * 12]
    ldp	x26, x27, [sp, #16 * 13]
    ldp	x28, x29, [sp, #16 * 14]
    ldr	x30, [sp, #16 * 15]
    add	sp, sp, 256
    eret
.endm

.macro handle_invalid_entry type
    kernel_entry
    mov	x0, #\type
    mrs	x1, esr_el1
    mrs	x2, elr_el1
    bl	show_invalid_entry_message
    b	err_hang
.endm

.macro	ventry	label
.align	7
    b	\label
.endm

//Exception vectors table
.align	11
.globl vectors
vectors:
    ventry	sync_invalid_el1t			// Synchronous EL1t
    ventry	irq_invalid_el1t			// IRQ EL1t
    ventry	fiq_invalid_el1t			// FIQ EL1t
    ventry	error_invalid_el1t			// Error EL1t

    ventry	sync_invalid_el1h			// Synchronous EL1h
    ventry	handle_el1_irq				// IRQ EL1h
    ventry	fiq_invalid_el1h			// FIQ EL1h
    ventry	error_invalid_el1h			// Error EL1h

    ventry	sync_invalid_el0_64			// Synchronous 64-bit EL0
    ventry	irq_invalid_el0_64			// IRQ 64-bit EL0
    ventry	fiq_invalid_el0_64			// FIQ 64-bit EL0
    ventry	error_invalid_el0_64		// Error 64-bit EL0

    ventry	sync_invalid_el0_32			// Synchronous 32-bit EL0
    ventry	irq_invalid_el0_32			// IRQ 32-bit EL0
    ventry	fiq_invalid_el0_32			// FIQ 32-bit EL0
    ventry	error_invalid_el0_32		// Error 32-bit EL0


sync_invalid_el1t:
    handle_invalid_entry  1

irq_invalid_el1t:
    handle_invalid_entry  2

fiq_invalid_el1t:
    handle_invalid_entry  3

error_invalid_el1t:
    handle_invalid_entry  4

sync_invalid_el1h:
    handle_invalid_entry  5

fiq_invalid_el1h:
    handle_invalid_entry  6

error_invalid_el1h:
    handle_invalid_entry  7

sync_invalid_el0_64:
    handle_invalid_entry  8

irq_invalid_el0_64:
    handle_invalid_entry  9

fiq_invalid_el0_64:
    handle_invalid_entry  10

error_invalid_el0_64:
    handle_invalid_entry  11

sync_invalid_el0_32:
    handle_invalid_entry  12

irq_invalid_el0_32:
    handle_invalid_entry  13

fiq_invalid_el0_32:
    handle_invalid_entry  14

error_invalid_el0_32:
    handle_invalid_entry  15

handle_el1_irq:
    kernel_entry
    bl	handle_irq
    kernel_exit

.globl err_hang
err_hang: b err_hang

.globl irq_init_vectors
irq_init_vectors:
    adr x0, vectors
    msr vbar_el1, x0
    ret

.globl irq_enable
irq_enable:
    msr daifclr, #2
    ret

.globl irq_disable
irq_disable:
    msr daifset, #2
    ret
"#);

extern {
    pub fn irq_init_vectors();
    pub fn irq_enable();
    pub fn irq_disable();
}

const AUX_IRQ: u32 = 1<<29;

pub fn enable_interrupt_controller() {
    unsafe {
        ptr::write_volatile(IRQ0_PENDING0, AUX_IRQ);
    }
}

#[no_mangle]
pub extern "C" fn show_invalid_entry_message(kind: u32, esr: u64, address: u64) {
    let mut buf = [0u8; 256];

    let output = format_to::show(&mut buf,
        format_args!("Exception: type:{}, ESR:{:x?}, Address:{:x?} \n", kind, esr, address)).unwrap();
    uart1::puts(output);
}

#[no_mangle]
pub extern "C" fn handle_irq() {
    unsafe {
        uart1::puts("enter\n");
        let mut irq = ptr::read_volatile(IRQ0_PENDING0);
        while irq != 0 {
            if (irq & AUX_IRQ) == AUX_IRQ {
                irq = irq & !AUX_IRQ;
                while (ptr::read_volatile(AUX_MU_IIR) & 4) == 4 {
                    let mut buf = [0u8; 256];
                    let kar = uart1::getc();
                    let output = format_to::show(&mut buf,
                                                 format_args!("Recv: {}\n", kar)).unwrap();
                    uart1::puts(output);
                }
            }
        }
    }
}