use smali_disassembler::SmaliDecoder;
fn main() {
    let nop_nop_nop_stream = [0, 0, 0];

    let decoder = SmaliDecoder::new(&nop_nop_nop_stream);
    let result = decoder.decode_all();

    println!("{:?}", &result);
}
