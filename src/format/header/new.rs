#[derive(Debug, Clone, Copy)]
pub struct NewHeader {
    pub version: u8,
    pub vanilla_instrument_count: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct LoopData {
    pub loop_enabled: bool,
    pub loop_count: u8,
    pub loop_start: u16,
}
