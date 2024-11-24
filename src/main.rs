use std::io;

use app::App;
use clap::Parser;
use mpd_client::client::Client;
use tokio::net::TcpStream;

mod app;
mod cli;
mod ui;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    // mpd connection
    let addr = format!("{}:{}", args.host, args.port);
    let connection = TcpStream::connect(addr).await.unwrap();
    let (client, _) = Client::connect(connection).await.unwrap();
    println!("connected successfully");

    // create app
    let mut app = App::init(io::stdout(), client).unwrap();

    // run app (aka the actual functionality)
    match app.run() {
        Ok(()) => {}
        Err(error) => println!("ERROR: {:?}", error),
    };

    // destroy app (important to make sure the terminal is restored to normal)
    app.teardown().unwrap();
}
