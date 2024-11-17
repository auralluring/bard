use async_mpd::MpdClient;

mod cli;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), async_mpd::Error> {
    let args = cli::Args::parse();
    let mut mpd = MpdClient::new();
    let addr = format!("{}:{}", args.host, args.port);
    println!("connecting to {addr}");
    mpd.connect(addr).await?;
    Ok(())
}
