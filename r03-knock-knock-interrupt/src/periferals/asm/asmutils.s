.globl asm_delay


asm_delay:
    subs x0, x0, #1
    bne asm_delay
    ret