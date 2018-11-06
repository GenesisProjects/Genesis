mod protocol;
use self::protocol::*;

use super::super::peer_manager::*;
use super::super::socket::message::message_handler::*;
use super::super::socket::message::defines::*;
use common::address::Address;
use common::hash::Hash;
use gen_core::account::Account;
use gen_core::block::*;
use gen_core::transaction::*;
use gen_core::blockchain::*;
use gen_processor::*;
use gen_message::{ Message, MESSAGE_CENTER, defines::p2p::* };
use gen_utils::config_parser::version;
use mio::Token;
use std::collections::HashMap;
use std::io::Result;
use std::sync::{ Mutex, MutexGuard };

const P2P_MANAGER_CH_NAME: &'static str = "download_p2p_manager";
const P2P_MANAGER_EVENT_SIZE: usize = 1024;
const P2P_MANAGER_STACK_SIZE: usize = 4 * 1024 * 1024 * 1024;

pub enum ConsensusServiceSessionStatus {
    Init,
    Idle,
    SendingBlockInfo,
    SendingBlock,
    SendingTransactionInfo,
    SendingTransaction,
    SendingAccountInfo,
    SendingAccount,
    SendingStorageInfo,
    SendingStorage
}

pub struct ConsensusServiceSession {
    pub account: Option<Address>,
    pub ancestor: Option<Hash>,
    pub cur_height: Option<usize>,
    pub tail_hash: Option<Hash>,
    pub handshaked: bool,
    pub status: ConsensusServiceSessionStatus,
    pub pending_chain: Vec<Block>
}

impl ConsensusServiceSession {
    pub fn new() -> ConsensusServiceSession {
        ConsensusServiceSession {
            account: None,
            ancestor: None,
            cur_height: None,
            tail_hash: None,
            handshaked: false,
            status: ConsensusServiceSession::Init,
            pending_chain: vec![]
        }
    }
}

/// Consensus Session Pool Singleton
lazy_static! {
    pub static ref PEER_SESSION_POOL: Mutex<HashMap<Token, ConsensusServiceSession>> = {
        Mutex::new(HashMap::new())
    };
}

fn peer_info() -> Option<PeerInfo> {
    match Address::load() {
        Some(addr) => Some(PeerInfo::new (
            addr,
            block_chain_len() as u64,
            last_block_hash())
        ),
        None => None
    }

}

/// Consensus controller
pub struct ConsensusController {
    p2p_manager_ref: ContextRef<P2PManager>
}

impl ConsensusController {
    pub fn new() -> Result<Self> {
        let hook = SyncMessageHook::new();
        hook.into_p2p_manager_ref().and_then(|context_ref| {
            Ok(ConsensusController { p2p_manager_ref: context_ref })
        })
    }

    pub fn p2p_manager_ref(&self) -> ContextRef<P2PManager> {
        self.p2p_manager_ref.clone()
    }

    pub fn start(&mut self) {
        self.p2p_manager_ref.lock().start();
    }
}

//////////////////////////////////////////////////////////////////
// Message Handlers                                             //
//////////////////////////////////////////////////////////////////
// Consensus peer message handler
fn sync_peer_handler(session: &mut ConsensusServiceSession, msg: &SocketMessage, name: String) -> bool {
    match session.status {
        ConsensusServiceSessionStatus::Init | ConsensusServiceSessionStatus::Idle => {},
        _ => { return false }
    }
    let version = version();
    if let Some(info) = SyncProtocol::new(version.as_str()).parse_sync(msg) {
        session.account = Some(info.account());
        session.cur_height = Some(info.cur_height() as usize);
        session.tail_hash = Some(info.tail_hash());
        session.handshaked = true;
    }
    true
}

/// Consensus message hook
pub struct SyncMessageHook {
    handler: SocketMessageHandler<ConsensusServiceSession>
}

impl SyncMessageHook {
    pub fn new() -> Self {
        let mut handle: SocketMessageHandler<ConsensusServiceSession> = SocketMessageHandler::new();
        handle.add_event_handler(PEER_SYNC_STR.to_string(), sync_peer_handler);
        SyncMessageHook {
            handler: handle
        }
    }

    pub fn into_p2p_manager_ref(self) -> Result<ContextRef<P2PManager>> {
        let config = P2PConfig::load("network.p2p");
        let p2p_manager_ref_result = P2PManager::create(
            P2P_MANAGER_CH_NAME.to_string(),
            config,
            P2P_MANAGER_EVENT_SIZE
        );
        p2p_manager_ref_result.and_then(|p2p_manager_ref| {
            p2p_manager_ref.lock().set_msg_hook(self);
            Ok(p2p_manager_ref)
        })
    }

    fn notify_send_msg(&self, token: &Token, msg: SocketMessage) {
        let id = token.0;
        notify!(P2P_MANAGER_CH_NAME.to_string(), Message::new(SEND_MESSAGE.to_string(), id as u32, msg.into_bytes()))
            .expect("Could not notify sending msg");
    }
}

impl Drop for SyncMessageHook {
    fn drop(&mut self) {

    }
}

impl SocketMessageHook for SyncMessageHook {
    fn notify(&mut self, msg: SocketMessage, token: Token) {
        if let Some(session) = PEER_SESSION_POOL.lock().unwrap().get_mut(&token) {
            self.handler.process_event(
                msg.event(),
                session,
                &msg
            );
        }
    }

    fn peer_accepted(&mut self, token: Token) {
        let new_session = ConsensusServiceSession::new();
        let mut guard = PEER_SESSION_POOL.lock().unwrap();
        guard.insert(token.clone(), new_session);
        let peer_info = peer_info().expect("Could not get the peer info");
        let version = version();
        let msg = SyncProtocol::new(version.as_str()).sync(&peer_info);
        self.notify_send_msg(&token, msg);
    }

    fn peer_droped(&mut self, token: Token) {
        let mut guard =  PEER_SESSION_POOL.lock().unwrap();
        guard.remove(&token);
    }
}