use super::super::peer_manager::*;
use super::super::socket::message::message_handler::*;
use super::super::socket::message::defines::*;
use gen_processor::ContextRef;
use mio::Token;
use std::io::Result;
use std::sync::{ Arc, Mutex };

const P2P_MANAGER_CH_NAME: &'static str = "download_p2p_manager";
const P2P_MANAGER_EVENT_SIZE: usize = 1024;
const P2P_MANAGER_STACK_SIZE: usize = 1024;

pub struct DownloadServiceSession;

pub fn new_trait_obj_ref(service: DownloadService) -> ContextRef<SocketMessageListener> {
    let service_trait_obj = Arc::new(Mutex::new(service)) as Arc<Mutex<SocketMessageListener>>;
    ContextRef::new_trait_obj_ref(service_trait_obj)
}

pub struct DownloadService {
    handler: SocketMessageHandler<DownloadServiceSession>
}

impl DownloadService {
    pub fn new() -> Self {
        DownloadService {
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

impl Drop for DownloadService {
    fn drop(&mut self) {

    }
}

impl SocketMessageListener for DownloadService {
    fn notify(&mut self, msg: SocketMessage) {
        unimplemented!()
    }
    fn peer_accepted(&mut self, token: Token) {
        unimplemented!()
    }

    fn peer_droped(&mut self, token: Token) {
        unimplemented!()
    }
}