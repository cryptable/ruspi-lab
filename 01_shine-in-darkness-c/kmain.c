/*
MIT License

Copyright (c) 2020 Cryptable BVBA

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#define MMIO_BASE       0xFE000000

#define GPFSEL0         ((volatile unsigned int*)(MMIO_BASE+0x00200000))
#define GPFSEL1         ((volatile unsigned int*)(MMIO_BASE+0x00200004))
#define GPFSEL2         ((volatile unsigned int*)(MMIO_BASE+0x00200008))
#define GPFSEL3         ((volatile unsigned int*)(MMIO_BASE+0x0020000C))
#define GPFSEL4         ((volatile unsigned int*)(MMIO_BASE+0x00200010))
#define GPFSEL5         ((volatile unsigned int*)(MMIO_BASE+0x00200014))
#define GPSET0          ((volatile unsigned int*)(MMIO_BASE+0x0020001C))
#define GPSET1          ((volatile unsigned int*)(MMIO_BASE+0x00200020))
#define GPCLR0          ((volatile unsigned int*)(MMIO_BASE+0x00200028))
#define GPLEV0          ((volatile unsigned int*)(MMIO_BASE+0x00200034))
#define GPLEV1          ((volatile unsigned int*)(MMIO_BASE+0x00200038))
#define GPEDS0          ((volatile unsigned int*)(MMIO_BASE+0x00200040))
#define GPEDS1          ((volatile unsigned int*)(MMIO_BASE+0x00200044))
#define GPHEN0          ((volatile unsigned int*)(MMIO_BASE+0x00200064))
#define GPHEN1          ((volatile unsigned int*)(MMIO_BASE+0x00200068))
#define GPPUD           ((volatile unsigned int*)(MMIO_BASE+0x00200094))
#define GPPUDCLK0       ((volatile unsigned int*)(MMIO_BASE+0x00200098))
#define GPPUDCLK1       ((volatile unsigned int*)(MMIO_BASE+0x0020009C))

#define GPFUNC_IN       0
#define GPFUNC_OUT      1
#define GPFUNC_ALT0     4
#define GPFUNC_ALT1     5
#define GPFUNC_ALT2     6
#define GPFUNC_ALT3     7
#define GPFUNC_ALT4     3
#define GPFUNC_ALT5     2

#define TRUE            1
#define FALSE           0

/**
 * Set GPIO Function type
 */
void setGPIOFunc(int pin, int fun) {
  if (pin >= 28) return;
  if (fun >= 8) return;

  int shift = (pin % 10) * 3; // Calculate shift for 3 bit function code

  if (pin < 10) {
    //               clear 3 bits          set 3 bits
    *GPFSEL0 = (*GPFSEL0 & ~(7 << shift)) | (fun << shift); 
    return;
  }
  if (pin < 20) {
    *GPFSEL1 = (*GPFSEL1 & ~(7 << shift)) | (fun << shift);
    return;
  }

  *GPFSEL2 = (*GPFSEL2 & ~(7 << shift)) | (fun << shift);
  return;
}

/**
 * Enable the PIN (set to high)
 */
void enableGPIO(int pin, char on) {
  if (pin >= 28) return;

  if (on) {
    *GPSET0 = 1 << pin;
    return;
  }

  *GPCLR0 = 1 << pin;
  return;
}


void kmain() {
  setGPIOFunc(21, GPFUNC_OUT);
  enableGPIO(21, TRUE);
}