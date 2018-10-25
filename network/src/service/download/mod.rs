mod protocol;

use super::super::peer_manager::*;
use super::super::socket::message::message_handler::*;
use super::super::socket::message::defines::*;
use common::address::Address;
use common::hash::Hash;
use gen_core::account::Account;
use gen_core::block::*;
use gen_core::transaction::*;
use gen_processor::ContextRef;
use mio::Token;
use std::collections::HashMap;
use std::io::Result;
use std::sync::{ Arc, Mutex };

const P2P_MANAGER_CH_NAME: &'static str = "download_p2p_manager";
const P2P_MANAGER_EVENT_SIZE: usize = 1024;
const P2P_MANAGER_STACK_SIZE: usize = 1024;

pub enum DownloadServiceSessionStatus {
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

pub struct DownloadServiceSession {
    account: Option<Address>,
    cur_height: Option<usize>,
    tail_hash: Option<Hash>,
    handshaked: bool,
    status: DownloadServiceSessionStatus,
    pending_chain: Vec<Block>
}

impl DownloadServiceSession {
    pub fn new() -> DownloadServiceSession {
        DownloadServiceSession {
            account: None,
            cur_height: None,
            tail_hash: None,
            handshaked: false,
            status: DownloadServiceSessionStatus::Init,
            pending_chain: vec![]
        }
    }
}

/// Downloader Session Pool Singleton
lazy_static! {
    pub static ref PEER_SESSION_POOL: Mutex<HashMap<Token, DownloadServiceSession>> = {
        Mutex::new(HashMap::new())
    };
}

pub fn new_trait_obj_ref(service: DownloadMessageHook) -> ContextRef<SocketMessageHook> {
    let service_trait_obj = Arc::new(Mutex::new(service)) as Arc<Mutex<SocketMessageHook>>;
    ContextRef::new_trait_obj_ref(service_trait_obj)
}

pub struct DownloadController {

}

impl DownloadController {

}

pub struct DownloadMessageHook {
    handler: SocketMessageHandler<DownloadServiceSession>
}

impl DownloadMessageHook {
    pub fn new() -> Self {
        DownloadMessageHook {
            handler: SocketMessageHandler::new()
        }
    }

    pub fn into_p2p_manager_ref(self) -> Result<ContextRef<P2PManager>> {
        let config = P2PConfig::load("network.p2p");
        P2PManager::create(
            P2P_MANAGER_CH_NAME.to_string(),
            config,
            P2P_MANAGER_EVENT_SIZE,
            P2P_MANAGER_STACK_SIZE,
            new_trait_obj_ref(self)
        )
    }
}

impl Drop for DownloadMessageHook {
    fn drop(&mut self) {

    }
}

impl SocketMessageHook for DownloadMessageHook {
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
        let new_session = DownloadServiceSession::new();
        PEER_SESSION_POOL.lock().unwrap().insert(token, new_session);
    }

    fn peer_droped(&mut self, token: Token) {
        PEER_SESSION_POOL.lock().unwrap().remove(&token);
    }
}