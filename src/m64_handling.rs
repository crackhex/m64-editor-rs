use std::array::TryFromSliceError;
use std::ascii::Char as AsciiChar;
use std::ops::{Shr};
use bitvec::prelude::BitArray;
use bitvec::view::BitViewSized;

pub type Controllers = [Vec<Input>; 4];
pub type ByteVec = Vec<u8>;

pub struct M64File {
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
    pub inputs: Controllers,            //0x400
}

pub struct Input {
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

impl Input {
    fn new() -> Input {
        Input {
            r_dpad: false,
            l_dpad: false,
            d_dpad: false,
            u_dpad: false,
            start: false,
            z_trig: false,
            b_button: false,
            a_button: false,
            c_right: false,
            c_left: false,
            c_down: false,
            c_up: false,
            r_trig: false,
            l_trig: false,
            x: 0,
            y: 0,
        }
    }
    fn parse(input_bytes: &ByteVec, controller_flags: u8) -> Controllers {
        let mut inputs: Controllers = [const { Vec::new() }; 4];
        let mut active_controllers = M64File::active_controllers(controller_flags as u32).expect("TODO: panic message");
        for i in (0..input_bytes.len()).step_by(4) {

            let input = u32::from_le_bytes(input_bytes[i..i+4].try_into().unwrap());
            let current_controller = active_controllers[(i/4) % active_controllers.len()];
            inputs[current_controller].push(Input {
                r_dpad: (input & 0x01) != 0,
                l_dpad: (input & 0x02) != 0,
                d_dpad: (input & 0x04) != 0,
                u_dpad: (input & 0x08) != 0,
                start: (input & 0x10) != 0,
                z_trig: (input & 0x20) != 0,
                b_button: (input & 0x40) != 0,
                a_button: (input & 0x80) != 0,
                c_right: (input & 0x100) != 0,
                c_left: (input & 0x200) != 0,
                c_down: (input & 0x400) != 0,
                c_up: (input & 0x800) != 0,
                r_trig: (input & 0x1000) != 0,
                l_trig: (input & 0x2000) != 0,
                x: input.shr(16) as i8,
                y: input.shr(24) as i8,
            });
        }
        inputs
    }

    pub(crate) fn samples_to_bytes(inputs: &Controllers, active_controllers: &Vec<usize>) -> ByteVec {

        let size = inputs[0].len() + inputs[1].len() + inputs[2].len() + inputs[3].len();
        let mut input_bytes: ByteVec = vec![0; size*4];
        for i in 0..size {
            let current_controller = active_controllers[i % active_controllers.len()];
            let frame = i.div_floor(active_controllers.len());
            let input = &inputs[current_controller][frame];
            let mut input_byte: u32 = 0;
            input_byte |= input.r_dpad as u32;
            input_byte |= (input.l_dpad as u32) << 1;
            input_byte |= (input.d_dpad as u32) << 2;
            input_byte |= (input.u_dpad as u32) << 3;
            input_byte |= (input.start as u32) << 4;
            input_byte |= (input.z_trig as u32) << 5;
            input_byte |= (input.b_button as u32) << 6;
            input_byte |= (input.a_button as u32) << 7;
            input_byte |= (input.c_right as u32) << 8;
            input_byte |= (input.c_left as u32) << 9;
            input_byte |= (input.c_down as u32) << 10;
            input_byte |= (input.c_up as u32) << 11;
            input_byte |= (input.r_trig as u32) << 12;
            input_byte |= (input.l_trig as u32) << 13;
            input_byte |= ((input.x  as u8) as u32) << 16;
            input_byte |= ((input.y as u8) as u32) << 24;
            input_bytes[i*4..i*4+4].copy_from_slice(&input_byte.to_le_bytes());
        }
        input_bytes
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
            inputs: [const { Vec::new() }; 4],
        }
    }
    pub fn from_bytes(buf: &ByteVec) -> Result<M64File, TryFromSliceError> {
        if buf.len() < 0x400 {
            panic!("File is too small to be a valid M64 file");
        }
        let m64 = M64File {
            signature: buf[0x0..0x4].try_into()?,
            version: u32::from_le_bytes(buf[0x4..0x8].try_into()?),
            uid: i32::from_le_bytes(buf[0x8..0xC].try_into()?),
            vi_count: u32::from_le_bytes(buf[0xC..0x10].try_into()?),
            rerecord_count: u32::from_le_bytes(buf[0x10..0x14].try_into()?),
            vi_per_second: buf[0x14],
            controller_count: buf[0x15],
            num_samples: u32::from_le_bytes(buf[0x18..0x1C].try_into()?),
            movie_start_type: u16::from_le_bytes(buf[0x1C..0x1E].try_into()?),
            controller_flags: u32::from_le_bytes(buf[0x20..0x24].try_into()?),
            internal_name: *<&[u8] as TryInto<[u8; 32]>>::try_into(&buf[0xC4..0xE4])?.as_ascii().unwrap(),
            crc32: u32::from_le_bytes(buf[0xE4..0xE8].try_into()?),
            country_code: u16::from_le_bytes(buf[0xE8..0xEA].try_into()?),
            video_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x122..0x162])?.as_ascii().unwrap(),
            sound_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x162..0x1A2])?.as_ascii().unwrap(),
            input_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x1A2..0x1E2])?.as_ascii().unwrap(),
            rsp_plugin: *<&[u8] as TryInto<[u8; 64]>>::try_into(&buf[0x1E2..0x222])?.as_ascii().unwrap(),
            author: *<&[u8] as TryInto<[u8; 222]>>::try_into(&buf[0x222..0x300])?.as_ascii().unwrap(),
            movie_desc: *<&[u8] as TryInto<[u8; 256]>>::try_into(&buf[0x300..0x400])?.as_ascii().unwrap(),
            inputs: Input::parse(&buf[0x400..].to_vec(), buf[0x20]),
        };

        Ok(m64)

    }
    pub fn active_controllers(controller_flags: u32) -> Option<Vec<usize>> {
        // Returns a vector with the indices of the active controllers,
        // e.g., if controller 1, 2, and 4 are enabled, it will return [1, 2, 4]
        let mut controllers: BitArray<u32>= controller_flags.into_bitarray();
        let active_controllers: Vec<usize> = (0..4).filter(|&i| controllers[i]).collect();
        Option::from((!active_controllers.is_empty()).then_some(active_controllers))

    }
    pub fn to_bytes(&self) -> ByteVec {
        let active_controllers = Self::active_controllers(self.controller_flags).expect("TODO: panic message");
        let mut sample_bytes: ByteVec = Input::samples_to_bytes(&self.inputs, &active_controllers);
        let mut buffer: ByteVec = vec![0; 0x400 + sample_bytes.len()];
        buffer[0x0..0x4].copy_from_slice(&self.signature);
        buffer[0x4..0x8].copy_from_slice(&self.version.to_le_bytes());
        buffer[0x8..0xC].copy_from_slice(&self.uid.to_le_bytes());
        buffer[0xC..0x10].copy_from_slice(&self.vi_count.to_le_bytes());
        buffer[0x10..0x14].copy_from_slice(&self.rerecord_count.to_le_bytes());
        buffer[0x14] = self.vi_per_second;
        buffer[0x15] = self.controller_count;
        buffer[0x18..0x1C].copy_from_slice(&self.num_samples.to_le_bytes());
        buffer[0x1C..0x1E].copy_from_slice(&self.movie_start_type.to_le_bytes());
        buffer[0x20..0x24].copy_from_slice(&self.controller_flags.to_le_bytes());
        buffer[0xC4..0xE4].copy_from_slice(&self.internal_name.as_bytes());
        buffer[0xE4..0xE8].copy_from_slice(&self.crc32.to_le_bytes());
        buffer[0xE8..0xEA].copy_from_slice(&self.country_code.to_le_bytes());
        buffer[0x122..0x162].copy_from_slice(&self.video_plugin.as_bytes());
        buffer[0x162..0x1A2].copy_from_slice(&self.sound_plugin.as_bytes());
        buffer[0x1A2..0x1E2].copy_from_slice(&self.input_plugin.as_bytes());
        buffer[0x1E2..0x222].copy_from_slice(&self.rsp_plugin.as_bytes());
        buffer[0x222..0x300].copy_from_slice(&self.author.as_bytes());
        buffer[0x300..0x400].copy_from_slice(&self.movie_desc.as_bytes());
        buffer[0x400..].copy_from_slice(&sample_bytes);
        buffer
    }
}



