use anyhow::Result;
use crdts::{CmRDT, List};
use futures::executor::block_on;
use quinn::{congestion, ClientConfig, Endpoint, ServerConfig};
use std::{collections::BTreeSet, hash::Hash, net::SocketAddr, sync::Arc, error::Error};

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
        Self {
            config: BendConfig::default(),
            out_going_connections: Vec::new(),
        }
    }

    async fn server(&mut self) -> Result<(), Box<dyn Error>> {
        let (server_config, server_cert) = Self::configure_server()?;
        // Bind this endpoint to a UDP socket on the given server address. 
        let endpoint = Endpoint::server(server_config, Self::server_addr())?;
    
        // Start iterating over incoming connections.
        while let Some(conn) = endpoint.accept().await {
            let mut connection = conn.await?;
    
            // Save connection somewhere, start transferring, receiving data, see DataTransfer tutorial.
        }
    
        Ok(())
    }

    async fn client() -> Result<(), Box<dyn Error>> {
        // Bind this endpoint to a UDP socket on the given client address.
        let mut endpoint = Endpoint::client(Self::client_addr())?;
    
        // Connect to the server passing in the server name which is supposed to be in the server certificate.
        let connection = endpoint.connect(Self::server_addr(), "localhost")?.await?;
    
        // Start transferring, receiving data, see data transfer page.
    
        Ok(())
    }

    fn client_addr() -> SocketAddr {
        "127.0.0.1:4358".parse::<SocketAddr>().unwrap()
    }

    fn server_addr() -> SocketAddr {
        "127.0.0.1:4438".parse::<SocketAddr>().unwrap()
    }

    pub fn make_server_endpoint(&mut self, bind_addr: SocketAddr) -> Result<(Endpoint, Vec<u8>)> {
        let (server_config, server_cert) = Self::configure_server()?;
        let mut endpoint = Endpoint::server(server_config, bind_addr)?;
        endpoint.set_default_client_config(ClientConfig::with_native_roots());
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
    let (server, _certs) = bend
        .make_server_endpoint("0.0.0.0:0".parse().unwrap())
        .unwrap();
    println!("Listening on: {:?}", server.local_addr().unwrap());

    server
        .connect("127.0.0.1:4238".parse::<SocketAddr>().unwrap(), "localhost")
        .unwrap()
        .await
        .unwrap();

    loop {
        while let Some(con) = server.accept().await {
            println!("{:?}", con.remote_address())
        }
    }
}
