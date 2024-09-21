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
        // c.id();
        // let please = dex_object.class_defs().into_iter().filter(|idk| idk.is_ok());
        // for idk in please  {
        //     let a = idk.unwrap();
        //     println!("{}")

        // }
        // Convert Vec<u16> to Vec<u8>
        for method in c.methods() {
            let class_name = dex_object.get_type(c.id())?.to_string();
            // println!("{}->method:{}", class_name, method.name());
            if let Some(code) = method.code() {
                println!("{}->{}", class_name, method.name().to_string());
                let code: Vec<u8> = code
                    .insns()
                    .iter()
                    .flat_map(|num| num.to_ne_bytes())
                    .collect();
                println!("{:x?}", &code);
                let decoder = SmaliDecoder::new(&code);
                let please = decoder.decode_all();
                println!("{:#?}", please);
                return Ok(());
            }

            // println!("{:?}", please);
        }
    }
    // for (index, method_id) in dex_object.method_ids().enumerate() {
    //     println!("0x{:x} {}", index, dex_object.get_string(method_id.unwrap().name_idx())?);
    // }

    // let p = wow.method_ids().into_iter()
    // println!("{:?}", wow.method_ids().collect::<Vec<u32>>());
    // // let contents = std::fs::read_to_string("Cargo.toml").expect("Failed to read file");
    // // println!("{}", contents);
    // // assert_eq!(1, 1);

    Ok(())
}
