use super::*;

impl CustomInstrument {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(mut reader: R) -> Result<Self> {
        Ok(Self {
            instrument_name: read_string(&mut reader).await?,
            file_name: read_string(&mut reader).await?,
            key: reader.read_u8().await?,
            press_piano_key: reader.read_u8().await? == 1,
        })
    }
}
