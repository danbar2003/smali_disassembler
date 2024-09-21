pub mod dalvik;
pub mod errors;

use dalvik::bytecode_format::DexInstructionFormatReader;
use dalvik::DalvikInstruction;

pub type Result<T> = std::result::Result<T, errors::Error>;
pub struct SmaliDecoder<'a> {
    stream: &'a [u8],
}

impl<'a> SmaliDecoder<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self { stream }
    }

    /// decode all instruction
    pub fn decode_all(&self) -> Vec<DalvikInstruction> {
        // vector to hold instructions
        let mut instructions = vec![];

        // new reader so we start at the beginning
        let mut new_reader = DexInstructionFormatReader::new(&self.stream);

        // loop until finishing decoding all instructions
        while let Ok(inst) = DalvikInstruction::decode_instruction(&mut new_reader) {
            instructions.push(inst)
        }

        // decode all instructions
        instructions
    }
}
