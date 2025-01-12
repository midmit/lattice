#ruledef
{
  move r{a: u8}, r{b: u8} => 0x01 @ a @ b @ 0x00
  swap r{a: u8}, r{b: u8} => 0x02 @ a @ b @ 0x00

  loadi32 r{reg: u8}, {value: i16} => 0x10 @ reg @ value
  loadi64 r{reg: u8}, {value: i16} => 0x15 @ reg @ value
  loadu32 r{reg: u8}, {value: u16} => 0x16 @ reg @ value
  loadu64 r{reg: u8}, {value: u16} => 0x17 @ reg @ value

  loadk r{reg: u8}, {index: u16} => 0x11 @ reg @ index
  loadfalse r{reg: u8} => 0x12 @ reg @ 0x0000
  loadtrue r{reg: u8} => 0x13 @ reg @ 0x0000
  loadnull r{reg: u8} => 0x14 @ reg @ 0x0000

  add r{reg1: u8}, r{reg2: u8}, r{reg3: u8} => 0x20 @ reg1 @ reg2 @ reg3

  jmp r{reg: u8} => 0xe0 @ reg @ 0x0000
  jmpi {target: i24} => 0xe1 @ target
  jeq r{reg1: u8}, r{reg2: u8}, r{target: u8} => 0xe2 @ reg1 @ reg2 @ target

  eq r{reg1: u8}, r{reg2: u8}, r{reg3: u8} => 0x30 @ reg1 @ reg2 @ reg3
  neq r{reg1: u8}, r{reg2: u8}, r{reg3: u8} => 0x31 @ reg1 @ reg2 @ reg3
  gt r{reg1: u8}, r{reg2: u8}, r{reg3: u8} => 0x32 @ reg1 @ reg2 @ reg3
  lt r{reg1: u8}, r{reg2: u8}, r{reg3: u8} => 0x33 @ reg1 @ reg2 @ reg3
  gtq r{reg1: u8}, r{reg2: u8}, r{reg3: u8} => 0x34 @ reg1 @ reg2 @ reg3
  ltq r{reg1: u8}, r{reg2: u8}, r{reg3: u8} => 0x35 @ reg1 @ reg2 @ reg3

  call r{reg: u8}, {args: u8}, ret => 0x41 @ reg @ args @ 0x01
  call r{reg: u8}, {args: u8}, noret => 0x41 @ reg @ args @ 0x00

  loadlib r{location: u8}, r{name: u8} => 0x50 @ location @ name @ 0x00
  loadsym r{lib: u8}, r{sym_name: u8} => 0x51 @ lib @ sym_name @ 0x00

  print => 0xfe @ 0x000000
}

string = 0x01
i32 = 0x02
i64 = 0x03
u32 = 0x04
u64 = 0x05
