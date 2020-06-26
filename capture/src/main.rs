use do_capture::{size, Session};
use std::error::Error;
use structopt::StructOpt;
use tokio::io::{self, AsyncWriteExt};
use url::Url;

#[derive(StructOpt)]
struct Args {
    /// Run as a web server
    #[structopt(short = "s", required_unless("url"))]
    server_mode: bool,
    #[structopt(short, default_value = "8080")]
    port: String,
    /// Output capture to stdout of the provided Url
    url: Option<Url>,
}

#[paw::main]
#[tokio::main]
async fn main(args: Args) -> Result<(), Box<dyn Error>> {
    if args.server_mode {
        println!("Server (will be) listening on port {}", args.port);
        return Ok(());
    }
    let url = args.url.unwrap();
    let screenshot = Session::new().await?.capture(url, size::PHONE).await?;
    io::stdout().write_all(&screenshot).await?;
    Ok(())
}
