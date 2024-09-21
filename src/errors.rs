#[derive(Debug)]
pub enum Error {
    InvalidOpcode,
    InvalidPseudoOpcode,
    ReadByteFailed,
}
