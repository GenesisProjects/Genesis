use chrono::*;

use mio::{Evented, Poll, PollOpt, Ready, Token};
use mio::tcp::TcpStream;

use std::io::*;
use std::net::{Shutdown, SocketAddr};
use std::time::Instant;

use common::address::Address as Account;
use frame::*;
use pool_manager::SHARED_POOL_MANAGER;
use socket::*;

#[derive(Debug, Clone)]
pub enum SessionStatus {
    Init,
    RequestConnection,      // Client Only
    EstablishConnection,    // Server Only
    Rejected,               // Server Only

    Connected,
    Disconnected,
    Idle,
    WaitReceiving,
    Receiving,
    WaitTransmitting,
    Transmitting
}

struct FrameManager {
    start: SEQ,
    end: SEQ,
    frames: Vec<FrameRef>,
    last_frame: Option<FrameRef>,
}

impl FrameManager {
    pub fn new() -> Self {
        FrameManager {
            start: 0,
            end: 0,
            frames: vec![],
            last_frame: None,
        }
    }

    fn flush(&mut self) -> Vec<u8> {
        let frames = self.frames.to_owned();
        self.start = 0;
        self.end = 0;
        self.frames = vec![];
        self.last_frame = None;

        let mut result: Vec<u8> = vec![];
        for frame in frames {
            result.append(&mut frame.get_payload())
        }
        SHARED_POOL_MANAGER.lock().unwrap().accept(&result);
        result
    }

    fn current_seq(&self) -> Option<SEQ> {
        match self.last_frame {
            Some(ref frame) => Some(frame.get_seq()),
            None => None
        }
    }

    fn is_full(&self) -> bool {
        match self.last_frame {
            Some(ref frame) => frame.get_seq() == self.end,
            None => false
        }
    }

    pub fn push_frames(&mut self, frames: &mut Vec<FrameRef>) {
        loop {
            let remain_frames = self._push_frames(frames);
            if remain_frames.is_empty() { break; }
            if self.is_full() { self.flush(); }
        }
    }

    fn _push_frames(&mut self, frames: &mut Vec<FrameRef>) -> Vec<FrameRef> {
        if frames.is_empty() {
            return vec![];
        }

        if self.last_frame.is_none() {
           frames.first().and_then(|frame: &FrameRef| {
               let (start, end) = frame.get_trans_range();
               self.start = start;
               self.end = end;
               self.last_frame = Some(frame.clone());
               Some(frame.clone())
           });
        }

        let mut frames_within_range: Vec<FrameRef> = self.frames.to_owned()
            .into_iter()
            .filter(|frame| frame.get_seq() <= self.end && frame.get_seq() >= self.start)
            .collect();

        let frames_out_range: Vec<FrameRef> = self.frames.to_owned()
            .into_iter()
            .filter(|frame| frame.get_seq() > self.end)
            .collect();

        frames_within_range.last().and_then(|frame: &FrameRef| {
            self.last_frame = Some(frame.clone());
            Some(frame.clone())
        });

        self.frames.append(&mut frames_within_range);

        frames_out_range
    }
}

pub struct TaskContext {
    data_send: usize,
    data_recv: usize,
    frame_send: usize,
    frame_recv: usize,
    cur_task: Task,
    frame_manager: FrameManager,
}

impl TaskContext {
    pub fn new() -> Self {
        TaskContext {
            data_send: 0,
            data_recv: 0,
            frame_send: 0,
            frame_recv: 0,
            cur_task: Task::Idle,
            frame_manager: FrameManager::new()
        }
    }

    pub fn reset(&mut self) {
        self.data_send = 0;
        self.data_recv = 0;
        self.frame_send = 0;
        self.frame_recv = 0;
        self.frame_manager.flush();
    }

    pub fn push_frames(&mut self, frames: &mut Vec<FrameRef>) {
        self.frame_manager.push_frames(frames)
    }
}

pub struct Session {
    socket: PeerSocket,
    status: SessionStatus,
    addr: SocketAddr,
    created: DateTime<Utc>,
    context: TaskContext
}

impl Session {
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        match PeerSocket::connect(addr) {
            Ok(r) => {
                Ok(Session {
                    socket: r,
                    status: SessionStatus::Init,
                    addr: addr.clone(),
                    created: Utc::now(),
                    context: TaskContext::new()
                })
            },
            Err(e) => Err(e)
        }
    }

    pub fn recieve(&mut self) -> Result<Vec<FrameRef>> {
        match self.status {
            SessionStatus::Transmitting => self.socket.read_stream_to_cache().and_then(|n: usize|
                Ok(self.socket.read_frames_from_cache())
            ).and_then(|frames: Vec<FrameRef>| {
                self.context.push_frames(&mut frames.to_owned());
                Ok(frames)
            }),
            _ => Err(Error::new(ErrorKind::Other, "Session is not ready"))
        }

    }

    pub fn disconnect(addr: &SocketAddr) -> Self {
        unimplemented!()
    }

    pub fn status(&self) -> SessionStatus {
        self.status.clone()
    }

}

impl Evented for Session {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.socket.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        self.socket.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> Result<()> {
        self.socket.deregister(poll)
    }
}