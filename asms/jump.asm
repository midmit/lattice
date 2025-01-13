#include "rules.asm"

#d64 start

data:
  #d string, "Hello from main 😳", 0x00
  #d string, "Hello from foo", 0x00
  #d string, "__lat_dbg", 0x00

start:
  loadk r1, 2
  loadk r0, 0 
  call r1, 1, noret
  jmpi (foo - $)
  .foo_ret:
  jmpi (end - $)

foo:
  loadk r0, 1
  call r1, 1, noret
  jmpi (start.foo_ret - $)

end:
