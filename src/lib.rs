use fantoccini::{error as fan_err, Client};
use futures::prelude::*;
use std::process::Stdio;
use thiserror::Error;
use tokio::process::{Child, Command};
use url::Url;
use which::which;

pub struct Session {
    client: Client,
    _driver: Child,
    _browser: Child,
}

type ScreenSize = (i32, i32);

pub mod size {
    pub const PHONE: super::ScreenSize = (400, 700);
}

impl Session {
    pub async fn new() -> Result<Self, Error> {
        let firefox = which("firefox").map_err(|_| Error::MissingDependency("firefox".into()))?;
        let geckodriver =
            which("geckodriver").map_err(|_| Error::MissingDependency("geckodriver".into()))?;
        let _browser = Command::new(firefox)
            .kill_on_drop(true)
            .arg("-headless")
            .arg("-marionette")
            .stdout(Stdio::null())
            .spawn()?;
        let _driver = Command::new(geckodriver)
            .kill_on_drop(true)
            .arg("--connect-existing")
            .args(&["--marionette-port", "2828"])
            .spawn()?;
        let client = Client::new("http://localhost:4444")
            .map_err(Error::from)
            .await?;
        Ok(Self {
            client,
            _driver,
            _browser,
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
    #[error("Missing {0} dependency")]
    MissingDependency(String),
    #[error("WebDriver error")]
    WebDriver(#[from] std::io::Error),
    #[error("Couldn't connect to browser")]
    Connection(#[from] fan_err::NewSessionError),
    #[error("Browser error")]
    Browser(#[from] fan_err::CmdError),
}
