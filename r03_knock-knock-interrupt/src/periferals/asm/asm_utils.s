.globl asm_delay
asm_delay:
    subs x0, x0, #1
    bne asm_delay
    ret

.globl get_el
asm_get_el:
    mrs x0, CurrentEL
    lsr x0, x0, #2
    ret
