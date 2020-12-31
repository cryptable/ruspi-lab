.globl _start
_start:
  /* 
  Clear the bit 10, which is the Trap Floating Point register (TFP) in the  to disable the trap
  */
  msr CPTR_EL3, xzr

  /* 
  Set up Secure Configuration Register, where we set:
    - bit 10 (RW) to support AArch64 and AArch32 on lower levels 
    - bit 8 (HCE) to enable the Hypervisor call on EL3, EL2, EL1
    - bit 7 (Secure Monitor) to disable the Secure Monitor on EL1 and above
    - bit 0 (Non Secure) whech set all exception levels above EL3 on non Secure
  */
  mov x0, #0x0581
  msr SCR_EL3, x0

  /* 
  Set up ACTLR: Implementation define and not clear what it does
  */
  mov x0, #0x73
  msr ACTLR_EL3, x0

  mrs x6, MPIDR_EL1
  and x6, x6, #0x3
  cbz x6, primary_cpu

  adr x5, spin_cpu0
secondary_spin:
  wfe
  ldr x4, [x5, x6, lsl #3]
  cbz x4, secondary_spin
  mov x0, #0
  b boot_kernel

primary_cpu:
  ldr w4, kernel_entry32
  ldr w0, dtb_ptr32

boot_kernel:
  mov x1, #0
  mov x2, #0
  mov x3, #0
  br x4

.ltorg

.org 0xd8
.globl spin_cpu0
spin_cpu0:
  .quad 0
.org 0xe0
.globl spin_cpu1
spin_cpu1:
  .quad 0
.org 0xe8
.globl spin_cpu2
spin_cpu2:
  .quad 0
.org 0xf0
.globl spin_cpu3
spin_cpu3:
  # Shared with next two symbols/.word
  # FW clears the next 8 bytes after reading the initial value, leaving
  # the location suitable for use as spin_cpu3
.org 0xf0
.globl stub_magic
stub_magic:
  .word 0x5afe570b
.org 0xf4
.globl stub_version
stub_version:
  .word 0
.org 0xf8
.globl dtb_ptr32
dtb_ptr32:
  .word 0x0
.org 0xfc
.globl kernel_entry32
kernel_entry32:
  .word 0x0
