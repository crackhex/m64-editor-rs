use std::array::TryFromSliceError;
use std::ascii::Char as AsciiChar;
use std::fs::File;
use std::io::Read;

pub(crate) struct M64File {
    pub header: M64Header,
    pub inputs: i32,
}

impl M64File {
    fn new() -> M64File {
        M64File {
            header: M64Header::new(),
            inputs: 0,
        }
    }
    pub fn build_m64(f: &mut File) -> Result<M64File, TryFromSliceError> {
        let mut m64 = M64File::new();
        m64.header = M64Header::build_header(f)?;
        Ok(m64)
    }
}

pub(crate) struct M64Header {
    pub signature: [u8; 4],         //0x00 4 bytes
    pub version: u32,               //0x04
    pub movie_uid: i32,             //0x08
    pub movie_length: u32,          //0x0C
    pub rerecord_count: u32,        //0x10
    pub vi_per_second: u8,          //0x14 UNUSED
    pub num_controllers: u8,        //0x15
    pub unused1: u16,               //0x16 UNUSED
    pub num_samples: u32,           //0x18
    pub movie_start_type: u16,      //0x1C
    pub unused2: u16,               //0x1E UNUSED
    pub controller_flags: u32,      //0x20
    pub unused3: [u8; 160],         //0x24 UNUSED 160 bytes
    pub internal_name: [AsciiChar; 32],    //0xC4 32 bytes
    pub crc32: u32,                 //0xE4
    pub country_code: u16,          //0xE8
    pub unused4: [u8; 56],          //0xEA UNUSED 56 bytes
    pub video_plugin: [AsciiChar; 64],     //0x122 64 bytes
    pub sound_plugin: [AsciiChar; 64],     //0x162 64 bytes
    pub input_plugin: [AsciiChar; 64],     //0x1A2 64 bytes
    pub rsp_plugin: [AsciiChar; 64],       //0x1E2 64 bytes
    pub author: [AsciiChar; 222],          //0x222 220 bytes
    pub movie_desc: [AsciiChar; 256],      //0x300 256 bytes
}

impl M64Header {
    fn new() -> M64Header {
        M64Header {
            signature: [0x4D, 0x36, 0x34, 0x1A],
            version: 0x03,
            movie_uid: 0,
            movie_length: 0,
            rerecord_count: 0,
            vi_per_second: 0,
            num_controllers: 0,
            unused1: 0,
            num_samples: 0,
            movie_start_type: 0,
            unused2: 0,
            controller_flags: 0,
            unused3: [0; 160],
            internal_name: [0_u8.as_ascii().unwrap(); 32],
            crc32: 0,
            country_code: 0,
            unused4: [0; 56],
            video_plugin: [0_u8.as_ascii().unwrap(); 64],
            sound_plugin: [0_u8.as_ascii().unwrap(); 64],
            input_plugin: [0_u8.as_ascii().unwrap(); 64],
            rsp_plugin: [0_u8.as_ascii().unwrap(); 64],
            author: [0_u8.as_ascii().unwrap(); 222],
            movie_desc: [0_u8.as_ascii().unwrap(); 256],
        }
    }
    fn build_header(f: &mut File) -> Result<M64Header, TryFromSliceError> {
        let mut buffer: [u8; 1024] = [0; 1024];
        f.read(&mut buffer[..]).expect("TODO: panic message");
        let header = M64Header {
            signature: buffer[0x0..0x4].try_into()?,
            version: u32::from_le_bytes(buffer[0x4..0x8].try_into()?),
            movie_uid: i32::from_le_bytes(buffer[0x8..0xC].try_into()?),
            movie_length: u32::from_le_bytes(buffer[0xC..0x10].try_into()?),
            rerecord_count: u32::from_le_bytes(buffer[0x10..0x14].try_into()?),
            vi_per_second: buffer[0x14],
            num_controllers: buffer[0x15],
            unused1: u16::from_le_bytes(buffer[0x16..0x18].try_into()?),
            num_samples: u32::from_le_bytes(buffer[0x18..0x1C].try_into()?),
            movie_start_type: u16::from_le_bytes(buffer[0x1C..0x1E].try_into()?),
            unused2: u16::from_le_bytes(buffer[0x1E..0x20].try_into()?),
            controller_flags: u32::from_le_bytes(buffer[0x20..0x24].try_into()?),
            unused3: buffer[0x24..0xC4].try_into()?,
            internal_name: *<&[u8] as TryInto<[u8; 32]>>::try_into(&buffer[0xC4..0xE4])?.as_ascii().unwrap(),
            crc32: u32::from_le_bytes(buffer[0xE4..0xE8].try_into()?),
            country_code: u16::from_le_bytes(buffer[0xE8..0xEA].try_into()?),
            unused4: buffer[0xEA..0x122].try_into()?,
            video_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x122..0x162])?.as_ascii().unwrap(),
            sound_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x162..0x1A2])?.as_ascii().unwrap(),
            input_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x1A2..0x1E2])?.as_ascii().unwrap(),
            rsp_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x1E2..0x222])?.as_ascii().unwrap(),
            author: *<&[u8] as TryInto<[u8; 222]>>::try_into(&buffer[0x222..0x300])?.as_ascii().unwrap(),
            movie_desc: *<&[u8] as TryInto<[u8; 256]>>::try_into(&buffer[0x300..0x400])?.as_ascii().unwrap(),
        };
        Ok(header)

    }
}


