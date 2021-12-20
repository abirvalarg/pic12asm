# pic12asm
simple assembler for PIC12 microcontrollers

## Note
This guide was made for Linux. It may work on MacOS. For windows some details may be different.

## installation
### from sources
1. install Rust and Cargo to build the program
2. run `cargo build --release` in root directory of this repository
3. copy file `target/release/pic12asm` to any directory in your PATH (eg. `~/.local/bin` or `/usr/local/bin`)

### From package
Simply put the executable file in a directory with executable files

## Usage
don't forget about [notes](#notes)

You can use all instructions from the datasheet. Labels can be defined by adding a colon (`:`) after it's name, then it can be used instead of any number. As in other assemblers, everything that goes after a semicolon (`;`) is a comment and therefore ignored.

At this point you can only compile 1 file. Hopefully, it won't be a big problem because of limited resources of the platform.

There's 1 additional preudo-instruction `DATA k`. Corresponding word in ROM will be set to the number `k`.

to build a file use following comand:
```bash
pic12asm input.asm output.bin
```
`output.bin` is a raw binary file ready to be flashed into your controller

### Example
Here's an example of program for blinking a LED. I'm not sure if it works as intended on the hardware but it's a valid piece of code that can be built with this assembler.
```
DATA start
start:
        CLRWDT
        MOVLW 0b111110
        TRIS 6
        MOVLW 0
        MOVWF 1
        MOVLW 0b11010000
        OPTION
loop:
        BTFSC 1, 5
        GOTO isSet
        MOVLW 0
        MOVWF 6
        GOTO loop
isSet:
        MOVLW 1
        MOVWF 6
        GOTO loop
```

### notes
- Tested only with PIC12F509
- Register names don't work yet, will be fixed in future versions
- Destination select is always required if supported by instruction
- Assembler doesn't check for incorrect values
