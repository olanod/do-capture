use do_capture::Session;
use std::env::args;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = args().skip(1).next().unwrap_or("".into());
    let url = url.parse()?;
    let screenshot = Session::new().await?.capture(url).await?;
    eprintln!("{:?}", screenshot);
    Ok(())
}
