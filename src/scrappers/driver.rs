use rand::prelude::ThreadRng;
use rand::Rng;
use std::env;
use std::process::Command;
use std::time::Duration;
use thirtyfour::error::WebDriverError;
use thirtyfour::{
    CapabilitiesHelper, ChromeCapabilities, ChromiumLikeCapabilities, DesiredCapabilities,
    SafariCapabilities, WebDriver,
};

pub async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let mut rng: ThreadRng = rand::thread_rng();
    let port_number: i32 = rng.gen_range(1000..=6000);
    let port_host = format!("http://localhost:{}", port_number);

    let driver_path: &str =
        &*env::var("DRIVER_PATH").expect("env variable `DRIVER_PATH` should be set");

    Command::new(driver_path)
        .arg(format!("--port={}", port_number))
        .spawn()?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    let driver: WebDriver = match driver_path {
        path if path.contains("chromedriver") => {
            let mut caps: ChromeCapabilities = DesiredCapabilities::chrome();
            caps.add_arg("--disable-blink-features=AutomationControlled")?;
            caps.add_arg("--window-size=1920,1080")?;
            caps.set_javascript_enabled(true)?;
            WebDriver::new(port_host, caps).await?
        }
        path if path.contains("safaridriver") => {
            let mut caps: SafariCapabilities = DesiredCapabilities::safari();
            caps.set_javascript_enabled(true)?;
            WebDriver::new(port_host, caps).await?
        }
        _ => panic!("Unsupported driver path"),
    };

    Ok(driver)
}
