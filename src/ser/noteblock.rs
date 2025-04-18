use super::*;

impl CommonNote {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(mut reader: R) -> Result<Self> {
        Ok(Self {
            instrument: Instrument::from_byte(reader.read_u8().await?),
            key: reader.read_u8().await?,
        })
    }
}

impl NewNote {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(mut reader: R) -> Result<Self> {
        Ok(Self {
            volume: reader.read_u8().await?,
            panning: reader.read_u8().await?,
            pitch: reader.read_i16_le().await?,
        })
    }
}

impl Note {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(
        mut reader: R,
        header: &Header,
        next_layer_jump: i16,
    ) -> Result<Self> {
        let common = CommonNote::parse_async(&mut reader).await?;

        Ok(if header.get_version() >= 4 {
            Self::New {
                next_layer_jump,
                common,
                new: NewNote::parse_async(reader).await?,
            }
        } else {
            Self::Original {
                next_layer_jump,
                common,
            }
        })
    }
}

impl NoteBlock {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(
        mut reader: R,
        header: &Header,
        max_notes: usize,
    ) -> Result<Option<Self>> {
        let next_tick_jump = reader.read_i16_le().await?;

        if next_tick_jump == 0 {
            return Ok(None);
        }

        let mut notes = Vec::new();

        loop {
            let next_layer_jump = reader.read_i16_le().await?;
            if next_layer_jump == 0 {
                break Ok(Some(Self {
                    next_tick_jump,
                    notes,
                }));
            }

            if notes.len() == max_notes {
                return Err(ParseError::TooMuchData("notes"));
            }

            notes.push(Note::parse_async(&mut reader, header, next_layer_jump).await?);
        }
    }
}
