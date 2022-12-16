use std::net::SocketAddrV4;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

struct Session {
    local_port: i32,
    remote_addr: SocketAddrV4,
}

impl Session {
    fn new(l: i32, s: &str) -> anyhow::Result<Self> {
        let addr = s.parse()?;
        Ok(Self {
            local_port: l,
            remote_addr: addr,
        })
    }

    fn local_addr(&self) -> String {
        format!("{}:{}", "0.0.0.0", self.local_port)
    }
}

async fn handle(s: &Session) -> anyhow::Result<()> {
    let listener = TcpListener::bind(s.local_addr()).await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let r = transfer(inbound, s.remote_addr.to_string());
        //let r2 = r.await.map(|_|{})?;
        tokio::spawn(r);
    }
    Ok(())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> anyhow::Result<()> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let first = Session::new(2000, "172.18.18.11:22")?;
    let second = Session::new(2001, "172.18.18.11:22")?;
    //let r = tokio::spawn(async move {handle(&first).await} );
    //r.await?.expect("TODO: panic message");

    tokio::try_join!(handle(&first), handle(&second))?;
    Ok(())
}
