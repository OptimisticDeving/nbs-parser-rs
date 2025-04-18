use super::*;

impl OriginalLayer {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(mut reader: R, name: String) -> Result<Self> {
        Ok(Self {
            name,
            volume: reader.read_u8().await?,
        })
    }
}

impl NewLayer {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(
        mut reader: R,
        header: &Header,
        name: String,
    ) -> Result<Self> {
        Ok(Self {
            name,
            is_locked: if header.get_version() >= 4 {
                reader.read_u8().await? == 1
            } else {
                false
            },
            volume: reader.read_u8().await?,
            stereo: reader.read_u8().await?,
        })
    }
}

impl Layer {
    #[inline]
    pub async fn parse_async<R: AsyncRead + Unpin>(mut reader: R, header: &Header) -> Result<Self> {
        let name = read_string(&mut reader).await?;

        if header.get_version() <= 1 {
            return Ok(Self::Original(
                OriginalLayer::parse_async(reader, name).await?,
            ));
        }

        Ok(Self::New(
            NewLayer::parse_async(reader, header, name).await?,
        ))
    }
}
