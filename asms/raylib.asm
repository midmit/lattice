#include "rules.asm"

#d64 code

data:
  #d string, "Hello, World!", 0x00 ; 0
  #d string, "asms/libraylib.so.5.5.0", 0x00 ; 1
  #d string, "raylib", 0x00 ; 2
  #d string, "InitWindow", 0x00 ; 3
  #d string, "BeginDrawing", 0x00 ; 4
  #d string, "ClearBackground", 0x00 ; 5
  #d string, "DrawText", 0x00 ; 6
  #d string, "EndDrawing", 0x00 ; 7
  ; ----------- fully qualified import
  #d string, "raylib.InitWindow", 0x00 ; 8
  #d string, "raylib.BeginDrawing", 0x00 ; 9
  #d string, "raylib.ClearBackground", 0x00 ; 10
  #d string, "raylib.DrawText", 0x00 ; 11
  #d string, "raylib.EndDrawing", 0x00 ; 12
  #d u32, 0xffff_ffff ; 13 WHITE
  #d u32, 0xff00_0000 ; 14 BLACK
  ; ------------ we need libc to sleep
  #d string, "std/libs/libc.so", 0x00 ; 15
  #d string, "libc", 0x00 ; 16
  #d string, "sleep", 0x00 ; 17
  #d string, "libc.sleep", 0x00 ; 18

code:
  loadk r255, 1
  loadk r254, 2
  loadlib r255, r254 ; import raylib
  loadk r253, 3
  loadsym r254, r253
  loadk r253, 8 ; InitWindow()
  loadk r252, 4
  loadsym r254, r252
  loadk r252, 9 ; BeginDrawing()
  loadk r251, 5
  loadsym r254, r251
  loadk r251, 10 ; ClearBackground()
  loadk r250, 6
  loadsym r254, r250
  loadk r250, 11 ; DrawText()
  loadk r249, 7
  loadsym r254, r249
  loadk r249, 12 ; EndDrawing()
  ; -- load libc
  loadk r248, 15
  loadk r247, 16
  loadlib r248, r247 ; import libc
  loadk r248, 17
  loadsym r247, r248
  loadk r248, 18 ; sleep()
  ; ----- call functions -------
  loadi32 r0, 500
  loadi32 r1, 500
  loadk r2, 0 ; Hello, World!
  call r253, 3, noret ; InitWindow(500, 500, "Hello, World!");
  .eventloop:
  call r252, 0, noret ; BeginDrawing();
  loadk r0, 13 ; WHITE
  call r251, 1, noret ; ClearBackground(WHITE);
  loadk r0, 0 ; Hello, World!
  loadi32 r1, 10
  loadi32 r2, 10
  loadi32 r3, 50
  loadk r4, 14 ; BLACK
  call r250, 5, noret ; DrawText("Hello, World!", 10, 10, 20, BLACK);
  call r249, 0, noret ; EndDrawing();
  ; jmpi code.eventloop - $
  ; sleep for 5s
  loadu32 r0, 5
  call r248, 1, noret ; sleep(5)

