use std::sync::Arc;

use av_data::{
    audiosample::{formats, ChannelMap},
    params::{AudioInfo, CodecParams, MediaKind},
    rational::Rational64,
};
use av_format::{demuxer::Demuxer, error::Error, stream::Stream};
use nom::Offset;

use crate::parser::OggPage;

use av_vorbis::parser::VorbisInfo;

pub struct OggDemuxer {}

impl OggDemuxer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Demuxer for OggDemuxer {
    fn read_headers(
        &mut self,
        buf: &mut dyn av_format::buffer::Buffered,
        info: &mut av_format::common::GlobalInfo,
    ) -> av_format::error::Result<std::io::SeekFrom> {
        let mut input = buf.data();

        let (reamining, page) =
            OggPage::parse_headers(&mut input).map_err(|_| Error::InvalidData)?;
        input = reamining;

        if page.header_type_flag & 0x02 != 0 {
            let (_, segments) = OggPage::parse_segments(&mut input, &page.segments_table)
                .map_err(|_| Error::InvalidData)?;

            if let Ok((reamining, vorbis_info)) =
                VorbisInfo::parse_header_identification(segments).map_err(|_| Error::InvalidData)
            {
                let VorbisInfo {
                    channels,
                    sample_rate,
                    bitrate_maximum,
                    bitrate_nominal,
                    bitrate_minimum,
                    ..
                } = vorbis_info;
                let bit_rate = if bitrate_nominal == 0 {
                    (bitrate_maximum + bitrate_minimum) / 2
                } else {
                    bitrate_nominal
                } as usize;
                let rate = sample_rate as usize;
                let channels = channels as usize;

                let params = CodecParams {
                    kind: Some(MediaKind::Audio(AudioInfo {
                        rate,
                        map: Some(ChannelMap::default_map(channels)),
                        format: Some(Arc::new(formats::F32)),
                    })),
                    codec_id: Some("vorbis".to_string()),
                    extradata: None,
                    bit_rate,
                    convergence_window: 0,
                    delay: 0,
                };

                let timebase = Rational64::new(1, rate as i64);

                let stream = Stream::from_params(&params, timebase);
                info.streams.push(stream);
                input = reamining;
            };
        }

        Ok(std::io::SeekFrom::Current(buf.data().offset(input) as i64))
    }

    fn read_event(
        &mut self,
        buf: &mut dyn av_format::buffer::Buffered,
    ) -> av_format::error::Result<(std::io::SeekFrom, av_format::demuxer::Event)> {
        todo!()
    }
}

mod tests {
    use std::fs::File;
    use std::io::{BufReader, Error, SeekFrom};

    use av_format::buffer::{AccReader, Buffered};
    use av_format::demuxer::{Context, Demuxer};

    use crate::demuxer::OggDemuxer;

    #[test]
    fn test_read_vorbis() -> Result<(), ()> {
        let file = File::open("../assets/DSCF0002.oga").map_err(|err| {
            println!("{:?}", err);
            ()
        })?;
        let reader = AccReader::new(BufReader::new(file));
        let demuxer = OggDemuxer::new();

        let mut context = Context::new(demuxer, reader);

        let _ = context.read_headers().map_err(|err| {
            println!("{:?}", err);
            ()
        })?;

        println!("{:?}", context.info);

        Ok(())
    }

    #[test]
    fn test_read_theora_and_vorbis() -> Result<(), ()> {
        let file = File::open("../assets/DSCF0002.ogg").map_err(|err| {
            println!("{:?}", err);
            ()
        })?;
        let reader = AccReader::new(BufReader::new(file));
        let demuxer = OggDemuxer::new();

        let mut context = Context::new(demuxer, reader);

        let _ = context.read_headers().map_err(|err| {
            println!("{:?}", err);
            ()
        })?;

        println!("{:?}", context.info);

        Ok(())
    }

    #[test]
    fn test_read_theora() -> Result<(), ()> {
        let file = File::open("../assets/DSCF0002.ogv").map_err(|err| {
            println!("{:?}", err);
            ()
        })?;
        let reader = AccReader::new(BufReader::new(file));
        let demuxer = OggDemuxer::new();

        let mut context = Context::new(demuxer, reader);

        let _ = context.read_headers().map_err(|err| {
            println!("{:?}", err);
            ()
        })?;

        println!("{:?}", context.info);

        Ok(())
    }

    #[test]
    fn test_read_opus() -> Result<(), ()> {
        let file = File::open("../assets/DSCF0002.opus").map_err(|err| {
            println!("{:?}", err);
            ()
        })?;
        let reader = AccReader::new(BufReader::new(file));
        let demuxer = OggDemuxer::new();

        let mut context = Context::new(demuxer, reader);

        let _ = context.read_headers().map_err(|err| {
            println!("{:?}", err);
            ()
        })?;

        println!("{:?}", context.info);

        Ok(())
    }

    #[test]
    fn test_read_flac() -> Result<(), ()> {
        let file = File::open("../assets/DSCF0002_fLaC.oga").map_err(|err| {
            println!("{:?}", err);
            ()
        })?;
        let reader = AccReader::new(BufReader::new(file));
        let demuxer = OggDemuxer::new();

        let mut context = Context::new(demuxer, reader);

        let _ = context.read_headers().map_err(|err| {
            println!("{:?}", err);
            ()
        })?;

        println!("{:?}", context.info);

        Ok(())
    }
}
