use super::super::peer_manager::*;
use super::super::socket::message::message_handler::*;
use super::super::socket::message::defines::*;
use gen_processor::ContextRef;
use mio::Token;

const P2P_MANAGER_CH_NAME: &'static str = "download_p2p_manager";

pub struct DownloadServiceSession;

pub struct DownloadService {
    handler: SocketMessageHandler<DownloadServiceSession>
}

impl DownloadService {
    /*fn new() -> Self {

    }

    pub fn start() -> ContextRef<Self> {
        let config = P2PConfig::load("network.p2p");
        P2PManager::create(P2P_MANAGER_CH_NAME.to_string(), config, 1024, )
    }*/
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