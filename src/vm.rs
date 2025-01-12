use std::{
    collections::HashMap,
    ffi::{c_void, CString},
    usize,
};

use libffi::middle::*;
use libloading::Library;

use crate::{
    conf, l_false, l_null, l_str, l_true, obj,
    opcode::Opcode,
    state::LatticeState,
    types::{CallerCtx, Object, Value},
};

pub struct VM {
    registers: Vec<Object<Value>>,
    pc: usize,
    program: Vec<u8>,
    state: LatticeState,
    stack_top: usize,
    stack: Vec<Object<Value>>,
    history_top: usize,
    history: Vec<Vec<Object<Value>>>,
    extern_libs: HashMap<String, Library>,
    debug: bool,
}

impl VM {
    pub fn new() -> VM {
        let mut registers = Vec::with_capacity(256);
        for _ in 0..256 {
            registers.push(obj!(Value::Null));
        }
        VM {
            registers,
            pc: 0,
            program: vec![],
            state: LatticeState::new(),
            stack_top: 0,
            stack: vec![],
            history_top: 0,
            history: vec![],
            extern_libs: HashMap::new(),
            debug: false,
        }
    }

    pub fn run(&mut self) {
        if self.debug {
            println!("[DEBUG] Entered program execution\n");
        }
        loop {
            if self.pc >= self.program.len() {
                break;
            }

            let op = self.decode_opcode();

            match op {
                Opcode::Invalid(code) => {
                    println!("Invalid Opcode: 0x{:x}", code);
                    break;
                }
                Opcode::MOVE => {
                    let a = self.next_8_bits() as usize;
                    let b = self.next_8_bits() as usize;

                    self.registers[b] = obj!(l_null!());
                    self.registers.swap(a, b);

                    self.pc += 1;

                    if self.debug {
                        println!("\x1b[34mMOVE\x1b[0m r{a} -> r{b}");
                    }
                }
                Opcode::SWAP => {
                    let a = self.next_8_bits() as usize;
                    let b = self.next_8_bits() as usize;

                    self.registers.swap(a, b);

                    self.pc += 1;

                    if self.debug {
                        println!("\x1b[34mSWAP\x1b[0m r{a} <-> r{b}");
                    }
                }
                Opcode::LOAD_I16_AS_I32 => {
                    let reg = self.next_8_bits() as usize;
                    let number = self.next_i16();
                    self.registers[reg] = obj!(Value::I32(number as i32));

                    if self.debug {
                        println!("\x1b[34mLOADI\x1b[0m r{reg} <- {:?}", self.registers[reg]);
                    }
                }
                Opcode::LOAD_I16_AS_I64 => {
                    let reg = self.next_8_bits() as usize;
                    let number = self.next_i16();
                    self.registers[reg] = obj!(Value::I64(number as i64));

                    if self.debug {
                        println!("\x1b[34mLOADI\x1b[0m r{reg} <- {:?}", self.registers[reg]);
                    }
                }
                Opcode::LOAD_U16_AS_U32 => {
                    let reg = self.next_8_bits() as usize;
                    let number = self.next_16_bits();
                    self.registers[reg] = obj!(Value::U32(number as u32));

                    if self.debug {
                        println!("\x1b[34mLOADI\x1b[0m r{reg} <- {:?}", self.registers[reg]);
                    }
                }
                Opcode::LOAD_U16_AS_U64 => {
                    let reg = self.next_8_bits() as usize;
                    let number = self.next_16_bits();
                    self.registers[reg] = obj!(Value::U64(number as u64));

                    if self.debug {
                        println!("\x1b[34mLOADI\x1b[0m r{reg} <- {:?}", self.registers[reg]);
                    }
                }

                Opcode::LOADK => {
                    let reg = self.next_8_bits() as usize;
                    let index = self.next_16_bits() as usize;

                    self.registers[reg] = obj!(self.state.constants[index].clone());

                    if self.debug {
                        println!("\x1b[34mLOADK\x1b[0m r{reg} <- {:?}", self.registers[reg]);
                    }
                }
                Opcode::LOAD_TRUE => {
                    let reg = self.next_8_bits() as usize;

                    self.registers[reg] = obj!(l_true!());

                    self.pc += 2;

                    if self.debug {
                        println!("\x1b[34mLOAD_TRUE\x1b[0m r{reg}");
                    }
                }
                Opcode::LOAD_FALSE => {
                    let reg = self.next_8_bits() as usize;

                    self.registers[reg] = obj!(l_false!());

                    self.pc += 2;

                    if self.debug {
                        println!("\x1b[34mLOAD_FALSE\x1b[0m r{reg}");
                    }
                }
                Opcode::LOAD_NULL => {
                    let reg = self.next_8_bits() as usize;

                    self.registers[reg] = obj!(l_null!());

                    self.pc += 2;

                    if self.debug {
                        println!("\x1b[34mLOAD_NULL\x1b[0m r{reg}");
                    }
                }
                Opcode::CALL => {
                    let reg = self.next_8_bits() as usize;
                    let num_args = self.next_8_bits() as usize;
                    let should_ret = self.next_8_bits();

                    if self.debug {
                        println!(
                            "\x1b[34mCALL\x1b[0m {:?} with {num_args} argument",
                            self.registers[reg]
                        );
                    }

                    if let Value::String(ident) = self.registers[reg].as_ref() {
                        let function = self.state.globals.get(ident);

                        if let Some(value) = function {
                            match value {
                                Value::Function(ref f) => {
                                    let mut ctx = CallerCtx::default();

                                    for i in 0..num_args {
                                        ctx.push_arg(self.registers[i].clone());
                                    }

                                    // Actual function call happens here
                                    f.call(&mut ctx);

                                    if ctx.has_errored() {
                                        eprintln!("{}", ctx.error_unwrap());
                                    } else {
                                        self.stack.push(ctx.return_t);
                                        self.stack_top += 1;

                                        for (i, v) in ctx.args.into_iter().enumerate() {
                                            self.registers[i] = v;
                                        }
                                    }
                                }
                                Value::ExternSymbol(sym) => {
                                    let mut args = Vec::with_capacity(num_args);
                                    let mut arg_types = Vec::with_capacity(num_args);
                                    let mut owned_strings = Vec::new();

                                    for i in 0..num_args {
                                        let (a, atype) = match self.registers[i].as_ref() {
                                            Value::String(s) => {
                                                owned_strings.push(
                                                    CString::new(s.as_str()).expect(
                                                        "Failed to convert String to CString",
                                                    ),
                                                );
                                                (
                                                    arg(&owned_strings.last().unwrap().as_ptr()),
                                                    Type::pointer(),
                                                )
                                            }
                                            Value::I32(n) => (arg(n), Type::i32()),
                                            Value::I64(n) => (arg(n), Type::i64()),
                                            Value::U32(n) => (arg(n), Type::u32()),
                                            Value::U64(n) => (arg(n), Type::u64()),
                                            Value::Null => (arg(&()), Type::void()),
                                            _ => todo!("Not implemented yet"),
                                        };
                                        args.push(a);
                                        arg_types.push(atype);
                                    }

                                    let cif = Cif::new(arg_types.into_iter(), Type::void());

                                    let result: c_void = unsafe { cif.call(CodePtr(*sym), &args) };

                                    // if should_ret == 1 {
                                    //     self.stack.push(obj!(Value::Integer(result as i64)));
                                    //     self.stack_top += 1;
                                    // }
                                }
                                _ => {
                                    eprintln!("{:?} is not a callable function", function);
                                    break;
                                }
                            }
                        } else {
                            eprintln!("function \"{}\" not found in global scope", ident);
                            break;
                        }
                    } else {
                        eprintln!(
                            "{:?} can't be used as a global identifier",
                            self.registers[reg]
                        );
                        break;
                    }
                }
                Opcode::JMPI => {
                    let target = self.next_i24();
                    self.pc -= 4;

                    if self.debug {
                        println!("\x1b[34mJUMPI\x1b[0m PC=0x{:x}+({})", self.pc, target);
                    }

                    if target.is_positive() {
                        self.pc += target as usize;
                    } else {
                        self.pc -= target.abs() as usize;
                    }
                }
                Opcode::LOAD_LIB => {
                    let loc_reg = self.next_8_bits() as usize;
                    let lib_name_reg = self.next_8_bits() as usize;
                    self.pc += 1;
                    let location = &self.registers[loc_reg];
                    let lib_name = &self.registers[lib_name_reg];

                    if let Value::String(ref loc) = location.as_ref() {
                        if let Value::String(ref name) = lib_name.as_ref() {
                            let lib = unsafe { Library::new(loc).expect("Could not load library") };
                            self.extern_libs.insert(name.clone(), lib);
                        } else {
                            panic!("library name must be a string");
                        }
                    } else {
                        panic!("Location \"{:?}\" is not a string", location);
                    }

                    if self.debug {
                        println!("\x1b[34mLOAD_LIB\x1b[0m {:?} as {:?}", location, lib_name);
                    }
                }
                Opcode::LOAD_SYM => {
                    let lib_name_reg = self.next_8_bits() as usize;
                    let sym_name_reg = self.next_8_bits() as usize;
                    self.pc += 1;
                    let lib_name = &self.registers[lib_name_reg];
                    let sym_name = &self.registers[sym_name_reg];

                    if let Value::String(ref lib_name) = lib_name.as_ref() {
                        if let Value::String(ref sym_name) = sym_name.as_ref() {
                            let lib = self
                                .extern_libs
                                .get(lib_name)
                                .expect(&format!("Librarya {} isnt loaded", lib_name));
                            let sym = unsafe {
                                lib.get::<*mut c_void>(sym_name.as_bytes())
                                    .expect("Could not load symbol")
                                    .try_as_raw_ptr()
                                    .unwrap()
                            };
                            self.state.globals.insert(
                                format!("{}.{}", lib_name, sym_name),
                                Value::ExternSymbol(sym),
                            );
                        } else {
                            panic!("symbol name must be a string");
                        }
                    } else {
                        panic!("library \"{:?}\" is not a string", lib_name);
                    }

                    if self.debug {
                        println!("\x1b[34mLOAD_SYM\x1b[0m {:?} from {:?}", sym_name, lib_name);
                    }
                }
                Opcode::DBG_PRINT => {
                    if self.debug {
                        println!("\x1b[34mDBG_PRINT\x1b[0m");
                    }

                    println!("\n---- SNAPSHOT r[0] to r[9] ----");
                    println!("{:?}", &self.registers[..10]);
                    println!("------------------------------");
                    self.pc += 3;
                }
                op => {
                    unimplemented!("{:?}", op);
                }
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    fn next_i16(&mut self) -> i16 {
        let num: [u8; 2] = self.program[self.pc..self.pc + 2].try_into().unwrap();

        self.pc += 2;

        i16::from_be_bytes(num)
    }

    fn next_i24(&mut self) -> i32 {
        let num = [
            self.program[self.pc],
            self.program[self.pc + 1],
            self.program[self.pc + 2],
        ];

        let mut value = ((num[0] as i32) << 16) | ((num[1] as i32) << 8) | (num[2] as i32);

        if num[0] & 0x80 != 0 {
            value |= 0xff_00_00_00u32 as i32;
        }

        self.pc += 3;

        value
    }

    pub fn set_program(&mut self, prog: Vec<u8>) {
        if self.debug {
            println!("[DEBUG] Program size = {} bytes", prog.len());
        }
        self.program = prog;
    }

    pub fn check_magic(&self) {
        assert_eq!(self.program[..4], conf::MAGIC);

        if self.debug {
            println!("[DEBUG] Verified magic sequence: {:?}", conf::MAGIC);
        }
    }

    pub fn check_version(&self) {
        let version: [u8; 4] = self.program[4..8].try_into().unwrap();

        let version = u32::from_be_bytes(version);

        assert!(conf::SUPPORTED_VERSIONS.contains(&version));

        if self.debug {
            println!("[DEBUG] Running LASS version {}", version);
        }
    }

    pub fn set_debug_mode(&mut self, mode: bool) {
        self.debug = mode;

        if self.debug {
            println!("[DEBUG] VM running with DEBUG = true");
        }
    }

    pub fn load_consts(&mut self) {
        let code_start: [u8; 8] = self.program[8..16].try_into().unwrap();
        let code_start = u64::from_be_bytes(code_start);

        let mut const_start = 16;

        while const_start < code_start {
            let ty = self.program[const_start as usize];

            match ty {
                0x01 => {
                    // String
                    const_start += 1;
                    let start = const_start;
                    let mut end = const_start;
                    while self.program[end as usize] != 0x00 {
                        end += 1;
                    }

                    let string =
                        String::from_utf8(self.program[(start as usize)..(end as usize)].to_vec())
                            .expect("Failed to decode utf-8 string constant");

                    self.state.constants.push(l_str!(string));

                    const_start = end + 1;
                }
                0x02 => {
                    // I32
                    const_start += 1;

                    let value: [u8; 4] = self.program
                        [(const_start as usize)..(const_start + 4) as usize]
                        .try_into()
                        .unwrap();
                    let value = i32::from_be_bytes(value);

                    self.state.constants.push(Value::I32(value));

                    const_start += 4;
                }
                0x03 => {
                    // I64
                    const_start += 1;

                    let value: [u8; 8] = self.program
                        [(const_start as usize)..(const_start + 8) as usize]
                        .try_into()
                        .unwrap();
                    let value = i64::from_be_bytes(value);

                    self.state.constants.push(Value::I64(value));

                    const_start += 8;
                }
                0x04 => {
                    // U32
                    const_start += 1;

                    let value: [u8; 4] = self.program
                        [(const_start as usize)..(const_start + 4) as usize]
                        .try_into()
                        .unwrap();
                    let value = u32::from_be_bytes(value);

                    self.state.constants.push(Value::U32(value));

                    const_start += 4;
                }
                0x05 => {
                    // U64
                    const_start += 1;

                    let value: [u8; 8] = self.program
                        [(const_start as usize)..(const_start + 8) as usize]
                        .try_into()
                        .unwrap();
                    let value = u64::from_be_bytes(value);

                    self.state.constants.push(Value::U64(value));

                    const_start += 8;
                }
                _ => panic!("Unknown constant type: {}", ty),
            }
        }

        if self.debug {
            println!("[DEBUG] Loaded constants: {:?}", self.state.constants);
        }
    }

    pub fn jump_to_code(&mut self) {
        let code_start: [u8; 8] = self.program[8..16].try_into().unwrap();
        let code_start = u64::from_be_bytes(code_start);

        self.pc = code_start as usize;

        if self.debug {
            println!("[DEBUG] PC jumped to code start at 0x{:x}", self.pc);
            println!(
                "[DEBUG] Code size = {} bytes",
                self.program[self.pc..].len()
            );
        }
    }
}
