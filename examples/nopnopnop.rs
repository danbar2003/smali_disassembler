use smali_disassembler::SmaliDecoder;

fn main() {
    let raw_bytes = [0u8; 4];

    let smali_decoder = SmaliDecoder::new(&raw_bytes);
    let disassembled_smali_code = smali_decoder.decode_all();

    println!("result {:#x?}", disassembled_smali_code)
}
