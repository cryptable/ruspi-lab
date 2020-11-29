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
    ldr     x5, =_start
    mov     sp, x5
 
    // clear bss
    ldr     x5, =__bss_start
    ldr     w6, =__bss_size
3:  cbz     w6, 4f
    str     xzr, [x5], #8
    sub     w6, w6, #1
    cbnz    w6, 3b
 
    // GPIO location of the Raspberry Pi 4 (a Raspberry Pi 3 is 0x3F20000)
4:  ldr     x19, =0xFE200000

    // Set GPIO pin 21 as output
    mov     w0, #21
    mov     w1, #1
    bl      SetGpioFunc

    // Set GPIO PIN 21 to high, light a LED
    mov     w0, #21
    mov     w1, #1
    bl      SetGpioSet
    // Stop the core
    b       1b

    // Set GPIO pin x0 as defined function in x1
SetGpioFunc:
    // Paramater checking
    cmp     w0, #28
    //  if previous cmp (lt) is true we compare w1 with #8, if true set #nzcv for #4 (enable Z-flag)
    ccmp     w1, #8, #4, lt
    b.ge    SetGpoiFunc_ret

    // FSel0 ?
    cmp     w0, #10
    b.ge    SetGpoiFunc_sel1
    mov     w2, #3
    mul     w0, w0, w2
    ldr     w2, [x19, #0x00]
    mov     w3, #7
    lsl     w3, w3, w0
    mvn     w3, w3
    and     w2, w2, w3
    lsl     w1, w1, w0
    orr     w1, w1, w2
    str     w1, [x19, #0x00]
    ret

SetGpoiFunc_sel1:
    // FSel1 ?
    cmp     w0, #20
    b.ge    SetGpoiFunc_sel2
    sub     w0, w0, #10
    mov     w2, #3
    mul     w0, w0, w2
    ldr     w2, [x19, #0x04]
    mov     w3, #7
    lsl     w3, w3, w0
    mvn     w3, w3
    and     w2, w2, w3
    lsl     w1, w1, w0
    orr     w1, w1, w2
    str     w1, [x19, #0x04]
    ret

SetGpoiFunc_sel2:
    // FSel2 ?
    sub     w0, w0, #20
    mov     w2, #3
    mul     w0, w0, w2
    ldr     w2, [x19, #0x08]
    mov     w3, #7
    lsl     w3, w3, w0
    mvn     w3, w3
    and     w2, w2, w3
    lsl     w1, w1, w0
    orr     w1, w1, w2
    str     w1, [x19, #0x08]

SetGpoiFunc_ret:
    ret

    // Set GPIO pin x0 to output value x1 (on/off)
SetGpioSet:
    // Paramater checking
    cmp     w0, #28
    //  if previous cmp (lt) is true we compare w1 with #2, if true set #nzcv for #4 (enable Z-flag)
    ccmp    w1, #2, #4, lt
    b.ge    SetGpoiSet_ret

    mov     w2, #1
    lsl     w2, w2, w0
    // LED off or on ?
    cmp     w1, #1
    b.ge    SetGpoiSet_on
    // LED off
    str     w2, [x19, #0x28]
    ret
SetGpoiSet_on:
    str     w2, [x19, #0x1C]

SetGpoiSet_ret:
    reT
