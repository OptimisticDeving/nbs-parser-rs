pub mod common;
pub mod new;

pub(crate) use {common::*, new::*};

#[derive(Debug, Clone)]
pub enum Header {
    Original {
        length: u16,
        common: CommonHeader,
    },
    New {
        new: NewHeader,
        common: CommonHeader,
    },
    NewWithLength {
        new: NewHeader,
        length: u16,
        common: CommonHeader,
        loop_data: Option<LoopData>,
    },
}

impl Default for Header {
    fn default() -> Self {
        Self::Original {
            length: 0,
            common: CommonHeader::default(),
        }
    }
}

impl Header {
    #[inline]
    pub const fn has_length(&self) -> bool {
        match self {
            Self::Original { .. } | Self::NewWithLength { .. } => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn is_new(&self) -> bool {
        match self {
            Header::Original { .. } => false,
            _ => true,
        }
    }

    #[inline]
    pub const fn get_length(&self) -> Option<&u16> {
        match self {
            Self::Original { length, .. } | Self::NewWithLength { length, .. } => Some(length),
            Self::New { .. } => None,
        }
    }

    #[inline]
    pub const fn get_common(&self) -> &CommonHeader {
        match self {
            Self::Original { common, .. }
            | Self::New { common, .. }
            | Self::NewWithLength { common, .. } => common,
        }
    }

    #[inline]
    pub const fn get_new(&self) -> Option<&NewHeader> {
        match self {
            Self::Original { .. } => None,
            Self::New { new, .. } | Self::NewWithLength { new, .. } => Some(new),
        }
    }

    #[inline]
    pub const fn get_version(&self) -> u8 {
        match self {
            Self::Original { .. } => 0,
            Self::New { new: new_start, .. } | Self::NewWithLength { new: new_start, .. } => {
                new_start.version
            }
        }
    }
}
