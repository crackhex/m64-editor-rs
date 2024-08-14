use std::array::TryFromSliceError;
use std::ascii::Char as AsciiChar;
use std::fs::File;
use std::io::Read;

pub(crate) struct M64File {
    pub signature: [u8; 4],                 //0x00 4 bytes
    pub version: u32,                       //0x04
    pub uid: i32,                           //0x08
    pub vi_count: u32,                      //0x0C
    pub rerecord_count: u32,                //0x10
    pub vi_per_second: u8,                  //0x14
    pub controller_count: u8,               //0x15
    pub num_samples: u32,                   //0x18
    pub movie_start_type: u16,              //0x1C
    pub controller_flags: u32,              //0x20
    pub internal_name: [AsciiChar; 32],     //0xC4 32 bytes
    pub crc32: u32,                         //0xE4
    pub country_code: u16,                  //0xE8
    pub video_plugin: [AsciiChar; 64],      //0x122 64 bytes
    pub sound_plugin: [AsciiChar; 64],      //0x162 64 bytes
    pub input_plugin: [AsciiChar; 64],      //0x1A2 64 bytes
    pub rsp_plugin: [AsciiChar; 64],        //0x1E2 64 bytes
    pub author: [AsciiChar; 222],           //0x222 220 bytes
    pub movie_desc: [AsciiChar; 256],       //0x300 256 bytes
    pub inputs: Vec<InputLayout>,                   //0x400
}

pub(crate) struct InputLayout {
    pub r_dpad: bool,
    pub l_dpad: bool,
    pub d_dpad: bool,
    pub u_dpad: bool,
    pub start: bool,
    pub z_trig: bool,
    pub b_button: bool,
    pub a_button: bool,
    pub c_right: bool,
    pub c_left: bool,
    pub c_down: bool,
    pub c_up: bool,
    pub r_trig: bool,
    pub l_trig: bool,
    pub x: i8,
    pub y: i8,
}

impl InputLayout {
    fn parse_inputs(inputs: &Vec<u8>) -> Vec<InputLayout> {
        let mut input_layouts: Vec<InputLayout> = vec![];
        for i in (0..inputs.len()).step_by(4) {
            let input = u32::from_le_bytes(inputs[i..i+4].try_into().unwrap());
            let c_right = (input | 0x0001) != 0;
            let c_left = (input | 0x0002) != 0;
            let c_down = (input | 0x0004) != 0;
            let c_up = (input | 0x0008) != 0;
            let r_trig = (input | 0x0010) != 0;
            let l_trig = (input | 0x0020) != 0;
            let r_dpad = (input | 0x0100) != 0;
            let l_dpad = (input | 0x0200) != 0;
            let d_dpad = (input | 0x0400) != 0;
            let u_dpad = (input | 0x0800) != 0;
            let start = (input | 0x1000) != 0;
            let z_trig = (input | 0x2000) != 0;
            let b_button = (input | 0x4000) != 0;
            let a_button = (input | 0x8000) != 0;


            let x = 0; // TODO: Implement x and y
            let y = 0;
            input_layouts.push(InputLayout {
                r_dpad,
                l_dpad,
                d_dpad,
                u_dpad,
                start,
                z_trig,
                b_button,
                a_button,
                c_right,
                c_left,
                c_down,
                c_up,
                r_trig,
                l_trig,
                x,
                y,
            });
        }
        input_layouts
    }
}

impl M64File {
    fn new() -> M64File {
        M64File {
            signature: [0x4D, 0x36, 0x34, 0x1A],
            version: 0x03,
            uid: 0,
            vi_count: 0,
            rerecord_count: 0,
            vi_per_second: 0,
            controller_count: 0,
            num_samples: 0,
            movie_start_type: 0,
            controller_flags: 0,
            internal_name: [0_u8.as_ascii().unwrap(); 32],
            crc32: 0,
            country_code: 0,
            video_plugin: [0_u8.as_ascii().unwrap(); 64],
            sound_plugin: [0_u8.as_ascii().unwrap(); 64],
            input_plugin: [0_u8.as_ascii().unwrap(); 64],
            rsp_plugin: [0_u8.as_ascii().unwrap(); 64],
            author: [0_u8.as_ascii().unwrap(); 222],
            movie_desc: [0_u8.as_ascii().unwrap(); 256],
            inputs: vec![],
        }
    }
    pub fn build_m64(f: &mut File) -> Result<M64File, TryFromSliceError> {
        let m64_len = f.metadata().unwrap().len() as usize;
        if m64_len < 0x400 {
            panic!("File is too small to be a valid M64 file");
        }
        let mut buffer: Vec<u8> = vec![0; m64_len];

        f.read(&mut buffer[..]).expect("TODO: panic message");
        let mut m64 = M64File {
            signature: buffer[0x0..0x4].try_into()?,
            version: u32::from_le_bytes(buffer[0x4..0x8].try_into()?),
            uid: i32::from_le_bytes(buffer[0x8..0xC].try_into()?),
            vi_count: u32::from_le_bytes(buffer[0xC..0x10].try_into()?),
            rerecord_count: u32::from_le_bytes(buffer[0x10..0x14].try_into()?),
            vi_per_second: buffer[0x14],
            controller_count: buffer[0x15],
            num_samples: u32::from_le_bytes(buffer[0x18..0x1C].try_into()?),
            movie_start_type: u16::from_le_bytes(buffer[0x1C..0x1E].try_into()?),
            controller_flags: u32::from_le_bytes(buffer[0x20..0x24].try_into()?),
            internal_name: *<&[u8] as TryInto<[u8; 32]>>::try_into(&buffer[0xC4..0xE4])?.as_ascii().unwrap(),
            crc32: u32::from_le_bytes(buffer[0xE4..0xE8].try_into()?),
            country_code: u16::from_le_bytes(buffer[0xE8..0xEA].try_into()?),
            video_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x122..0x162])?.as_ascii().unwrap(),
            sound_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x162..0x1A2])?.as_ascii().unwrap(),
            input_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x1A2..0x1E2])?.as_ascii().unwrap(),
            rsp_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buffer[0x1E2..0x222])?.as_ascii().unwrap(),
            author: *<&[u8] as TryInto<[u8; 222]>>::try_into(&buffer[0x222..0x300])?.as_ascii().unwrap(),
            movie_desc: *<&[u8] as TryInto<[u8; 256]>>::try_into(&buffer[0x300..0x400])?.as_ascii().unwrap(),
            inputs: InputLayout::parse_inputs(&buffer[0x400..].to_vec()),
        };

        Ok(m64)

    }
}



