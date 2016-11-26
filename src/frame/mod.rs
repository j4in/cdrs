//! `frame` module contains general Frame functionality.
use std::convert::{From};
use super::types::to_n_bytes;
use super::{AsByte, IntoBytes};
use self::frame_response::ResponseBody;

/// Number of version bytes in accordance to protocol.
pub const VERSION_LEN: usize = 1;
/// Number of flag bytes in accordance to protocol.
pub const FLAG_LEN: usize = 1;
/// Number of opcode bytes in accordance to protocol.
pub const OPCODE_LEN: usize = 1;
/// Number of stream bytes in accordance to protocol.
pub const STREAM_LEN: usize = 2;
/// Number of body length bytes in accordance to protocol.
pub const LENGTH_LEN: usize = 4;

pub mod frame_options;
pub mod frame_query;
pub mod frame_ready;
pub mod frame_response;
pub mod frame_result;
pub mod frame_startup;
pub mod parser;

#[derive(Debug)]
pub struct Frame {
    pub version: Version,
    pub flag: Flag,
    pub opcode: Opcode,
    pub stream: u64, // we're going to use 0 here until async client is implemented
    pub body: Vec<u8> // change type to Vec<u8>
}

impl Frame {
    pub fn get_body(&self) -> ResponseBody {
        return ResponseBody::from(self.body.clone(), &self.opcode);
    }
}

impl<'a> IntoBytes for Frame {
    fn into_cbytes(&self) -> Vec<u8> {
        let mut v = vec![];

        let version_bytes = self.version.as_byte();
        let flag_bytes = self.flag.as_byte();
        let opcode_bytes = self.opcode.as_byte();
        let body_len = self.body.len();

        v.push(version_bytes);
        v.push(flag_bytes);
        v.extend_from_slice(to_n_bytes(self.stream, STREAM_LEN).as_slice());
        v.push(opcode_bytes);
        v.extend_from_slice(to_n_bytes(body_len as u64, LENGTH_LEN).as_slice());
        v.extend_from_slice(self.body.as_slice());

        return v;
    }
}

/// Frame's version
#[derive(Debug)]
pub enum Version {
    Request,
    Response
}

impl AsByte for Version {
    fn as_byte(&self) -> u8 {
        return match self {
            &Version::Request => 0x04,
            &Version::Response => 0x84
        }
    }
}

impl From<Vec<u8>> for Version {
    fn from(v: Vec<u8>) -> Version {
        if v.len() != VERSION_LEN {
            error!("Unexpected Cassandra verion. Should has {} byte(-s), got {:?}", VERSION_LEN, v);
            panic!("Unexpected Cassandra verion. Should has {} byte(-s), got {:?}", VERSION_LEN, v);
        }
        return match v[0] {
            0x04 => Version::Request,
            0x84 => Version::Response,
            _ => {
                error!("Unexpected Cassandra version {:?}", v);
                panic!("Unexpected Cassandra version {:?}", v);
            }
        }
    }
}

/// Frame's flag
// Is not implemented functionality. Only Igonore works for now
#[derive(Debug)]
pub enum Flag {
    Compression,
    Tracing,
    CustomPayload,
    Warning,
    Ignore
}

impl AsByte for Flag {
    fn as_byte(&self) -> u8 {
        return match self {
            &Flag::Compression => 0x01,
            &Flag::Tracing => 0x02,
            &Flag::CustomPayload => 0x04,
            &Flag::Warning => 0x08,
            &Flag::Ignore => 0x00 // assuming that ingoring value whould be other than [0x01, 0x02, 0x04, 0x08]
         }
    }
}

impl From<Vec<u8>> for Flag {
    fn from(f: Vec<u8>) -> Flag {
        if f.len() != FLAG_LEN {
            panic!("Unexpected Cassandra flag. Should has {} byte(-s), got {:?}", FLAG_LEN, f);
        }
        return match f[0] {
            0x01 => Flag::Compression,
            0x02 => Flag::Tracing,
            0x04 => Flag::CustomPayload,
            0x08 => Flag::Warning,
            _ => Flag::Ignore // ignore by specification
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Error,
    Startup,
    Ready,
    Authenticate,
    Options,
    Supported,
    Query,
    Result,
    Prepare,
    Execute,
    Register,
    Event,
    Batch,
    AuthChallenge,
    AuthResponse,
    AuthSuccess
}

impl AsByte for Opcode {
    fn as_byte(&self) -> u8 {
        return match self {
            &Opcode::Error => 0x00,
            &Opcode::Startup => 0x01,
            &Opcode::Ready => 0x02,
            &Opcode::Authenticate => 0x03,
            &Opcode::Options => 0x05,
            &Opcode::Supported => 0x06,
            &Opcode::Query => 0x07,
            &Opcode::Result => 0x08,
            &Opcode::Prepare => 0x09,
            &Opcode::Execute => 0x0A,
            &Opcode::Register => 0x0B,
            &Opcode::Event => 0x0C,
            &Opcode::Batch => 0x0D,
            &Opcode::AuthChallenge => 0x0E,
            &Opcode::AuthResponse => 0x0F,
            &Opcode::AuthSuccess => 0x10
        }
    }
}

impl From<Vec<u8>> for Opcode {
    fn from(oc: Vec<u8>) -> Opcode {
        if oc.len() != OPCODE_LEN {
            panic!("Unexpected Cassandra opcode. Should has {} byte(-s), got {:?}", OPCODE_LEN, oc);
        }

        return match oc[0] {
            0x00 => Opcode::Error,
            0x01 => Opcode::Startup,
            0x02 => Opcode::Ready,
            0x03 => Opcode::Authenticate,
            0x05 => Opcode::Options,
            0x06 => Opcode::Supported,
            0x07 => Opcode::Query,
            0x08 => Opcode::Result,
            0x09 => Opcode::Prepare,
            0x0A => Opcode::Execute,
            0x0B => Opcode::Register,
            0x0C => Opcode::Event,
            0x0D => Opcode::Batch,
            0x0E => Opcode::AuthChallenge,
            0x0F => Opcode::AuthResponse,
            0x10 => Opcode::AuthSuccess,
            _ => {
                error!("Unexpected Cassandra opcode {:?}", oc);
                panic!("Unexpected Cassandra opcode {:?}", oc);
            }
        }
    }
}