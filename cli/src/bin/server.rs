use std::{net::SocketAddr, num::NonZeroUsize};

use mptcp::listen::MptcpListener;
use tokio::net::TcpStream;

#[derive(Debug,  serde::Serialize, serde::Deserialize)]
pub struct Cli {
    /// The listen address
    pub listen: String,
    pub streams: NonZeroUsize,
    pub remote: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = std::env::args().nth(1).unwrap();
    let args: Cli = serde_json::from_str(tokio::fs::read_to_string(&cfg).await?.as_str())?;
    let bind_addr = args.listen.parse::<SocketAddr>()?;
    let mut listener = MptcpListener::bind(bind_addr, args.streams).await.unwrap();
    while let Ok(mut s) = listener.accept().await {
        let remote = args.remote.parse::<SocketAddr>()?;
        tokio::spawn(async move {
            let mut remote_stream = TcpStream::connect(remote).await?;
            remote_stream.set_nodelay(true)?;
            let _ = tokio::io::copy_bidirectional(&mut remote_stream, &mut s).await;
            Ok::<_, anyhow::Error>(())
        });
    }
    Ok(())
}
