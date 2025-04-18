#[derive(Debug, Clone)]
pub struct CustomInstrument {
    pub instrument_name: String,
    pub file_name: String,
    pub key: u8,
    pub press_piano_key: bool,
}
