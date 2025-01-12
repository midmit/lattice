#include "rules.asm"

#d8 0x4c, 0x41, 0x53, 0x53 ; 32-bit magic value
#d32 0x0000_0001 ; 32-bit LASS-version
#d64 code

data:
 
code:
  loadi32 r0, 69
  loadi32 r1, -69
  loadi32 r2, 420
  move r2, r1
  swap r0, r1
  print
