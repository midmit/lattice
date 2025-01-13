#include "rules.asm"

#d64 start

data:
  #d string, "Hello, World!\n", 0x00
  #d string, "std/libs/libc.so", 0x00
  #d string, "libc", 0x00
  #d string, "printf", 0x00
  #d string, "libc.printf", 0x00

start:
  loadk r0, 1 ; r0 = "std/libs/libc.so"
  loadk r1, 2 ; r1 = "libc"
  loadlib r0, r1 ; load "std/libs/libc.so" as "libc"
  loadk r0, 3 ; r0 = "printf"
  loadsym r1, r0 ; load "printf" from "libc"
  loadk r0, 0 ; r0 = "Hello, World!\n"
  loadk r1, 4 ; r1 = "libc.printf"
  call r1, 1, noret ; libc.printf("Hello, World!\n")
