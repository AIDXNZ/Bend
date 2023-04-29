use anyhow::Result;
use quinn::{congestion, ClientConfig, Endpoint, ServerConfig};
use std::{
    collections::{btree_set::Range, hash_map::DefaultHasher, BTreeSet},
    error::Error,
    hash::{Hash, Hasher},
    io::{Cursor, Write},
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use crate::protocol::SyncMessage;

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
    pub out_going_connections: Arc<RwLock<Vec<SocketAddr>>>,
    pub store: BTreeSet<String>,
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
            out_going_connections: Arc::new(RwLock::new(Vec::new())),
            store: BTreeSet::new(),
        }
    }

    async fn server(&mut self) -> Result<(), Box<dyn Error>> {
        let (server_config, server_cert) = Self::configure_server()?;
        // Bind this endpoint to a UDP socket on the given server address.
        let endpoint = Endpoint::server(server_config, Self::server_addr())?;

        // Start iterating over incoming connections.
        while let Some(conn) = endpoint.accept().await {
            let mut connection = conn.await?;

            // Save connection somewhere, start transferring, receiving data
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
        //endpoint.set_default_client_config(ClientConfig::with_native_roots());
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

    pub fn calc_range_id(&mut self, start: String, end: String) -> String {
        let mut encoded_incrementally = Vec::new();
        let mut encoder = bao::encode::Encoder::new(Cursor::new(&mut encoded_incrementally));

        for ele in self.store.clone().range(start..end).into_iter() {
            encoder.write_all(ele.as_bytes()).unwrap();
        }
        let hash = encoder.finalize().unwrap();
        return hash.to_string();
    }

    pub fn handle_msg(
        &mut self,
        mut msg: SyncMessage,
    ) -> Result<Option<(SyncMessage, SyncMessage)>> {
        let mut my_range_hash = self
            .clone()
            .calc_range_id(msg.start.clone(), msg.end.clone());
        //Start Comparison
        if msg.id == my_range_hash {
            return Ok(None);
        } else {
            let mut count = 0;
            let mut buf1: Vec<String> = Vec::new();
            let mut buf2 = Vec::new();
            let length = msg.range_len as i32;

            let mut v: Vec<&String> = self
                .store
                .range(msg.start.clone()..msg.end.clone())
                .collect();
            for ele in v {
                if count >= (length / 2) {
                    buf2.push(ele.to_string())
                } else {
                    buf1.push(ele.to_string())
                }
                count += 1;
            }

            let mut r1 = SyncMessage {
                id: "".to_string(),
                len: buf1.len(),
                start: buf1.first().unwrap().to_string(),
                end: buf1.last().unwrap().to_string(),
                range_len: buf1.len() as i32,
            };
            r1.id = hash3(buf1.clone());

            let mut r2 = SyncMessage {
                id: "".to_string(),
                len: buf2.len(),
                start: buf2.first().unwrap().to_string(),
                end: buf2.last().unwrap().to_string(),
                range_len: buf2.len() as i32,
            };
            r2.id = hash3(buf2.clone());
            return Ok(Some((r1, r2)));
        }
    }
}

fn hash3(v: Vec<String>) -> String{
    let mut encoded_incrementally = Vec::new();
    let mut encoder = bao::encode::Encoder::new(Cursor::new(&mut encoded_incrementally));

    for ele in v {
        encoder.write_all(ele.as_bytes()).unwrap();
    }
    let hash = encoder.finalize().unwrap();
    return hash.to_string();
}
