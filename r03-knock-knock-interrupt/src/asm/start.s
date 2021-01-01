// AArch64 mode
 
// This file will be kept as first (see linker script).
.section ".text.start"
 
// Make _start global.
.globl _start
 
_start: 
    // Stop all cores except core 0
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
1:   
    wfe
    b       1b

2:
    ldr     x0, =0x30d00800 //SCTLR_EL1
    msr     sctlr_el1, x0

    ldr     x0, =(1 << 31)
    msr     hcr_el2, x0

    ldr     x0, =0x5b1
    msr     scr_el3, x0

    ldr     x0, =0x1c5
    msr     spsr_el3, x0

    adr     x0, 5f
    msr     elr_el3, x0

    eret
5:
    ldr     x5, =_start
    mov     sp, x5
 
    // clear bss
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
3:  cbz     w6, 4f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 3b
 
    // Jump into the C program
4:  bl      rmain

    // Stop the core when returned
    b       1b
  