use serde::Deserialize;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct Config {
    clients: Vec<String>,
    bind_client: String,
    bind_server: String,
    server: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("udp.json")?;
    //println!("{}", content);
    let x: Config = serde_json::from_str(&content)?;
    println!("{:?}", x);

    let clients = x.clients.clone();
    let to_server: SocketAddr = x.server.parse()?;

    let bind_client = Arc::new(UdpSocket::bind(x.bind_client).await?);
    let bind_server = Arc::new(UdpSocket::bind(x.bind_server).await?);
    let bind_client2 = bind_client.clone();
    let bind_server2 = bind_server.clone();

    let client_to_server = async move {
        let mut buf = [0; 1500];
        loop {
            let len = bind_client.recv(&mut buf).await.expect("recv failed");
            let len2 = bind_server.send_to(&buf[..len], to_server).await.expect("send to failed");
            if len != len2 {
                println!("error happened, while client to server");
            }
            println!("client to server: {:?}", &buf[..len]);
        }
    };
    let a = tokio::spawn(client_to_server);

    let server_to_client = async move {
        let mut buf = [0; 1500];
        loop {
            let len = bind_server2.recv(&mut buf).await.expect("recv failed");

            for to_client in clients.iter() {
                let to_client: SocketAddr = to_client.parse().unwrap();
                let len2 = bind_client2.send_to(&buf[..len], to_client).await.expect("send to failed");
                if len != len2 {
                    println!("error happened, while server to client");
                }
            }
            //println!("server to client: {:?}", &buf[..len]);
        }
    };
    let b = tokio::spawn(server_to_client);

    tokio::try_join!(a, b)?;
    //tokio::spawn(

    Ok(())
}
