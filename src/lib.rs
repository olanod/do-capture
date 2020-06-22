use fantoccini::{error as fan_err, Client};
use futures::prelude::*;
use thiserror::Error;
use url::Url;

pub struct Session {
    client: Client,
}

impl Session {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self {
            client: Client::new("http://localhost:4444")
                .map_err(Error::from)
                .await?,
        })
    }

    pub async fn capture(&mut self, document_url: Url) -> Result<Vec<u8>, Error> {
        self.client.goto(document_url.as_str()).await?;
        Ok(self.client.screenshot().await?)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Couldn't connect to browser")]
    Connection(#[from] fan_err::NewSessionError),
    #[error("Browser error")]
    Browser(#[from] fan_err::CmdError),
}
