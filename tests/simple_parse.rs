use smali_disassembler::SmaliDecoder;
use std::{fs::File, io::Read};
use zip::ZipArchive;

#[test]
fn test_wow() -> Result<(), Box<dyn std::error::Error>> {
    let apk_file_path = std::env::var("APK_FILE").unwrap();
    let zip_file = File::open(apk_file_path)?;

    let mut archive = ZipArchive::new(zip_file)?;
    let mut dex_file = archive.by_name("classes.dex")?;

    let mut file_data = vec![];
    dex_file.read_to_end(&mut file_data)?;
    let dex_object = dex::DexReader::from_vec(file_data)?;

    for class_res in dex_object.classes() {
        let class = class_res.unwrap();
        for method in class.methods() {
            if let Some(code) = method.code() {
                let code: Vec<u8> = code
                    .insns()
                    .iter()
                    .flat_map(|num| num.to_ne_bytes())
                    .collect();
                let decoder = SmaliDecoder::new(&code);
                let _ = decoder.decode_all();
            }
        }
    }

    Ok(())
}
