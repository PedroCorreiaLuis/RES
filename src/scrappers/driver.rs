use std::env;
use std::process::Command;
use std::time::Duration;
use thirtyfour::error::WebDriverError;
use thirtyfour::{DesiredCapabilities, WebDriver};

pub async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let driver_path: &str =
        &*env::var("DRIVER_PATH").expect("env variable `DRIVER_PATH` should be set");
    Command::new(driver_path).arg("--port=53466").spawn()?;

    tokio::time::sleep(Duration::from_secs(1)).await;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:53466", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}
