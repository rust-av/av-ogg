use av_format::muxer::Muxer;

pub struct OggMuxer {}

impl Muxer for OggMuxer {
    fn configure(&mut self) -> av_format::error::Result<()> {
        todo!()
    }

    fn write_header<W: std::io::Write>(&mut self, out: &mut av_format::muxer::Writer<W>) -> av_format::error::Result<()> {
        todo!()
    }

    fn write_packet<W: std::io::Write>(&mut self, out: &mut av_format::muxer::Writer<W>, pkt: std::sync::Arc<av_data::packet::Packet>) -> av_format::error::Result<()> {
        todo!()
    }

    fn write_trailer<W: std::io::Write>(&mut self, out: &mut av_format::muxer::Writer<W>) -> av_format::error::Result<()> {
        todo!()
    }

    fn set_global_info(&mut self, info: av_format::common::GlobalInfo) -> av_format::error::Result<()> {
        todo!()
    }

    fn set_option(&mut self, key: &str, val: av_data::value::Value) -> av_format::error::Result<()> {
        todo!()
    }
}
