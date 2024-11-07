use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn get_file_read(path: &str) -> Result<File, std::io::Error> {
    let file_read: File = OpenOptions::new().read(true).open(path).await?;
    Ok(file_read)
}

pub async fn get_file_write(path: &str) -> Result<File, std::io::Error> {
    let file_write: File = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .await?;

    Ok(file_write)
}

pub async fn get_content_as_string(mut file: File) -> Result<String, std::io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content).await?;

    Ok(content)
}

pub async fn write_to_file(file: &mut File, content: String) -> Result<(), std::io::Error> {
    file.write_all(content.as_bytes()).await?;
    Ok(())
}
