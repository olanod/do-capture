use fantoccini::{error as fan_err, Client};
use futures::prelude::*;
use thiserror::Error;
use tokio::process::{Child, Command};
use url::Url;

pub struct Session {
    client: Client,
    _driver: Child,
}

type ScreenSize = (i32, i32);

pub mod size {
    pub const PHONE: super::ScreenSize = (400, 700);
}

impl Session {
    pub async fn new() -> Result<Self, Error> {
        let driver = Command::new("geckodriver")
            .kill_on_drop(true)
            .arg("--connect-existing")
            .args(&["--marionette-port", "2828"])
            .spawn()?;
        let client = Client::new("http://localhost:4444")
            .map_err(Error::from)
            .await?;
        Ok(Self {
            client,
            _driver: driver,
        })
    }

    pub async fn capture(&mut self, document_url: Url, size: ScreenSize) -> Result<Vec<u8>, Error> {
        self.client.new_window(true).await?;
        self.client.set_window_size(size.0, size.1).await?;
        self.client.goto(document_url.as_str()).await?;
        Ok(self.client.screenshot().await?)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("WebDriver error")]
    WebDriver(#[from] std::io::Error),
    #[error("Couldn't connect to browser")]
    Connection(#[from] fan_err::NewSessionError),
    #[error("Browser error")]
    Browser(#[from] fan_err::CmdError),
}
