use smali_disassembler::SmaliDecoder;
use std::{fs::File, io::Read};
use zip::ZipArchive;

#[test]
fn test_wow() -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open("tests/duolingo.apk")?;
    let mut file_data = vec![];
    let mut archive = ZipArchive::new(zip_file)?;

    let mut dex_file = archive.by_name("classes.dex")?;
    dex_file.read_to_end(&mut file_data)?;
    let dex_object = dex::DexReader::from_vec(file_data)?;

    for c in dex_object.classes() {
        let c = c.unwrap();
        for method in c.methods() {
            let class_name = dex_object.get_type(c.id())?.to_string();
            if let Some(code) = method.code() {
                println!("{}->{}", class_name, method.name().to_string());
                let code: Vec<u8> = code
                    .insns()
                    .iter()
                    .flat_map(|num| num.to_ne_bytes())
                    .collect();
                let decoder = SmaliDecoder::new(&code);
                let please = decoder.decode_all();
                return Ok(());
            }
        }
    }

    Ok(())
}
