pub mod defines;
pub mod stun;
pub mod upnp;

use std::net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub type SocketInfo = SocketAddr;

pub fn socket_info(addr: String) -> Result<SocketInfo, AddrParseError> {
    let result = addr.parse::<SocketAddr>();
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e)
    }
}

pub enum Protocol {
    UPNP,
    STUN
}

pub fn get_public_ip_addr(protocol: Protocol, local: &SocketInfo) -> Option<SocketInfo> {
    match protocol {
        Protocol::UPNP => upnp::map_external_address_upnp(local),
        _ => panic!("Unknown protocol")
    }
}