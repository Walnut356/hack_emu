# Nand2Tetris Hack computer emulator

Implemented by me in Rust, following the specification described in The Elements of Computing Systems

Contains:

* logic gate and native-rust cpu implementations
* assembler
* Linker
* Jack to Hack compiler
* Simple screen output

Enhancements:

* Doubled addressable ROM space (32k -> 64k) via some assembler fiddling
* (In progress) OS functions built in to rust via expanded instructionset
