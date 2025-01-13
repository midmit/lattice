#include "rules.asm"

#d64 code

data:
 
code:
  loadi32 r0, 69
  loadi32 r1, -69
  loadi32 r2, 420
  move r2, r1
  swap r0, r1
  print
