pub(crate) use instrument::Instrument;

pub mod instrument;

#[derive(Debug, Clone)]
pub struct NoteBlock {
    pub next_tick_jump: i16,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Copy)]
pub struct CommonNote {
    pub instrument: Instrument,
    pub key: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct NewNote {
    pub volume: u8,
    pub panning: u8,
    pub pitch: i16,
}

#[derive(Debug, Clone, Copy)]
pub enum Note {
    Original {
        next_layer_jump: i16,
        common: CommonNote,
    },
    New {
        next_layer_jump: i16,
        common: CommonNote,
        new: NewNote,
    },
}

impl Note {
    #[inline]
    pub const fn get_common(&self) -> &CommonNote {
        match self {
            Self::Original { common, .. } | Self::New { common, .. } => common,
        }
    }

    #[inline]
    pub const fn get_new(&self) -> Option<&NewNote> {
        match self {
            Self::Original { .. } => None,
            Self::New { new, .. } => Some(new),
        }
    }

    #[inline]
    pub const fn get_next_layer_jump(&self) -> &i16 {
        match self {
            Self::Original {
                next_layer_jump, ..
            }
            | Self::New {
                next_layer_jump, ..
            } => next_layer_jump,
        }
    }
}
