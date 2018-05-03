use common::key::PublicKey;
use bytebuffer::ByteBuffer;
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
/// |                           SEQ                                 |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                     Address (32 bits)                         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             ext1                              |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                             ext2                              |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                     payload size (64 bits)                    |
/// |                                                               |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |...............................................................|
/// |...............................................................|
/// |...............................................................|
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1|
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

const STUFFING_BYTES: u32   = 0xffffffff;
const WINDOW_SIZE: usize    = 1024 * 1024 * 16;

const FRAME_MAX_SIZE: usize = 1024 * 16;

enum FrameType {
    Request,
    Response,
    Transmit
}

enum Task {
    SyncBlock,
    SyncChain,
    SyncTransaction,
    SyncAccount,
    SyncLog,
}

enum Role {
    Producer,
    Normal
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
    payload_size: u64,
    pub_key: PublicKey,
    role: Role,
    seq: SEQ,
    ext1: u32,
    ext2: u32,
    payload: Vec<u8>
}

impl Frame {
    pub fn new(buff: &[u8]) -> Option<Self> {
        unimplemented!()
    }

    pub fn serialize_frame(&self) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct FrameReader {
    cache: [u8; WINDOW_SIZE],
    r_pos: usize,
    w_pos: usize
}

impl FrameReader {
    pub fn new() -> Self {
        FrameReader {
            cache: [0u8; WINDOW_SIZE],
            r_pos: 0,
            w_pos: 0
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
                    self.read_cache(&mut frame_buff[0 .. offset]);
                    match Frame::new(&frame_buff[0 .. offset]) {
                        Some(f) => Ok(f),
                        None => Err(Error::new(ErrorKind::InvalidData, "Malformed frame"))
                    }
                }
            },
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
            self.write_cache(&frame_buff[0 .. frame_len]);
            Ok(frame_len)
        }
    }

    pub fn append_data(&mut self, data: &Vec<u8>) -> Result<usize, Error> {
        if data.len() + self.data_len() > WINDOW_SIZE {
            Err(Error::new(ErrorKind::InvalidData, "Reach max window size"))
        } else {
            self.write_cache(&data[0 .. data.len()])
        }
    }

    fn read_cache(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.len() > self.data_len() {
            Err(Error::new(ErrorKind::InvalidData, "Data is not ready"))
        } else {
            if self.r_pos + buffer.len() >= WINDOW_SIZE {
                let tail_len = WINDOW_SIZE - self.r_pos + 1;
                let buff_len =  buffer.len();
                buffer[0 .. tail_len].copy_from_slice(&self.cache[self.r_pos .. WINDOW_SIZE]);
                buffer[tail_len .. buff_len].copy_from_slice(&self.cache[0 .. buff_len - tail_len]);
            } else {
                let buff_len =  buffer.len();
                buffer[0 .. buff_len].copy_from_slice(&self.cache[self.r_pos .. self.r_pos + buff_len]);
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
                self.cache[self.w_pos .. WINDOW_SIZE].copy_from_slice(&buffer[0 .. tail_len]);
                self.cache[0 .. buffer.len() - tail_len].copy_from_slice(&buffer[tail_len .. buffer.len()]);
            } else {
                self.cache[self.w_pos .. self.w_pos + buffer.len()].copy_from_slice(&buffer[0 .. buffer.len()]);
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
        for i in self.r_pos .. len - 3 {
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