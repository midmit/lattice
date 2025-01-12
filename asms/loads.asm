#include "rules.asm"

#d8 0x4c, 0x41, 0x53, 0x53 ; 32-bit magic value
#d32 0x0000_0001 ; 32-bit LASS-version
#d64 code

data:
  #d string, "test string constant", 0x00
  #d i64
  #d64 9_223_372_036_854_775_807
 
code:
  loadi32 r0, 69
  loadi32 r1, -69
  loadk r2, 0
  loadfalse r3
  loadtrue r4
  loadtrue r5
  loadnull r5
  loadk r6, 1
  print
