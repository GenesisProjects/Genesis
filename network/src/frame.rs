use common::key::PublicKey;
use bytebuffer::ByteBuffer;
use std::io::{Error, ErrorKind};

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

const STUFFING_BYTES: u32 = 0xffffffff;
const WINDOW_SIZE: usize = 1024 * 1024 * 16;

enum FrameType {
    Request,
    Response,
    Transmit
}

enum Task {
    SyncBlock,
    SyncTransaction,
    SyncChain,
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
}

struct FrameReader {
    cache: [u8; WINDOW_SIZE],
    r_pos: usize,
    w_pos: usize
}

impl FrameReader {
    pub fn read_frame(&mut self, buffer: &mut ByteBuffer) -> Result<Option<Frame>, Error> {
        if !stick_mode {
            self.flush();
        }
        let len = self.data_len();
        unimplemented!()
    }

    fn read_cache(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.len() > self.data_len() {
            Err(Error::new(ErrorKind::InvalidData, "Data is not ready"))
        } else {
            if self.r_pos + buffer.len() >= WINDOW_SIZE {
                let tail_len = WINDOW_SIZE - self.r_pos + 1;
                buffer[0 .. tail_len].copy_from_slice(self.cache[self.r_pos .. WINDOW_SIZE]);
                buffer[tail_len .. buffer.len()].copy_from_slice(self.cache[0 .. buffer.len() - tail_len]);
            } else {
                buffer[0 .. buffer.len()].copy_from_slice(self.cache[self.r_pos .. self.r_pos + buffer.len()]);
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
                self.cache[self.w_pos .. WINDOW_SIZE].copy_from_slice(buffer[0 .. tail_len]);
                self.cache[0 .. buffer.len() - tail_len].copy_from_slice(buffer[tail_len .. buffer.len()]);
            } else {
                self.cache[self.w_pos .. self.w_pos + buffer.len()].copy_from_slice(buffer[0 .. buffer.len()]);
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

    fn find_tail(&self) -> Option<usize> {
        let len = self.data_len();

        if len < 4 {
            return None;
        }
        for i in self.r_pos .. len - 3 {
            if self.cache[i] == 0xffu8
                && self.cache[(i + 1) % WINDOW_SIZE] == 0xffu8
                && self.cache[(i + 2) % WINDOW_SIZE] == 0xffu8
                && self.cache[(i + 3) % WINDOW_SIZE] == 0xffu8 {
                return Some(i + 4 - r_pos);
            }
        }
        None
    }

    fn flush(&mut self) {
        self.w_pos = 0;
        self.r_pos = 0;
    }
}