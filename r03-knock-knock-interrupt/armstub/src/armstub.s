# Start entry, which will directly start the kernel
.globl _start
_start:
    ldr w0, kernel_entry32
    br x0

.ltorg

# Magic code which must be the following value
.org 0xf0
.globl stub_magic
stub_magic:
    .word 0x5afe570b

# The version is set to 0
.org 0xf4
.globl stub_version
stub_version:
    .word 0

# Start the kernel
.org 0xfc
.globl kernel_entry32
kernel_entry32:
    .word 0x0
