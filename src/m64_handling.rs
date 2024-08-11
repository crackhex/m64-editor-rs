use std::fs::File;
use std::io::Read;

pub(crate) struct M64File {
    header: M64Header,
    inputs: i32,
}
#[derive(Debug)]
struct M64Header {
    signature: [u8; 4],            //0x00 4 bytes
    version: u32,               //0x04
    movie_uid: i32,             //0x08
    movie_length: u32,          //0x0C
    rerecord_count: u32,        //0x10
    vi_per_second: u8,          //0x14 UNUSED
    num_controllers: u8,        //0x15
    unused1: u16,               //0x16 UNUSED
    num_samples: u32,           //0x18
    movie_start_type: u16,      //0x1C
    unused2: u16,               //0x1E UNUSED
    controller_flags: u32,      //0x20
    unused3: [u8; 160],              //0x24 UNUSED 160 bytes
    internal_name: [u8; 32],         //0xC4 32 bytes
    crc32: u32,                 //0xE4
    country_code: u16,          //0xE8
    unused4: [u8; 56],              //0xEA UNUSED 56 bytes
    video_plugin: [u8; 64],         //0x122 64 bytes
    sound_plugin: [u8; 64],     //0x162 64 bytes
    input_plugin: [u8; 64],     //0x1A2 64 bytes
    rsp_plugin: [u8; 64],       //0x1E2 64 bytes
    author: [u8; 222],          //0x222 220 bytes
    movie_desc: [u8; 256],      //0x2FE 256 bytes
}
impl M64File {
    fn new() -> M64File {
        M64File {
            header: M64Header::new(),
            inputs: 0,
        }
    }
    pub(crate) fn build_m64(f: &mut File) -> M64File {
        let mut m64 = M64File::new();
        let mut header = M64Header::build_header(f);
        m64.header = header;
        println!("{:?}", &m64.header.author);
        m64
    }
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
            internal_name: [0; 32],
            crc32: 0,
            country_code: 0,
            unused4: [0; 56],
            video_plugin: [0; 64],
            sound_plugin: [0; 64],
            input_plugin: [0; 64],
            rsp_plugin: [0; 64],
            author: [0; 222],
            movie_desc: [0; 256],
        }
    }

    fn build_header(f: &mut File) -> M64Header {
        let mut buffer: [u8; 1024] = [0; 1024];
        f.read(&mut buffer).expect("TODO: panic message");
        let mut header = M64Header {
            signature: buffer[0x0..0x4].try_into().expect("TODO: panic message"),
            version: u32::from_le_bytes(buffer[0x4..0x8].try_into().expect("TODO: panic message")),
            movie_uid: i32::from_le_bytes(buffer[0x8..0xC].try_into().expect("TODO: panic message")),
            movie_length: u32::from_le_bytes(buffer[0xC..0x10].try_into().expect("TODO: panic message")),
            rerecord_count: u32::from_le_bytes(buffer[0x10..0x14].try_into().expect("TODO: panic message")),
            vi_per_second: buffer[0x14],
            num_controllers: buffer[0x15],
            unused1: u16::from_le_bytes(buffer[0x16..0x18].try_into().expect("TODO: panic message")),
            num_samples: u32::from_le_bytes(buffer[0x18..0x1C].try_into().expect("TODO: panic message")),
            movie_start_type: u16::from_le_bytes(buffer[0x1C..0x1E].try_into().expect("TODO: panic message")),
            unused2: u16::from_le_bytes(buffer[0x1E..0x20].try_into().expect("TODO: panic message")),
            controller_flags: u32::from_le_bytes(buffer[0x20..0x24].try_into().expect("TODO: panic message")),
            unused3: buffer[0x24..0xC4].try_into().expect("TODO: panic message"),
            internal_name: buffer[0xC4..0xE4].try_into().expect("TODO: panic message"),
            crc32: u32::from_le_bytes(buffer[0xE4..0xE8].try_into().expect("TODO: panic message")),
            country_code: u16::from_le_bytes(buffer[0xE8..0xEA].try_into().expect("TODO: panic message")),
            unused4: buffer[0xEA..0x122].try_into().expect("TODO: panic message"),
            video_plugin:  buffer[0x122..0x162].try_into().expect("TODO: panic message"),
            sound_plugin: buffer[0x162..0x1A2].try_into().expect("TODO: panic message"),
            input_plugin: buffer[0x1A2..0x1E2].try_into().expect("TODO: panic message"),
            rsp_plugin: buffer[0x1E2..0x222].try_into().expect("TODO: panic message"),
            author: buffer[0x222..0x300].try_into().expect("TODO: panic message"),
            movie_desc: buffer[0x300..1024].try_into().expect("TODO: panic message"),
        };
        header
    }
}

