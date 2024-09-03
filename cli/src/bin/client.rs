use std::{net::SocketAddr, num::NonZeroUsize};

use mptcp::stream::MptcpStream;
use tokio::net::TcpListener;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cli {
    /// The server address
    pub servers: Vec<String>,
    pub local_bind: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = std::env::args().nth(1).unwrap();
    let args: Cli = serde_json::from_str(tokio::fs::read_to_string(&cfg).await?.as_str())?;
    let streams = args.servers.len();
    let local_bind = TcpListener::bind(args.local_bind).await?;
    while let Ok((mut s, _)) = local_bind.accept().await {
        let servers = args.servers.clone();
        s.set_nodelay(true)?;
        tokio::spawn(async move {
            let address = servers
                .into_iter()
                .map(|x| x.parse::<SocketAddr>().unwrap());
            let mut stream = MptcpStream::connect(address, NonZeroUsize::new(streams).unwrap())
                .await
                .unwrap();
            let _ = tokio::io::copy_bidirectional(&mut s, &mut stream).await;
        });
    }
    Ok(())
}
