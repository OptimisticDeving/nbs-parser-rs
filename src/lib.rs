#![feature(string_from_utf8_lossy_owned)]

pub(crate) const MAX_STRING_LEN: u32 = 8192;

pub mod format;
pub mod ser;

use format::*;
use std::fmt::{Debug, Display};

use thiserror::Error;

pub trait GenericError: Display + Debug + Send + Sync {}

impl<T: Display + Debug + Send + Sync> GenericError for T {}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Too much data: {0}")]
    TooMuchData(&'static str),
    #[error("Data format violated: '{0}'")]
    FormatViolated(Box<dyn GenericError>),
}

impl ParseError {
    #[inline]
    fn format_violated<T: Display + Debug + Send + Sync + 'static>(err: T) -> Self {
        Self::FormatViolated(Box::new(err))
    }
}

pub(crate) type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone, Copy, Default)]
pub struct CommonHeaderParseOptions {
    pub skip_metadata: bool,
    pub skip_telemetry: bool,
    pub skip_import_source: bool,
}

#[derive(Debug, Clone, Default)]
pub struct NBS {
    pub header: Header,
    pub note_slots: Vec<NoteSlot>,
    pub layers: Vec<Layer>,
    pub custom_instruments: Vec<CustomInstrument>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum OptionalPartStrategy {
    #[default]
    HasReachedEOF,
    Known(bool),
}

#[derive(Debug, Clone, Copy)]
pub struct NBSParseOptions {
    pub common_header_parse_options: CommonHeaderParseOptions,
    pub max_note_count: usize,
    pub max_note_slot_count: usize,
    pub max_layer_count: u16,
    pub max_custom_instrument_count: u8,
    pub custom_instrument_part_strategy: OptionalPartStrategy,
    pub ignore_layer_name: bool,
}

impl NBSParseOptions {
    #[inline]
    pub fn verify_header(&self, common_header: &CommonHeader) -> Result<()> {
        if common_header.layer_count > self.max_layer_count {
            Err(ParseError::TooMuchData("layer count"))
        } else {
            Ok(())
        }
    }
}

impl Default for NBSParseOptions {
    fn default() -> Self {
        Self {
            common_header_parse_options: Default::default(),
            max_note_count: 65535,
            max_note_slot_count: 16384,
            max_layer_count: 256,
            max_custom_instrument_count: 128,
            custom_instrument_part_strategy: Default::default(),
            ignore_layer_name: false,
        }
    }
}
