# Overview

VERY simple smali disassembler written in Rust

# TODO

- [ ] add control flow traits to basic blocks related instuctions (goto, branch, return, ...)
- [ ] add basic util as a cli (kinda like baksmali)
- [ ] add a simple example and document the functions (even though its very very simple :D)

# Example
```rust
use smali_disassembler::SmaliDecoder;

fn main() {
    let raw_bytes = [0u8; 4];

    let smali_decoder = SmaliDecoder::new(&raw_bytes);
    let disassembled_smali_code = smali_decoder.decode_all();

    println!("result {:#x?}", disassembled_smali_code)
}
```
```bash
result [
    DalvikInstruction {
        inst: Nop,
        offset: 0x0,
    },
    DalvikInstruction {
        inst: Nop,
        offset: 0x2,
    },
]
```
