use std::{collections::BTreeSet, hash::Hash, net::SocketAddr, sync::Arc, error::Error};
use bend::node::Node;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    
    block_on(daemon())
}

async fn daemon() {

    let mut bend = Node::new();
    let (server, _certs) = bend.make_server_endpoint("127.0.0.1:4238".parse().unwrap()).unwrap();
    println!("Listening on: {:?}", server.local_addr().unwrap());

    

    loop {
        while let Some(con) = server.accept().await {
            let mut peer = con.await.unwrap();
            println!("Connected to {:?}", peer.remote_address());
            match bend.out_going_connections.try_write() {
                Ok(mut guard) => {
                    let addr = peer.remote_address().clone();
                    guard.push(addr);
                    println!("{:?}", bend.out_going_connections)
                },
                Err(_) => todo!(),
            }
        }
    }
}

