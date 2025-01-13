# Lattice - WIP

A simple bytecode VM and maybe a languagee in future.

## Compile asm examples

You need [customasm](https://github.com/hlorenzi/customasm) to compile the assembly examples.


```sh
cargo install customasm
make
```

## Run bytecode

```sh
cargo r -- run -d asms/bins/printf.lass
```

For documentation on opcode check `src/opcode.rs`

Bytecode format is defined in `asms/rules.asm`

# Raylib Demo

Works on native wayland with my `shell.nix`. You might need to troubleshoot.

```sh
cargo r -- run -d asms/bins/raylib.lass
```

![Raylib Hello World](./imgs/raylib)
