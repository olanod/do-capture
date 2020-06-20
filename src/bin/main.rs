use async_std::{io, prelude::*};
use do_capture::{capture, Format};
use std::env::args;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = args().skip(1).next().unwrap();
    let screenshot = capture(url.parse()?, Format::JPEG).await?;
    io::stdout().write_all(&screenshot);
    Ok(())
}
