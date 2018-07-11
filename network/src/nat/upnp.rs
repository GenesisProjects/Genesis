use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::io;
use igd::{PortMappingProtocol, search_gateway_from_timeout};
use std::time::Duration;
use super::SocketInfo;

pub fn map_external_address_upnp(local: &SocketInfo) -> Option<SocketInfo> {
    let udp_port = local.port();
    let result = match *local {
        SocketAddr::V4(ref local_addr) => match search_gateway_from_timeout(local_addr.ip().clone(), Duration::new(5, 0)) {
            Err(ref err) => { println!("Gateway search error: {}", err); None },
            Ok(gateway) => {
                match gateway.get_external_ip() {
                    Err(ref err) => {
                        println!("IP request error: {}", err);
                        None
                    },
                    Ok(external_addr) => {
                        match gateway.add_any_port(PortMappingProtocol::TCP, SocketAddrV4::new(local_addr.ip().clone(), local_addr.port()), 0, "TCP") {
                            Err(ref err) => {
                                println!("Port mapping error: {}", err);
                                None
                            },
                            Ok(tcp_port) => {
                                match gateway.add_any_port(PortMappingProtocol::UDP, SocketAddrV4::new(local_addr.ip().clone(), udp_port), 0, "UDP") {
                                    Err(ref err) => {
                                        println!("Port mapping error: {}", err);
                                        None
                                    },
                                    Ok(udp_port) => {
                                        Some(SocketAddr::V4(SocketAddrV4::new(external_addr, tcp_port)))
                                    },
                                }
                            },
                        }
                    },
                }
            },
        },
        _ => { println!("Can not unpack the socket info for the internal host information"); None },
    };
    result
}