use core::str;
use std::str::Utf8Error;

use nom::{bytes::streaming::{tag, take}, combinator::map, number::streaming::{le_u32, le_u8}, sequence::tuple, Parser};

pub type VorbisResult<'a, T> = nom::IResult<&'a [u8], T, ()>;

#[derive(Debug, Clone, Default)]
pub struct VorbisComment {
    pub vendor: String,
    pub comments: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct VorbisInfo {
    pub version: u32,
    pub channels: u8,
    pub sample_rate: u32,
    pub bitrate_maximum: u32,
    pub bitrate_nominal: u32,
    pub bitrate_minimum: u32,
    pub comment: VorbisComment,
}

impl VorbisInfo {
    pub fn new() -> Self {
        Self {
            version: 0,
            channels: 0,
            sample_rate: 0,
            bitrate_maximum: 0,
            bitrate_nominal: 0,
            bitrate_minimum: 0,
            comment: VorbisComment::default(),
        }
    }

    pub fn parse_header_identification(input: &[u8]) -> VorbisResult<VorbisInfo> {
        let (remaining, _) = tag(&b"\x01vorbis"[..])(input)?;
        let (remaining, vorbis_info) = map(
            tuple((le_u32, le_u8, le_u32, le_u32, le_u32, le_u32)),
            |(version, channels, sample_rate, bitrate_maximum, bitrate_nominal, bitrate_minimum)| VorbisInfo {
                version,
                channels,
                sample_rate,
                bitrate_maximum,
                bitrate_minimum,
                bitrate_nominal,
                ..Default::default()
            }).parse(remaining)?;

        Ok((remaining, vorbis_info))
    }

    pub fn parse_header_comment(input: &[u8]) -> VorbisResult<VorbisComment> {
        let (remaining, _) = tag(&b"\x03vorbis"[..])(input)?;
        let (remaining, vendor_len) = le_u32(remaining)?;
        let (remaining, vendor) = take(vendor_len as usize)(remaining)?;

        let (remaining, user_comment_count) = le_u32(remaining)?;

        let mut comments = Vec::new();
        let mut comment_input = remaining;
        for _ in 0..user_comment_count {
            let (remaining, comment_len) = le_u32(comment_input)?;
            let (remaining, comment) = take(comment_len as usize)(remaining)?;
            let comment = str::from_utf8(comment).map_err(|_: Utf8Error| nom::Err::Error(()))?;
            if let Some((key, value)) = comment.split_once('=') {
                comments.push((key.to_string(), value.to_string()));
            };
            comment_input = remaining;
        }

        let vendor = str::from_utf8(vendor).map_err(|_: Utf8Error| nom::Err::Error(()))?.to_string();

        let vorbis_comment = VorbisComment {
            vendor,
            comments,
        };

        Ok((remaining, vorbis_comment))
    }
}
