Labo 3: Yet Another Bare metal Rust adventure
=============================================

Introduction
------------

This Labo will about investigating interrupts of the Raspberry Pi 4. We will also continue our testing investigation. Our process will be developing our labos on the qemu emulator, automated unittesting and debugging. We will debugging our Rust application using remote debugging into our Qemu session. The result of the unittesting must be reported back from the qemu session into my IDE (CLion).
We will need to perform conditional compiling to differentiate the Qemu environments, Raspberry Pi 3 and Raspberry Pi 4.


Formatting the output
---------------------

Reusing the core framework of Rust, we can create formatted output.

Exception Levels (EL) of ARM Aarch64
------------------------------------

When rfollowing teh documentation, ARM  on the Raspbery Pi has 4 Exception Levels from 0 to 3, where 0 is the lowest privileged level AKA the User level and the 3 is the highest exception level, which is the Firmware level.

+-----+------------------+
|Level| name             |
+-----+------------------+
|  0  | User level       |
|  1  | OS level         |
|  2  | Hypervisor level |
|  3  | Firmware level   |
+-----+------------------+

Interrupts and Timers don't work in User level and can only be configured in lower levels.
The Raspberry Pi 3 and 4 they will start in Exception Level (EL) 2, while the QEMU starts in EL 3. You can also start the Raspberry Pi in EL 3, but you need to overwrite the armstub with some new assembly code.

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

When you read the armstub in 'tools' repository of the Rasberry Pi (ARMSTUB). You see it configures the general interrrupt controller and switches to exception level 2, which is the hypervisor level.
If we replace it with the code above (thank you (LOWLEVELDEV)), then the Raspberry Pi will stay in Exception Level 3. All the interesting code is removed.

Compile & build the new armstub code.

```shell
#!/bin/bash

mkdir -p build
aarch64-none-elf-as -o build/armstub.o src/armstub.S
aarch64-none-elf-ld --section-start=.text=0 -o build/armstub.elf build/armstub.o
aarch64-none-elf-objcopy -O binary build/armstub.elf build/armstub.bin
```

We do have to tell the Raspberry Pi boot code to execute the new armstub code. This is done through **config.txt** file. You need to add a 'armstub'-entry pointing to the armstub.bin file.
```
arm_64bit=1
uart_2ndstage=1
dtoverlay=miniuart-bt

armstub=armstub.bin
```

This allows to boot the new armstub, which will load your kernel, while in excpetion level 3.

Debugging with Qemu and Gdb in Rust
-----------------------------------

You can use gdb to perform debugging action in your Rust code and the Qemu environment. You will perform a remote debugging into Qemu, which runs your bare metal application.


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

Some useful commands:
'info registers': display all registers
'info line': display current line of Rust code with the address in assembly
'disas <begin address>, <end address>': disassemble the code from begin to end address. Nice to us in relation with 'info line', because a line in Rust will be multiple lines in assembly.
'si': step an assembly line



References
----------

- (FORMAT.r) https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment
- (GDB) https://sourceware.org/gdb/onlinedocs/gdb/
- https://wiki.qemu.org/Features/gdbstub
- https://www.qemu.org/docs/master/system/gdb.html
- https://plugins.jetbrains.com/plugin/10428-qemu-for-clion
- (LOWLEVELDEV) https://www.youtube.com/channel/UCRWXAQsN5S3FPDHY4Ttq1Xg
- (ARMSTUB) https://github.com/raspberrypi/tools
- (INLINE) http://llvm.org/docs/LangRef.html#inline-assembler-expressions
