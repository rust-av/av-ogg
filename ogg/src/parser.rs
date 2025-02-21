use nom::{
    branch::alt,
    bytes::streaming::{tag, take},
    combinator::{eof, map, value},
    number::streaming::{le_u32, le_u64, le_u8},
    Err, Parser,
};

#[derive(Debug, Clone, Default)]
pub struct OggPage<'a> {
    pub capture_pattern: u32,
    pub stream_structure_version: u8,
    pub header_type_flag: u8,
    pub absolute_granule_position: u64,
    pub stream_serial_number: u32,
    pub page_sequence_number: u32,
    pub page_checksum: u32,
    pub page_segments: u8,
    pub segments_table: &'a [u8],
    pub segments: &'a [u8],
}

impl<'a> OggPage<'a> {
    pub fn parse_headers(input: &'a [u8]) -> Result<(&'a [u8], OggPage<'a>), ()> {
        if input.len() < 27 {
            return Err(());
        }

        let (remaining, capture_pattern) = le_u32(input).map_err(|_: Err<(), ()>| ())?;
        let (remaining, stream_structure_version) =
            le_u8(remaining).map_err(|_: Err<(), ()>| ())?;
        let (remaining, header_type_flag) = le_u8(remaining).map_err(|_: Err<(), ()>| ())?;
        let (remaining, absolute_granule_position) =
            le_u64(remaining).map_err(|_: Err<(), ()>| ())?;
        let (remaining, stream_serial_number) = le_u32(remaining).map_err(|_: Err<(), ()>| ())?;
        let (remaining, page_sequence_number) = le_u32(remaining).map_err(|_: Err<(), ()>| ())?;
        let (remaining, page_checksum) = le_u32(remaining).map_err(|_: Err<(), ()>| ())?;
        let (remaining, page_segments) = le_u8(remaining).map_err(|_: Err<(), ()>| ())?;
        let (remaining, segments_table) =
            take(page_segments)(remaining).map_err(|_: Err<(), ()>| ())?;

        Ok((
            remaining,
            OggPage {
                capture_pattern,
                stream_structure_version,
                header_type_flag,
                absolute_granule_position,
                stream_serial_number,
                page_sequence_number,
                page_checksum,
                page_segments,
                segments_table,
                segments: &[],
            },
        ))
    }

    pub fn parse_segments(
        input: &'a [u8],
        segments_table: &'a [u8],
    ) -> Result<(&'a [u8], &'a [u8]), ()> {
        let segments_len: usize = segments_table.iter().map(|&x| x as usize).sum();

        let (remaining, segments) = take(segments_len)(input).map_err(|_: Err<(), ()>| ())?;

        Ok((remaining, segments))
    }

    pub fn parse_codec(input: &'a [u8]) -> Result<(&'a [u8], String), ()> {
        alt((
            map(tag(&b"\x01vorbis"[..]), |_| "vorbis".to_string()),
            map(tag(&b"OpusHead"[..]), |_| "opus".to_string()),
            map(tag(&b"\x80theora"[..]), |_| "theora".to_string()),
            map(tag(&b"\x7FFLAC"[..]), |_| "flac".to_string()),
            value("unknown".to_string(), eof),
        ))
        .parse(input)
        .map_err(|_: Err<(), ()>| ())
    }
}

mod tests {
    use av_bitstream::byteread::get_u32l;

    use super::OggPage;

    #[test]
    fn test_parse_headers_unsufficient_input() -> Result<(), ()> {
        let input = include_bytes!("../../assets/DSCF0002.oga");

        let page = OggPage::parse_headers(&input[0..26]);

        assert!(page.is_err());

        Ok(())
    }

    #[test]
    fn test_parse_headers() -> Result<(), ()> {
        let input = include_bytes!("../../assets/DSCF0002.oga");

        let (_, page) = OggPage::parse_headers(input)?;

        assert_eq!(page.capture_pattern, get_u32l(b"OggS"));

        Ok(())
    }
}
