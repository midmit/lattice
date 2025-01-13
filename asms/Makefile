all: create_bin_dir loads moves constants jump printf raylib

create_bin_dir:
	mkdir -p bins/

loads: loads.asm
	customasm -o bins/loads.lass loads.asm

moves: moves.asm
	customasm -o bins/moves.lass moves.asm

constants: constants.asm
	customasm -o bins/constants.lass constants.asm

map: map.asm
	customasm -o bins/map.lass map.asm

jump: jump.asm
	customasm -o bins/jump.lass jump.asm

printf: printf.asm
	customasm -o bins/printf.lass printf.asm

raylib: raylib.asm
	customasm -o bins/raylib.lass raylib.asm
