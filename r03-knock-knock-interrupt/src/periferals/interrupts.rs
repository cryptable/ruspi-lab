use core::ptr;
use crate::periferals::memmap::MMIO_BASE;

// Interrupts registers
const TIMER_CR: *mut u32                = (MMIO_BASE + 0x00000000) as *mut u32;
const TIMER_PRESCALER: *mut u32         = (MMIO_BASE + 0x00000008) as *mut u32;
const TIMER_LS32: *mut u32              = (MMIO_BASE + 0x0000001C) as *mut u32;
const TIMER_MS32: *mut u32              = (MMIO_BASE + 0x00000020) as *mut u32;

const INTRPT_ROUTING: *mut u32          = (MMIO_BASE + 0x0000000C) as *mut u32;
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
