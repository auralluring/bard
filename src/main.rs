use mpd_client::client::Client;
use tokio::net::TcpStream;

mod cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    let addr = format!("{}:{}", args.host, args.port);
    println!("connecting to {addr}");
    let connection = TcpStream::connect(addr).await.unwrap();
    let (client, _) = Client::connect(connection).await.unwrap();
}
