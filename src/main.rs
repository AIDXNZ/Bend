use std::{collections::{BTreeSet, btree_set::Range}, hash::Hash, net::SocketAddr, sync::Arc, error::Error};
use bend::{node::Node, protocol::Sync};
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    
    block_on(daemon())
}

async fn daemon() {

    let mut bend = Node::new();
    let (server, _certs) = bend.make_server_endpoint("127.0.0.1:4238".parse().unwrap()).unwrap();
    println!("Listening on: {:?}", server.local_addr().unwrap());

    let mut db = BTreeSet::new();
    db.insert("nvdfvjksnfvjdkdv");
    db.insert("ndasjfgnerwiebewb");
    db.insert("fsd dvnenvsdnvdv");
    db.insert("dfbgnsdjsdavnvjkanlva");
    db.insert("cbsdjhcbshjks");

    let msg = Sync{
        id: "Aidan".to_string(),
        len: 10,
        start: "cbsdjhcbshjks".to_string(),
        end: "3".to_string(),
        range_len: 3,
    };


    let mut v = Vec::new();
    // Start sync
    let nl = db.range("cbsdjhcbshjks".."nvdfvjksnfvjdkdv"); 
    for &ele in nl {
        if ele != msg.start {
            v.push(ele.to_string());
        }
    }
    if v.len() as i32 >= msg.range_len || msg.range_len == 1 {
        println!("Is out of sync")
    }

    println!("{:?}", v.len());
    println!("Elements needed to sync: {:?}", v);

    let msg2 = Sync{
        id: "Aidan".to_string(),
        len: 10,
        start: "dfbgnsdjsdavnvjkanlva".to_string(),
        end: "3".to_string(),
        range_len: 1,
    };
    let mut v2 = Vec::new();
    // Start sync
    let nl2 = db.range("dfbgnsdjsdavnvjkanlva".."ndasjfgnerwiebewb"); 
    for &ele in nl2 {
        if ele != msg2.start {
            v2.push(ele.to_string());
        }
    }

    println!("{:?}", v2.len());
    println!("2: {:?}", v2);

    loop {
        while let Some(con) = server.accept().await {
            let mut peer = con.await.unwrap();
            println!("Connected to {:?}", peer.remote_address());
            match bend.out_going_connections.try_write() {
                Ok(mut guard) => {
                    let addr = peer.remote_address().clone();
                    guard.push(addr);
                    println!("{:?}", bend.out_going_connections.read().unwrap())
                },
                Err(_) => todo!(),
            }
        }
    }
}

