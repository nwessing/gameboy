# Game Boy Emulator in Rust

To load a ROM in the emulator specify the path to the ROM file as an argument. The following is an example command when running the emulator via cargo:
```
cargo run -- roms/tetris.gb
```

### Controls
|Emulator|Gameboy|
|---|---|
|W|Up|
|A|Left|
|S|Down|
|D|Right|
|K|A|
|M|B|
|H|Select|
|J|Start|


### To Do
 - Fix sprite transparency issues
 - window rendering - pick up where left off when window is disabled and reenabled
 - joypad interrupt
 - STOP instruction
 - MBC 1 upper ROM banks
 - MBC 1 RAM
 - Memory bank controllers 2 and up
 - keybinding configuration
 - ROM selection
 - OAM transfer timings
 - sound