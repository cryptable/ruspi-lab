
.macro kernel_entry
    sub sp, sp, #256
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
    add	sp, sp, #256
    eret
.endm

.macro handle_irq_entry type
    kernel_entry
    mov	x0, #\type
    bl	handle_general_irq
    kernel_exit
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
    ventry	irq_invalid_el1h      		// IRQ EL1h
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
    handle_irq_entry  1

irq_invalid_el1t:
    handle_irq_entry  2

fiq_invalid_el1t:
    handle_irq_entry  3

error_invalid_el1t:
    handle_irq_entry  4

sync_invalid_el1h:
    handle_irq_entry  5

irq_invalid_el1h:
    handle_irq_entry  6

fiq_invalid_el1h:
    handle_irq_entry  7

error_invalid_el1h:
    handle_irq_entry  8

sync_invalid_el0_64:
    handle_irq_entry  9

irq_invalid_el0_64:
    handle_irq_entry  10

fiq_invalid_el0_64:
    handle_irq_entry  11

error_invalid_el0_64:
    handle_irq_entry  12

sync_invalid_el0_32:
    handle_irq_entry  13

irq_invalid_el0_32:
    handle_irq_entry  14

fiq_invalid_el0_32:
    handle_irq_entry  15

error_invalid_el0_32:
    handle_irq_entry  16

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
