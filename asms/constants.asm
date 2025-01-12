#include "rules.asm"

#d8 0x4c, 0x41, 0x53, 0x53 ; 32-bit magic value
#d32 0x0000_0001 ; 32-bit LASS-version
#d64 code

data:
  #d string, "test string constant", 0x00
  #d i32 ; integer
  #d32 69
  #d i32
  #d32 -69
 
code:
