use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'H', long, default_value = "localhost", env = "MPD_HOST")]
    pub host: String,
    #[arg(short = 'P', long, default_value = "6600", env = "MPD_PORT")]
    pub port: u16,
}
