Labo 3: Yet Another Bare metal Rust adventure
=============================================

Introduction
------------

This Labo will about investigating interrupts of the Raspberry Pi 4. We will also continue our testing investigation. Our process will be developing our labos on the qemu emulator, automated unittesting and debugging. We will debugging our Rust application using remote debugging into our Qemu session. The result of the unittesting must be reported back from the qemu session into my IDE (CLion).
We will need to perform conditional compiling to differentiate the Qemu environments, Raspberry Pi 3 and Raspberry Pi 4.


Formatting the output
---------------------

Reusing the core framework of Rust, we can create formatted output. Thanks to somebody explaining on Stackoverflow (he derserves the credit), I was able to reuse the core library of rust to create formatted output. You can use the core framework of Rust, because it is not tied to the standard C-library of Linux or Microsoft Windows. So you don't have to rewrite everything, just implement some Traits, which was thankfully very well explained on Stackoverflow.
In Rust core framework exists a *core::format_args* function, which provides formatting and arguments parsing. We don't have *malloc* (yet), because we don't have any memory management and no clib. So we can't use *format* macro.
So I implemented (copied and adapted) my own formatting function *show*, which looks like the *format*, but you need to give it a buffer to put the string.
We implement the *write_str* function from the Write Trait with some extra buffer slicing  and !Bam! we have a format-like function. Not finished, but enough for testing.
The code can be found in the file **format_to.rs** under /src/tools directory. I needed this fucntion for the next section.

Exercise:

1) I allocate the buffer before calling *show*, rewrite show to estimate the buffer length and create the buffer on the stack.

2) The *show* function uses a buffer and gives the buffer to the send for the UART1. Try to modify it without the buffer and using directly the send to UART command. (\*) I don't know this is possible :-/


Exception Levels (EL) of ARM Aarch64
------------------------------------

When following the documentation, ARM on the Raspbery Pi has 4 Exception Levels from 0 to 3, where 0 is the lowest privileged level AKA the User level and the 3 is the highest exception level, which is the Firmware level.

+-----+------------------+
|Level| name             |
+-----+------------------+
|  0  | User level       |
|  1  | OS level         |
|  2  | Hypervisor level |
|  3  | Firmware level   |
+-----+------------------+

The Raspberry Pi 3 and 4 they will start in Exception Level (EL) 2, while the QEMU starts in EL 3. That the Raspberry Pi can't start in EL3 looks normal, but to my surprise it can! You need to overwrite the armstub with some new assembly code. The original can be found on the github page of the Raspberry Pi foundation under the tools directory. There you have to read the assembly code and you find out it sets the Pi in EL2 and configures the GIC controller (only Pi 4).
I removed a lot of code and only kept the EL3 configuration. The nice thing is the configuration of the ACTLR register, which is an implementation defined control register and undocumented.

```asm
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
```

I created a tiny new assembler project with a shell-script to build the *armstub-new.bin* file. This file must be copied on your sd card. Last thing we need to do is tell the boot manager for Raspberry Pi to load the armstub-new.bin by changing the *config.txt* of the Raspberry Pi. We add 'armstub=armstub-new.bin'

```
arm_64bit=1
uart_2ndstage=1
dtoverlay=miniuart-bt

armstub=armstub-new.bin
```

Now your Pi will boot in Exception Level 3 (EL3). How do we now? Using the **[r03_knock-knock-interrupt](https://github.com/cryptable/ruspi-lab) (KNOCK)** project, you can modify the **start.s** file to jump directly to *start_el1* without switching the Exception Levels and add the stack address location to x5 just after *start_el1*.

```

_start: 
    // Stop all cores except core 0
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, start_el1
hang:
    wfe
    b       hang

...

start_el1:
    // where in EL1, so prepare to enter our OS code
    ldr     x5, =_start     // Setup stack before our code
    mov     sp, x5

```
This must be tested with a Raspberry Pi 4 and with the new armstub-new.bin. If you haven't replaced the armstub, you will see it starts in Exceptino Level 2 (EL2).

Now we are switching from EL3 to EL2, which is from Firmware to Hypervisor Level using the following part of the code (please verify with (KNOCK) project in the **start.s**:

```
start_el3:
    mrs     x0, CurrentEL   // Get current Exception level
    and     x0, x0, #12     // only bits of EL information
    cmp     x0, #12         // Running in EL3?
    bne     start_el2       // Probably in el1 then
    // where in EL 3, so prepare to go into EL2
    mov     x2, #0x5b1      // SCR initialization
    msr     scr_el3, x2
    mov     x2, #0x3c9      // 9 -> Switch to EL2
    msr     spsr_el3, x2
    adr     x2, start_el2
    msr     elr_el3, x2
    eret
```

Download the [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/) (ARMREF) to investigate the system registers we are modifying. You will need a decent PDF viewer, because it is 8000 plus pages.

We check the current exception level and if it is not in EL3, we jump to the switch EL2 to Exception Level 1 (EL1).
We initialize *scr_el3* (D13.2.112 in (ARMREF)) register with '5b1', which means :

- NS (bit 0): Lower then EL3 will run in Non Secure state, so they can't access the secure memory (hmmm, interesting to crypto driver)
- IRQ, FIQ, and EA (bit 1,2,3): Interrupts and Errors are not taken to EL3. We will handle them in EL1.
- Res (bit 4, 5, 6): Reserved (whatever)
- HVC (bit 8): Hypervisor call enabled on EL3, EL2, EL1. Maybe we can use these calls for nice tricks.
- RW (bit 10): EL2 or lower levels support AArch32 and AAarch64 (hopefully). We are only interested in Aarch64, because the higher the better ;-)

The other bits of the configuration, I leave for you to read in the documentation.

We prepare to switch to EL2, by configuring the *spsr_el3* (C5.2.19 in (ARMREF)) system register:
- M (bit 0,1,2,3): 9 means we 'return' to EL2 when the *eret* is called and using it own Stack Pointer (I didn't initialized it, but we wont use it because during EL2 we switch directly to EL1).
- DAIF(bit 9,8,7,6): all are set to mask all their interrupts, we won't take interrupts on EL3 (for now at least). 'D' is Debug, 'A' is used for SError traps, 'I' will mask the IRQ interrupts and 'F' blocks the FIQ interrupts.

Al last we tell the EL3 to jump to the EL2 section by writing the 'return' address in the *ELR_EL3* (C5.2.6 in (ARMREF)) register. At least we execute *eret* to switch (or return) to EL2. You can test this again by setting that *start_el2* address in the 'adr x2, start_el2' to 'start_el1'. (don't forget to set the stack pointer for EL1).

It is time to switch from EL2 to EL1, which is from Hypervisor Level to OS Level using the following part of the code (please verify with (KNOCK) project in the **start.s**:

```
    cmp     x0, #4          // Running in EL1?
    beq     start_el1       // Continue to EL1
    // where in EL2, so prepare to enter EL1
    ldr     x0, =0x10330000 // Permit SVE, SIMD and disable trap of SVE registers for EL0 and EL1 (ZEN)
    msr     cpacr_el1, x0
    mrs     x0, cnthctl_el2 // Setup CNTP for EL2
    orr     x0, x0, #3      // Disable traps for EL0
    msr     cnthctl_el2, x0
    msr     cntvoff_el2, xzr // Zeroize the EL2 offset
    mov     x0, #(1 << 31)  // Support Aarch64 and Aarch32 in EL1/EL0
    orr     x0, x0, #(1 << 1) // set Set/Way Invalidation Override , set/way instruction cleans data cache
    msr     hcr_el2, x0     // Write data into hcr_el2
    mov     x0, #0x3c5        // Prepare SPSR to with to EL1 (4 at the end)
    msr     spsr_el2, x0
    adr     x0, start_el1
    msr     elr_el2, x0
    eret
```

You need to set *cpacr_el1* register to allows SIMD instructions, becasue in Rust when you use a byte buffer of 32 or more bytes on the stack, Rust start using SIMD 'movi' command to initialize the buffer with '0' for example. EL1 must support this, which is not by default. Here is where GDB played its important part, because the formatted print workEd in EL3 and EL2, but stopped working in EL1. It started executing traps/exceptions when I implemented the interrupt parts (see later). So after 4 days investigation, research, googling, GDB debugging in Rust and at least stepping into the assembler, I saw the *movi* assembly command. This is a SIMD instruction which must be enables for EL1.
So I set the *cpacr_el1* register (D13.2.29 in (ARMREF)) to 0x90330000:

- FPEN (bit 20,21): '11' No SVE, SIMD or FP instructions will be trapped
- ZEN (bits 16,17): '11' Don't trap any instructions 

Again we check is we are in EL2 and if not we're probably in EL1. We came from EL3 so we can't be in EL3.
Here we are initializiong our stack pointer for EL1

Exceptions (interrupts and traps) and timers can't be configured in User level and must be configured in lower levels.

We will set the *cnthctl_el2* register (D13.8.2 in (ARMREF)):

- EL0PCTEN (bit 0): Don't trap any instructions when accessing counter register
- EL0VCTEN (bit 0): Don't trap any instructions when accessing virtual counter register

The offset of the virtual counter on EL2 is zero. This is done through settings the  *cntvoff_el2* (D13.8.30 (ARMREF)) to zero.

Using the HCR register *hcr_el2* (D13.2.47 in (ARMREF)) we will allow the EL1 and EL2 to run Aarch64 code by settings the RW bit. This register will certainly further investigated in further chapters.

As in the switching from EL3 to EL2, we will use the *spsr_el2* (C5.2.18 in (ARMREF)) to tell the CPU to switch to EL1. The bit parameters are the same except for the last 4 bits, which forces to switch to EL1. 

Again we will set the exception 'return' address in the *elr_el2* (C5.2.5 in (ARMREF)) to the 'start_el1' section in the assembly code to switch to EL1.

The rest is stays the same in the assembly code. You can ofcourse directly switch from EL3 to EL1, but this is left as an exercise.

Exercises:

1) Adapt the *start.s* to switch directly from EL3 to EL1.

Debugging with Qemu and Gdb in Rust
-----------------------------------

You can use gdb to perform debugging action in your Rust code and the Qemu environment. You will perform a remote debugging into QEMU using the [QEMU stub](https://www.qemu.org/docs/master/system/gdb.html) (QEMUGDB), which runs your bare metal application.


### Pre-requisites
- gdb-mulitarch

### Debugging

You need to start Qemu with the options '-s -S':

```
qemu-system-aarch64 -M raspi3 -kernel target/aarch64-unknown-none/debug/r03-knock-knock-interrupt -serial null -serial stdio -s -S
```

Then you start gdb-multiarch with your debug code and connect to qemu:

```
$ gdb-multiarch target/aarch64-unknown-none/debug/r03-knock-knock-interrupt
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/aarch64-unknown-none/debug/r03-knock-knock-interrupt...
(gdb) target remote localhost:1234
Remote debugging using localhost:1234
0x0000000000080000 in _start ()
```

Looks normal, but cool anyway.

Some useful commands, see also [gdb documentation](https://sourceware.org/gdb/onlinedocs/gdb/) (GDB):
'info registers': display all registers
'info line': display current line of Rust code with the address in assembly
'disas <begin address>, <end address>': disassemble the code from begin to end address. Nice to us in relation with 'info line', because a line in Rust will be multiple lines in assembly.
'si': step an assembly line

Exercises:

1) Test your gdb integration in your favorite IDE platform (see [Clion integration](https://plugins.jetbrains.com/plugin/10428-qemu-for-clion) (CLIONGDB)).

Interrupts
----------

We're at the OS Exception Level (EL1), so we can start trapping interrupts. As the person on the [Low Level Dev channel](https://www.youtube.com/channel/UCRWXAQsN5S3FPDHY4Ttq1Xg) (LOWLEVELDEV) shows, we will also trap the interrupts from our AUX port.


Conclusion
----------

These are all labos and everything is quite anarchic, but we wont let architecture and abstraction hide the simplicity of low level development. We also see more and more how this code is very targetted to the ARM processor/ When we want to write an OS, we will need to:
1) put all the ARM targetted code in 1 place. This means the Rust interface together with its assembler counterpart. Implement stubs for unittesting the code using the special ARM features.
2) Write abstractions, which can be reused for other modern processors

Next chapter will be timers and clock interrupts. When this labo is made, we can think in starting our OS Kernel.

References
----------

- (FORMAT) https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment
- (FORMAT_ARGS) https://doc.rust-lang.org/core/macro.format_args.html
- (KNOCK) https://github.com/cryptable/ruspi-lab
- (ARMREF) https://developer.arm.com/documentation/ddi0487/latest/
- (GDB) https://sourceware.org/gdb/onlinedocs/gdb/
- (QEMUGDB) https://www.qemu.org/docs/master/system/gdb.html
- (CLIONGDB) https://plugins.jetbrains.com/plugin/10428-qemu-for-clion
- (LOWLEVELDEV) https://www.youtube.com/channel/UCRWXAQsN5S3FPDHY4Ttq1Xg
- (ARMSTUB) https://github.com/raspberrypi/tools
- (INLINE) http://llvm.org/docs/LangRef.html#inline-assembler-expressions
