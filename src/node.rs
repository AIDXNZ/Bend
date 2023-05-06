use anyhow::Result;
use diamond_types::list::OpLog;
use futures::FutureExt;
use quinn::{congestion, ClientConfig, Connection, Endpoint, ServerConfig};
use std::{
    collections::{btree_set::Range, hash_map::DefaultHasher, BTreeSet},
    error::Error,
    hash::{Hash, Hasher},
    io::{Cursor, Write},
    net::SocketAddr,
    sync::{Arc, RwLock},
    thread,
};

use crate::{net, protocol::SyncMessage};

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
    pub op_log: OpLog,
    pub endpoint: Endpoint,
    pub cli_endpoint: Endpoint,
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
        let network = net::Net::new("127.0.0.1:0".parse().unwrap());
        Self {
            config: BendConfig::default(),
            out_going_connections: Arc::new(RwLock::new(Vec::new())),
            store: BTreeSet::new(),
            op_log: OpLog::new(),
            endpoint: network.endpoint,
            cli_endpoint: network.client_endpoint,
        }
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
    pub async fn handle_conn(&self, mut conn: Connection) {
        let (mut send, mut recv) = conn.accept_bi().await.unwrap();
               
        thread::spawn(|| async move{
            loop {
                let mut buf = Vec::new();
                while let Some(req) = recv.read_chunks(&mut buf).await.unwrap(){
                    println!("{:?}",req);
                }
            }
        });
    }

    fn handle_msg(&mut self, mut msg: SyncMessage) -> Result<Option<(SyncMessage, SyncMessage)>> {
        let mut my_range_hash = self
            .clone()
            .calc_range_id(msg.start.clone(), msg.end.clone());
        //Start Comparison
        if msg.id == my_range_hash {
            return Ok(None);
        } else if msg.range_len == 0 {
            todo!();
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
                items: None,
            };
            r1.id = hash3(buf1.clone());

            let mut r2 = SyncMessage {
                id: "".to_string(),
                len: buf2.len(),
                start: buf2.first().unwrap().to_string(),
                end: buf2.last().unwrap().to_string(),
                range_len: buf2.len() as i32,
                items: None,
            };
            r2.id = hash3(buf2.clone());
            return Ok(Some((r1, r2)));
        }
    }
}

fn hash3(v: Vec<String>) -> String {
    let mut encoded_incrementally = Vec::new();
    let mut encoder = bao::encode::Encoder::new(Cursor::new(&mut encoded_incrementally));

    for ele in v {
        encoder.write_all(ele.as_bytes()).unwrap();
    }
    let hash = encoder.finalize().unwrap();
    return hash.to_string();
}
