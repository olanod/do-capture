use async_std::task;
pub use headless_chrome::protocol::page::ScreenshotFormat;
use headless_chrome::Browser;
use thiserror::Error;
use url::Url;

type Result = std::result::Result<Vec<u8>, Error>;

pub enum Format {
    JPEG,
    PNG,
}

pub async fn capture(document_url: Url, format: Format) -> Result {
    task::spawn_blocking(move || {
        run_browser(document_url, format).map_err(|e| Error::Browser(e.to_string()))
    })
    .await
}

fn run_browser(
    doc: Url,
    format: Format,
) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to(&doc.to_string())?;
    tab.wait_until_navigated()?;

    let screen = match format {
        Format::JPEG => tab.capture_screenshot(ScreenshotFormat::JPEG(Some(80)), None, true)?,
        Format::PNG => tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?,
    };
    Ok(screen)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("oupsy!")]
    Browser(String),
}
