#[derive(Debug, Clone)]
pub struct SongMetadata {
    pub song_name: String,
    pub song_author: String,
    pub song_original_author: String,
    pub song_description: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Telemetry {
    pub minutes_spent: u32,
    pub left_clicks: u32,
    pub right_clicks: u32,
    pub note_blocks_added: u32,
    pub note_blocks_removed: u32,
}

#[derive(Debug, Default, Clone)]
pub struct CommonHeader {
    pub layer_count: u16,
    pub metadata: Option<SongMetadata>,
    pub song_tempo: u16,
    pub auto_saving: bool,
    pub auto_saving_duration: u8,
    pub time_signature: u8,
    pub telemetry: Option<Telemetry>,
    pub import_source: Option<String>,
}
