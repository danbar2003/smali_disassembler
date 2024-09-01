use smali_disassembler::SmaliDeocder;
fn main() {
    let nop_nop_nop_stream = [0, 0, 0];
    let mut res = SmaliDeocder::new(&nop_nop_nop_stream);
    let wow1 = res.decode_all();
    let wow2 = res.decode_all();

    println!("{:?}", &wow1);
    println!("{:?}", &wow2);
}