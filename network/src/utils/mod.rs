use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use regex::Regex;

pub fn get_local_ip() -> Option<IpAddr> { get() }


#[cfg(target_os = "linux")]
fn get() -> Option<IpAddr> {
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        let host = &cap[2];
        if host != "127.0.0.1" {
            if let Ok(addr) = host.parse::<Ipv4Addr>() {
                return Some(IpAddr::V4(addr))
            } else if let Ok(addr) = host.parse::<Ipv6Addr>() {
                return Some(IpAddr::V6(addr))
            }
        }
    }
    None
}

// TODO: windows version regex
#[cfg(target_os = "windows")]
fn get() -> Option<IpAddr> {
    let output = Command::new("ipconfig")
        .output()
        .expect("failed to execute `ipconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap_or("".into());

    let re = Regex::new(r#"(?m)^.*IPv4 Address.*?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        let host = &cap[1];
        if host != "127.0.0.1" {
            if let Ok(addr) = host.parse::<Ipv4Addr>() {
                return Some(IpAddr::V4(addr))
            } else if let Ok(addr) = host.parse::<Ipv6Addr>() {
                return Some(IpAddr::V6(addr))
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn get() -> Option<IpAddr> {
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        let host = &cap[2];
        if host != "127.0.0.1" {
            if let Ok(addr) = host.parse::<Ipv4Addr>() {
                return Some(IpAddr::V4(addr))
            } else if let Ok(addr) = host.parse::<Ipv6Addr>() {
                return Some(IpAddr::V6(addr))
            }
        }
    }
    None
}