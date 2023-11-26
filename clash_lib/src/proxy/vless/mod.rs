mod vless_impl;

use super::transport;
use super::transport::TLSOptions;
use super::{
    options::{GrpcOption, WsOption},
    utils::new_tcp_stream,
    AnyOutboundHandler, AnyStream, CommonOption, OutboundHandler, OutboundType,
};
use crate::app::dispatcher::{BoxedChainedDatagram, BoxedChainedStream};
use crate::app::dns::ThreadSafeDNSResolver;

use crate::session::{Session, SocksAddr};
use async_trait::async_trait;

use std::io;
use std::sync::Arc;
use uuid::{Uuid, Version};

pub struct Opts {
    pub name: String,
    pub common_opts: CommonOption,
    pub server: String,
    pub port: u16,
    pub password: String,
    pub cipher: String,
    pub udp: bool,
}

pub struct Handler {
    opts: Opts,
}

impl Handler {
    pub fn new(opts: Opts) -> Self {
        Self { opts }
    }
}

impl From<Handler> for AnyOutboundHandler {
    fn from(h: Handler) -> Self {
        Arc::new(h)
    }
}

#[async_trait]
impl OutboundHandler for Handler {
    #[doc = " The name of the outbound handler"]
    fn name(&self) -> &str {
        &self.opts.name
    }

    fn proto(&self) -> OutboundType {
        OutboundType::Vless
    }

    async fn remote_addr(&self) -> Option<SocksAddr> {
        Some(SocksAddr::Domain(self.opts.server.clone(), self.opts.port))
    }

    async fn support_udp(&self) -> bool {
        self.opts.udp
    }

    async fn connect_stream(
        &self,
        sess: &Session,
        resolver: ThreadSafeDNSResolver,
    ) -> io::Result<BoxedChainedStream> {
        todo!()
    }

    async fn proxy_stream(
        &self,
        s: AnyStream,
        sess: &Session,
        _: ThreadSafeDNSResolver,
    ) -> io::Result<AnyStream> {
        todo!()
    }

    async fn connect_datagram(
        &self,
        sess: &Session,
        resolver: ThreadSafeDNSResolver,
    ) -> io::Result<BoxedChainedDatagram> {
        todo!()
    }
}
