mod mm;
pub use mm::{ExtProtoDiscriminator, Nas5gMmMessageHeader, Nas5gSecurityHeader};

#[derive(Debug, Clone)]
pub enum NasMessageHeader {
    Mm(Nas5gMmMessageHeader),
}

impl NasMessageHeader {
    pub fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        if data.is_empty() {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Empty Data"))
        } else {
            match data[0] {
                0x7E => {
                    let (hdr, decoded) = Nas5gMmMessageHeader::decode(data)?;
                    Ok((Self::Mm(hdr), decoded))
                }
                _ => todo!(),
            }
        }
    }
}
