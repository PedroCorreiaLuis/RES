use std::env;
use std::process::Command;
use std::time::Duration;
use thirtyfour::error::WebDriverError;
use thirtyfour::{DesiredCapabilities, WebDriver};

pub async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let driver_path: &str =
        &*env::var("DRIVER_PATH").expect("env variable `DRIVER_PATH` should be set");
    Command::new(driver_path).arg("--port=53465").spawn()?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    let driver: WebDriver = match driver_path {
        path if path.contains("chromedriver") => {
            WebDriver::new("http://localhost:53465", DesiredCapabilities::chrome()).await?
        }
        path if path.contains("safaridriver") => {
            WebDriver::new("http://localhost:53465", DesiredCapabilities::safari()).await?
        }
        _ => panic!("Unsupported driver path"),
    };

    driver.maximize_window().await?;
    Ok(driver)
}
