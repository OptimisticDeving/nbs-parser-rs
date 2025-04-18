#[derive(Debug, Clone)]
pub struct OriginalLayer {
    pub name: Option<String>,
    pub volume: u8,
}

#[derive(Debug, Clone)]
pub struct NewLayer {
    pub name: Option<String>,
    pub is_locked: bool,
    pub volume: u8,
    pub stereo: u8,
}

#[derive(Debug, Clone)]
pub enum Layer {
    Original(OriginalLayer),
    New(NewLayer),
}

impl Layer {
    #[inline]
    pub const fn get_name(&self) -> &Option<String> {
        match self {
            Self::Original(original_layer) => &original_layer.name,
            Self::New(new_layer) => &new_layer.name,
        }
    }

    #[inline]
    pub const fn get_volume(&self) -> &u8 {
        match self {
            Self::Original(original_layer) => &original_layer.volume,
            Self::New(new_layer) => &new_layer.volume,
        }
    }
}
