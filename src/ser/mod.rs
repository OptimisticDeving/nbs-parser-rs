use std::io::ErrorKind;

pub(self) use super::*;
pub(self) use tokio::io::{AsyncRead, AsyncReadExt, copy};
use tokio::io::{AsyncSeek, AsyncSeekExt};

pub mod custom_instrument;
pub mod header;
pub mod layer;
pub mod noteslot;

#[inline]
pub(self) async fn skip_bytes<R: AsyncRead + Unpin>(reader: R, skip_cnt: u64) -> Result<()> {
    copy(&mut reader.take(skip_cnt), &mut tokio::io::sink()).await?;
    Ok(())
}

#[inline]
pub(self) async fn skip_string<R: AsyncRead + Unpin>(mut reader: R) -> Result<()> {
    let length = reader.read_u32_le().await?;
    skip_bytes(&mut reader, length as u64).await
}

#[inline]
pub(self) async fn read_string<R: AsyncRead + Unpin>(mut reader: R) -> Result<String> {
    let length = reader.read_u32_le().await?;
    if length > MAX_STRING_LEN {
        Err(ParseError::TooMuchData("string"))
    } else {
        let mut buf = vec![0u8; length.try_into().map_err(ParseError::format_violated)?];
        reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8_lossy_owned(buf))
    }
}

impl OptionalPartStrategy {
    #[inline]
    pub async fn has_part_async<R: AsyncRead + AsyncSeek + Unpin>(
        &self,
        mut reader: R,
    ) -> Result<bool> {
        Ok(match self {
            OptionalPartStrategy::HasReachedEOF => match reader.read_u8().await {
                Err(e) => match e.kind() {
                    ErrorKind::UnexpectedEof => false,
                    _ => return Err(e.into()),
                },
                _ => {
                    reader.seek(std::io::SeekFrom::Current(-1)).await?;
                    true
                }
            },
            OptionalPartStrategy::Known(known) => *known,
        })
    }
}

impl NBS {
    #[inline]
    pub async fn parse_async<R: AsyncRead + AsyncSeek + Unpin>(
        mut reader: R,
        parse_options: &NBSParseOptions,
    ) -> Result<Self> {
        let header =
            Header::parse_async(&mut reader, &parse_options.common_header_parse_options).await?;
        let common = header.get_common();
        parse_options.verify_header(common)?;

        let mut note_slots = Vec::new();
        let mut remaining_notes = parse_options.max_note_count;

        loop {
            if note_slots.len() == parse_options.max_note_slot_count {
                return Err(ParseError::TooMuchData("note slots"));
            }

            let Some(note_slot) =
                NoteSlot::parse_async(&mut reader, &header, remaining_notes).await?
            else {
                break;
            };

            remaining_notes -= note_slot.notes.len();
            note_slots.push(note_slot);
        }

        let layers = if common.layer_count != 0 {
            let mut layers = Vec::new();

            for _ in 0..common.layer_count {
                layers.push(
                    Layer::parse_async(&mut reader, &header, parse_options.ignore_layer_name)
                        .await?,
                );
            }

            layers
        } else {
            return Ok(Self {
                header,
                note_slots,
                ..Default::default()
            });
        };

        let custom_instruments = if parse_options
            .custom_instrument_part_strategy
            .has_part_async(&mut reader)
            .await?
        {
            let custom_instrument_count = reader.read_u8().await?;
            if custom_instrument_count > parse_options.max_custom_instrument_count {
                return Err(ParseError::TooMuchData("custom instrument"));
            }

            if custom_instrument_count == 0 {
                Vec::new()
            } else {
                let mut custom_instruments = Vec::with_capacity(
                    custom_instrument_count
                        .try_into()
                        .expect("Custom instrument count will always fit into usize"),
                );

                for _ in 0..custom_instrument_count {
                    custom_instruments.push(CustomInstrument::parse_async(&mut reader).await?);
                }

                custom_instruments
            }
        } else {
            Vec::new()
        };

        Ok(Self {
            header,
            note_slots,
            layers,
            custom_instruments,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{
        fs::{File, read_dir},
        io::BufReader,
        test,
    };

    #[test]
    async fn full_test() -> Result<()> {
        let mut dir = read_dir("./tests").await?;
        while let Some(file) = dir.next_entry().await? {
            for skip_metadata in [true, false].into_iter() {
                for skip_telemetry in [true, false].into_iter() {
                    for skip_import_source in [true, false].into_iter() {
                        for ignore_layer_name in [true, false].into_iter() {
                            println!(
                                "Testing {} (skip_metadata: {skip_metadata}, skip_telemetry: {skip_telemetry}, skip_import_source: {skip_import_source}, ignore_layer_name: {ignore_layer_name})",
                                file.path().to_string_lossy()
                            );

                            let parse_options = NBSParseOptions {
                                common_header_parse_options: CommonHeaderParseOptions {
                                    skip_metadata,
                                    skip_telemetry,
                                    skip_import_source,
                                },
                                max_note_count: usize::MAX,
                                max_custom_instrument_count: u8::MAX,
                                max_layer_count: u16::MAX,
                                ignore_layer_name,
                                ..Default::default()
                            };

                            NBS::parse_async(
                                BufReader::new(File::open(&file.path()).await?),
                                &parse_options,
                            )
                            .await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
