// AArch64 mode
 
// This file will be kept as first (see linker script).
.section ".text.start"
 
// Make _start global.
.globl _start
 
_start: 
    // Stop all cores except core 0
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, start_el3
hang:
    wfe
    b       hang

start_el3:
    mrs     x0, CurrentEL     // Get current Exception level
    and     x0, x0, #12       // only bits of EL information
    cmp     x0, #12           // Running in EL3?
    bne     start_el2         // Probably in el1 then
    // where in EL 3, so prepare to go into EL2
    mov     x2, #0x5b1        // SCR initialization
    msr     scr_el3, x2
    mov     x2, #0x3c9        // 9 -> Switch to EL2
    msr     spsr_el3, x2
    adr     x2, start_el2
    msr     elr_el3, x2
    eret

start_el2:
    cmp     x0, #4            // Running in EL1?
    beq     start_el1         // Continue to EL1
    // where in EL2, so prepare to enter EL1
    ldr     x0, =0x330000     // Permit SVE, SIMD and disable trap of SVE registers for EL0 and EL1 (ZEN)
    msr     cpacr_el1, x0
    mrs     x0, cnthctl_el2   // Setup CNTP for EL1
    orr     x0, x0, #3        // Disable traps for EL0
    msr     cnthctl_el2, x0
    msr     cntvoff_el2, xzr  // Zeroize the EL2 offset
    mov     x0, #(1 << 31)    // Support Aarch64 and Aarch32 in EL1/EL0
    orr     x0, x0, #(1 << 1) // set Set/Way Invalidation Override , set/way instruction cleans data cache
    msr     hcr_el2, x0       // Write data into hcr_el2
    mov     x0, #0x3c5        // Prepare SPSR to with to EL1 (4 at the end)
    msr     spsr_el2, x0
    adr     x0, start_el1
    msr     elr_el2, x0
    eret

start_el1:
    // where in EL1, so prepare to enter our OS code
    // clear bss
    ldr     x5, =_start       // Setup stack before our code
    mov     sp, x5
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
zeroize:
    cbz     w6, lift_off
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, zeroize
 
    // Jump into the Rust program
lift_off:
    bl      rmain
    // Stop the core when returned
    b       hang
  