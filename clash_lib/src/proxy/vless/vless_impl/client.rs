// client should impl different vless inner protocol.

use uuid::Uuid;

use crate::common::utils;

use super::proto::vless::Addons;


pub enum Command {
    TCP,
    UDP,
    Mux,
}

pub enum AddrType {
    IPv4,
    IPv6,
    DomainName,
}

pub struct DstAddr<'a> {
    pub udp: bool,
    pub addr_type: AddrType,
    pub addr: &'a [u8],
    pub port: u16,
    pub mux: bool,
}

pub struct Client {
    pub uuid: Uuid,
    pub addons: Option<Addons>,
}

impl Client {
    pub fn new(uuid_str: &str, addons: Option<Addons>) -> Self {
        Self {
            uuid: utils::uuid_map(uuid_str),
            addons,
        }
    }
}