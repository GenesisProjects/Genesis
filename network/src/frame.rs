use bytebuffer::ByteBuffer;
use common::key::PublicKey;
use std::io::{Error, ErrorKind};
use std::sync::Mutex;

lazy_static! {
    pub static ref SHARED_FRAME_READER: Mutex<FrameReader> = {
        Mutex::new(FrameReader::new())
    };
}

/// Frame Struct
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |0 0 0 0 0 0 0| type  |role   |c|   task      |    code         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           SEQ (32 bits)                       |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                     Address (256 bits)                        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             ext1 (32 bits)                    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             ext2 (32 bits)                    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                     payload size (64 bits)                    |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |...............................................................|
/// |...............................................................|
/// |...............................................................|
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1|
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

const STUFFING_BYTES: u32 = 0xffffffff;
const WINDOW_SIZE: usize = 1024 * 1024 * 16;

const FRAME_MAX_SIZE: usize = 1024 * 16;
const FRAME_MIN_SIZE: usize = 388;

pub enum FrameType {
    Request,
    Response,
    Transmit,
}

pub enum Task {
    SyncBlock,
    SyncChain,
    SyncTransaction,
    SyncAccount,
    SyncLog,
}

pub enum Role {
    Producer,
    Normal,
}


//TODO: Add more code
enum ReponseCode {
    Ok,
    Err,
}

type SEQ = u32;

pub struct Frame {
    frame_type: FrameType,
    task: Task,
    has_rlp_payload: bool,
    code: ReponseCode,
    payload_size: Option<u64>,
    pub_key: PublicKey,
    role: Role,
    seq: SEQ,
    ext1: u32,
    ext2: u32,
    payload: Option<Vec<u8>>,
}

impl Frame {
    pub fn new(buff: &[u8]) -> Option<Self> {
        let mut buffer = ByteBuffer::from_bytes(buff);

        // check if reach the minimum size
        if buff.len() < FRAME_MIN_SIZE {
            return None;
        }

        // prefix check
        let prefix = buffer.read_bits(7);
        if prefix != 0 {
            return None;
        }

        let frame_type = match buffer.read_bits(4) {
            0u64 => FrameType::Request,
            1u64 => FrameType::Response,
            2u64 => FrameType::Transmit,
            _ => { return None; }
        };

        let role = match buffer.read_bits(4) {
            0u64 => Role::Producer,
            1u64 => Role::Normal,
            _ => { return None; }
        };

        let has_rlp_payload = match buffer.read_bits(1) {
            0u64 => false,
            1u64 => true,
            _ => { return None; }
        };


        let task = match buffer.read_bits(7) {
            0u64 => Task::SyncBlock,
            1u64 => Task::SyncChain,
            2u64 => Task::SyncTransaction,
            3u64 => Task::SyncAccount,
            4u64 => Task::SyncLog,
            _ => { return None; }
        };

        let code = match buffer.read_bits(9) {
            0u64 => ReponseCode::Ok,
            1u64 => ReponseCode::Err,
            _ => { return None; }
        };

        let seq = buffer.read_u32();

        let pub_key = buffer.read_bytes(32);
        let mut pub_key_buff = [0u8; 32];
        pub_key_buff[0..32].copy_from_slice(&pub_key[0..32]);

        let ext1 = buffer.read_u32();

        let ext2 = buffer.read_u32();

        if has_rlp_payload {
            let payload_size = buffer.read_u64();
            let payload = buffer.read_bytes(payload_size as usize);
            if buff.len() < payload_size as usize + 8usize + FRAME_MIN_SIZE {
                return None;
            }
            return Some(Frame {
                frame_type: frame_type,
                task: task,
                has_rlp_payload: has_rlp_payload,
                code: code,
                payload_size: Some(payload_size),
                pub_key: pub_key_buff,
                role: role,
                seq: seq,
                ext1: ext1,
                ext2: ext2,
                payload: Some(payload),
            });
        } else {
            return Some(Frame {
                frame_type: frame_type,
                task: task,
                has_rlp_payload: has_rlp_payload,
                code: code,
                payload_size: None,
                pub_key: pub_key_buff,
                role: role,
                seq: seq,
                ext1: ext1,
                ext2: ext2,
                payload: None,
            });
        }
    }

    pub fn serialize_frame(&self) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct FrameReader {
    cache: [u8; WINDOW_SIZE],
    r_pos: usize,
    w_pos: usize,
}

impl FrameReader {
    pub fn new() -> Self {
        FrameReader {
            cache: [0u8; WINDOW_SIZE],
            r_pos: 0,
            w_pos: 0,
        }
    }

    pub fn flush(&mut self) {
        self.w_pos = 0;
        self.r_pos = 0;
    }

    pub fn read_one_frame(&mut self) -> Result<Frame, Error> {
        let len = self.data_len();
        match self.find_cur_frame_len() {
            Some(offset) => {
                if offset > FRAME_MAX_SIZE {
                    self.r_pos = (self.r_pos + offset) % WINDOW_SIZE;
                    Err(Error::new(ErrorKind::InvalidData, "Current frame is over max size, will be ignored"))
                } else {
                    let mut frame_buff: [u8; FRAME_MAX_SIZE] = [0u8; FRAME_MAX_SIZE];
                    self.read_cache(&mut frame_buff[0..offset]);
                    match Frame::new(&frame_buff[0..offset]) {
                        Some(f) => Ok(f),
                        None => Err(Error::new(ErrorKind::InvalidData, "Malformed frame"))
                    }
                }
            }
            None => Err(Error::new(ErrorKind::WouldBlock, "The frame is not ready"))
        }
    }

    pub fn write_one_frame(&mut self, frame: &Frame) -> Result<usize, Error> {
        let frame_buff = frame.serialize_frame();
        let frame_len = frame_buff.len();
        if frame_len + self.data_len() > WINDOW_SIZE {
            Err(Error::new(ErrorKind::InvalidData, "Reach max window size"))
        } else if frame_len > FRAME_MAX_SIZE {
            Err(Error::new(ErrorKind::InvalidData, "Current frame is over max size, will be ignored"))
        } else {
            self.write_cache(&frame_buff[0..frame_len]);
            Ok(frame_len)
        }
    }

    pub fn append_data(&mut self, data: &Vec<u8>) -> Result<usize, Error> {
        if data.len() + self.data_len() > WINDOW_SIZE {
            Err(Error::new(ErrorKind::InvalidData, "Reach max window size"))
        } else {
            self.write_cache(&data[0..data.len()])
        }
    }

    fn read_cache(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.len() > self.data_len() {
            Err(Error::new(ErrorKind::InvalidData, "Data is not ready"))
        } else {
            if self.r_pos + buffer.len() >= WINDOW_SIZE {
                let tail_len = WINDOW_SIZE - self.r_pos + 1;
                let buff_len = buffer.len();
                buffer[0..tail_len].copy_from_slice(&self.cache[self.r_pos..WINDOW_SIZE]);
                buffer[tail_len..buff_len].copy_from_slice(&self.cache[0..buff_len - tail_len]);
            } else {
                let buff_len = buffer.len();
                buffer[0..buff_len].copy_from_slice(&self.cache[self.r_pos..self.r_pos + buff_len]);
            }
            self.r_pos = (self.r_pos + buffer.len()) % WINDOW_SIZE;
            Ok(buffer.len())
        }
    }

    fn write_cache(&mut self, buffer: &[u8]) -> Result<usize, Error> {
        let len = self.data_len();
        if buffer.len() + len > WINDOW_SIZE {
            Err(Error::new(ErrorKind::InvalidData, "The frame data is overflow"))
        } else {
            if self.w_pos + buffer.len() >= WINDOW_SIZE {
                let tail_len = WINDOW_SIZE - self.w_pos + 1;
                self.cache[self.w_pos..WINDOW_SIZE].copy_from_slice(&buffer[0..tail_len]);
                self.cache[0..buffer.len() - tail_len].copy_from_slice(&buffer[tail_len..buffer.len()]);
            } else {
                self.cache[self.w_pos..self.w_pos + buffer.len()].copy_from_slice(&buffer[0..buffer.len()]);
            }
            self.w_pos = (self.w_pos + buffer.len()) % WINDOW_SIZE;
            Ok(buffer.len())
        }
    }

    fn data_len(&self) -> usize {
        if self.w_pos >= self.r_pos {
            self.w_pos - self.r_pos
        } else {
            self.w_pos + WINDOW_SIZE - self.r_pos
        }
    }

    fn find_cur_frame_len(&self) -> Option<usize> {
        let len = self.data_len();

        if len < 4 {
            return None;
        }
        for i in self.r_pos..len - 3 {
            if self.cache[i] == 0xffu8
                && self.cache[(i + 1) % WINDOW_SIZE] == 0xffu8
                && self.cache[(i + 2) % WINDOW_SIZE] == 0xffu8
                && self.cache[(i + 3) % WINDOW_SIZE] == 0xffu8 {
                return Some(i + 4 - self.r_pos);
            }
        }
        None
    }
}