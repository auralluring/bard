use async_mpd::{cmd, MpdClient};

#[tokio::main]
async fn main() -> Result<(), async_mpd::Error> {
    let mut mpd = MpdClient::new();
    mpd.connect("localhost:6600").await?;

    println!("{:?}", mpd.status().await?);
    Ok(())
}
