use bend::{node::Node, protocol::{ SyncMessage}};
use futures::executor::block_on;
use std::{
    collections::{btree_set::Range, BTreeSet},
    error::Error,
    hash::Hash,
    net::SocketAddr,
    sync::Arc,
};

#[tokio::main]
async fn main() {
    block_on(daemon())
}

async fn daemon() {
    let mut bend = Node::new();
    let (server, _certs) = bend
        .make_server_endpoint("127.0.0.1:4238".parse().unwrap())
        .unwrap();
    println!("Listening on: {:?}", server.local_addr().unwrap());

    //Insert
    bend.store.insert("nvdfvjksnfvjdkdv".to_string());
    bend.store.insert("ndasjfgnerwiebewb".to_string());
    bend.store.insert("fsd dvnenvsdnvdv".to_string());
    bend.store.insert("dfbgnsdjsdavnvjkanlva".to_string());
    bend.store.insert("cbsdjhcbshjks".to_string());

    let mut msg = SyncMessage {
        //Intentionally set the id to kick off sync
        id: "Aidan".to_string(),
        len: 4,
        start: "cbsdjhcbshjks".to_string(),
        end: "zvdfvjksnfvjdkdv".to_string(),
        range_len: 5,
    };
    

    let resp = bend.handle_msg(msg).unwrap().unwrap();
    println!("{} {} + {:?}", resp.0.id, resp.0.start, resp.0.end);
    println!("{} {}+ {:?}", resp.1.id, resp.1.start, resp.1.end);


    loop {
        while let Some(con) = server.accept().await {
            let mut peer = con.await.unwrap();
            println!("Connected to {:?}", peer.remote_address());
            match bend.out_going_connections.try_write() {
                Ok(mut guard) => {
                    let addr = peer.remote_address().clone();
                    guard.push(addr);
                    println!("{:?}", bend.out_going_connections.read().unwrap())
                }
                Err(_) => todo!(),
            }
        }
    }
}

