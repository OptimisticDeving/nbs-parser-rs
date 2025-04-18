use super::*;

impl NewHeader {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(mut reader: R) -> Result<Self> {
        Ok(Self {
            version: reader.read_u8().await?,
            vanilla_instrument_count: reader.read_u8().await?,
        })
    }
}

impl LoopData {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(mut reader: R) -> Result<Self> {
        Ok(Self {
            loop_enabled: reader.read_u8().await? == 1,
            loop_count: reader.read_u8().await?,
            loop_start: reader.read_u16_le().await?,
        })
    }
}

impl CommonHeader {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(
        mut reader: R,
        parse_options: &CommonHeaderParseOptions,
    ) -> Result<Self> {
        Ok(Self {
            layer_count: reader.read_u16_le().await?,
            metadata: if parse_options.skip_metadata {
                for _ in 0..4 {
                    skip_string(&mut reader).await?;
                }

                None
            } else {
                Some(SongMetadata {
                    song_name: read_string(&mut reader).await?,
                    song_author: read_string(&mut reader).await?,
                    song_original_author: read_string(&mut reader).await?,
                    song_description: read_string(&mut reader).await?,
                })
            },
            song_tempo: reader.read_u16_le().await?,
            auto_saving: reader.read_u8().await? == 1,
            auto_saving_duration: reader.read_u8().await?,
            time_signature: reader.read_u8().await?,
            telemetry: if parse_options.skip_telemetry {
                // Skip across 20 bytes (4 for each telemetry field)
                skip_bytes(&mut reader, 20).await?;

                None
            } else {
                Some(Telemetry {
                    minutes_spent: reader.read_u32_le().await?,
                    left_clicks: reader.read_u32_le().await?,
                    right_clicks: reader.read_u32_le().await?,
                    note_blocks_added: reader.read_u32_le().await?,
                    note_blocks_removed: reader.read_u32_le().await?,
                })
            },
            import_source: if parse_options.skip_import_source {
                skip_string(&mut reader).await?;

                None
            } else {
                Some(read_string(&mut reader).await?)
            },
        })
    }
}

impl Header {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(
        mut reader: R,
        parse_options: &CommonHeaderParseOptions,
    ) -> Result<Self> {
        let old_song_length = reader.read_u16_le().await?;

        Ok(if old_song_length != 0 {
            Self::Original {
                length: old_song_length,
                common: CommonHeader::parse_async(reader, parse_options).await?,
            }
        } else {
            let new = NewHeader::parse_async(&mut reader).await?;

            if new.version >= 3 {
                Self::NewWithLength {
                    new,
                    length: reader.read_u16_le().await?,
                    common: CommonHeader::parse_async(&mut reader, parse_options).await?,
                    loop_data: if new.version >= 4 {
                        Some(LoopData::parse_async(reader).await?)
                    } else {
                        None
                    },
                }
            } else {
                Self::New {
                    new,
                    common: CommonHeader::parse_async(&mut reader, parse_options).await?,
                }
            }
        })
    }
}
