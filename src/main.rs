use std::{collections::BTreeSet, hash::Hash, net::SocketAddr, sync::Arc};
use crdts::{List, CmRDT};
use quinn::{ServerConfig, Endpoint, congestion};
use anyhow::{Result};
use futures::executor::block_on;

#[derive(Clone)]
pub struct BendConfig {
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub port: Option<u16>,
    pub path: String,
}
#[derive(Clone)]
pub struct Node {
    pub config: BendConfig,
    pub out_going_connections: Vec<SocketAddr>,

}
impl Default for BendConfig {
    fn default() -> Self {
        BendConfig {
            cert_path: None,
            key_path: None,
            port: None,
            path: "".to_string(),
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Self {config: BendConfig::default(), out_going_connections: Vec::new(),}
    }

    pub fn make_server_endpoint(&mut self, bind_addr: SocketAddr) -> Result<(Endpoint, Vec<u8>)> {
        let (server_config, server_cert) = Self::configure_server()?;
        let endpoint = Endpoint::server(server_config, bind_addr)?;
        Ok((endpoint, server_cert))
    }
    
    
    fn configure_server() -> Result<(ServerConfig, Vec<u8>)> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let cert_der = cert.serialize_der().unwrap();
        let priv_key = cert.serialize_private_key_der();
        let priv_key = rustls::PrivateKey(priv_key);
        let cert_chain = vec![rustls::Certificate(cert_der.clone())];
    
        let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key).unwrap();
        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());
        #[cfg(any(windows, os = "linux"))]
        transport_config.mtu_discovery_config(Some(quinn::MtuDiscoveryConfig::default()));
    
        Ok((server_config, cert_der))
    }

    
}

#[tokio::main]
async fn main() {
    
    block_on(daemon())
}

async fn daemon() {

    let mut bend = Node::new();
    let (server, _certs) = bend.make_server_endpoint("0.0.0.0:4238".parse().unwrap()).unwrap();
    println!("Listening on: {:?}", server.local_addr().unwrap());

    










    loop {
        while let Some(con) = server.accept().await {
            let mut peer = con.await.unwrap();
            println!("Connected to {:?}", peer.remote_address())
        }
    }
}



