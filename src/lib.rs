use fantoccini::{error as fan_err, Client};
use futures::prelude::*;
use std::cell::RefCell;
use std::process::Stdio;
use thiserror::Error;
use tokio::process::{Child, Command};
use url::Url;
use which::which;

#[derive(Debug)]
pub struct Session {
    client: RefCell<Client>,
    _driver: Child,
    _browser: Child,
}

type ScreenSize = (i32, i32);

pub mod size {
    pub const PHONE: super::ScreenSize = (360, 740);
    pub const TABLET: super::ScreenSize = (768, 1024);
}

impl Session {
    pub async fn new() -> Result<Self, Error> {
        let firefox = which("firefox").map_err(|_| Error::MissingDependency("firefox".into()))?;
        let geckodriver =
            which("geckodriver").map_err(|_| Error::MissingDependency("geckodriver".into()))?;
        Command::new(firefox.clone())
            .arg("-headless")
            .args(&["-CreateProfile", "web-capture"])
            .stderr(Stdio::null())
            .spawn()?
            .await
            .expect("create browser profile");
        let _browser = Command::new(firefox)
            .kill_on_drop(true)
            .arg("-headless")
            .arg("-marionette")
            .args(&["-P", "web-capture"])
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()?;
        let _driver = Command::new(geckodriver)
            .kill_on_drop(true)
            .arg("--connect-existing")
            .args(&["--marionette-port", "2828"])
            .spawn()?;
        let client = RefCell::new(
            Client::new("http://localhost:4444")
                .map_err(Error::from)
                .await?,
        );
        Ok(Self {
            client,
            _driver,
            _browser,
        })
    }

    pub async fn capture(&self, document_url: Url, size: ScreenSize) -> Result<Vec<u8>, Error> {
        let mut client = self.client.borrow_mut();
        client.new_window(true).await?;
        let mut windows = client.windows().await?;
        client.switch_to_window(windows.pop().unwrap()).await?;
        client.set_window_size(size.0, size.1).await?;
        client.goto(document_url.as_str()).await?;
        let capture = client.screenshot().await?;
        client.close_window().await?;
        client.switch_to_window(windows.pop().unwrap()).await?;
        Ok(capture)
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
    #[error("Browser error {0}")]
    Browser(#[from] fan_err::CmdError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn capture_two() -> Result<(), Error> {
        let s = Session::new().await?;

        s.capture("http://duck.com".parse().unwrap(), size::PHONE)
            .await?;
        let cap = s
            .capture("http://google.com".parse().unwrap(), size::TABLET)
            .await?;
        assert!(!cap.is_empty());
        Ok(())
    }
}
