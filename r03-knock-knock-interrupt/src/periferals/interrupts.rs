use core::ptr;
use crate::periferals::memmap::MMIO_BASE;

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

pub const FRAME_SIZE: u32               = 256;

global_asm!(r#"
#include "entry.h"

.macro kernel_entry
    sub sp, sp, {frame_size}
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
    add	sp, sp, {frame_size}
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
    handle_invalid_entry  SYNC_INVALID_EL1t

irq_invalid_el1t:
    handle_invalid_entry  IRQ_INVALID_EL1t

fiq_invalid_el1t:
    handle_invalid_entry  FIQ_INVALID_EL1t

error_invalid_el1t:
    handle_invalid_entry  ERROR_INVALID_EL1t

sync_invalid_el1h:
    handle_invalid_entry  SYNC_INVALID_EL1h

fiq_invalid_el1h:
    handle_invalid_entry  FIQ_INVALID_EL1h

error_invalid_el1h:
    handle_invalid_entry  ERROR_INVALID_EL1h

sync_invalid_el0_64:
    handle_invalid_entry  SYNC_INVALID_EL0_64

irq_invalid_el0_64:
    handle_invalid_entry  IRQ_INVALID_EL0_64

fiq_invalid_el0_64:
    handle_invalid_entry  FIQ_INVALID_EL0_64

error_invalid_el0_64:
    handle_invalid_entry  ERROR_INVALID_EL0_64

sync_invalid_el0_32:
    handle_invalid_entry  SYNC_INVALID_EL0_32

irq_invalid_el0_32:
    handle_invalid_entry  IRQ_INVALID_EL0_32

fiq_invalid_el0_32:
    handle_invalid_entry  FIQ_INVALID_EL0_32

error_invalid_el0_32:
    handle_invalid_entry  ERROR_INVALID_EL0_32

handle_el1_irq:
    kernel_entry
    bl	handle_irq
    kernel_exit

.globl err_hang
err_hang: b err_hang
"#, frame_size = FRAME_SIZE);


fn handle_irq() {

    let mut irq: u32 = 
}