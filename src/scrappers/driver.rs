use std::process::Command;
use std::time::Duration;
use thirtyfour::error::WebDriverError;
use thirtyfour::{DesiredCapabilities, WebDriver};

pub async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    Command::new("/Users/pedrocorreialuis/Downloads/chromedriver-mac-arm64/chromedriver")
        .arg("--port=53466").spawn()?;

    tokio::time::sleep(Duration::from_secs(1)).await;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:53466", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}