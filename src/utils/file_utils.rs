use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

pub async fn get_file_read(path: &str) -> Result<File, std::io::Error> {
    let file_read: File = OpenOptions::new().read(true).open(path).await?;
    Ok(file_read)
}

pub async fn get_file_write_append(path: &str) -> Result<File, std::io::Error> {
    let file_write: File = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .await?;

    Ok(file_write)
}

pub async fn get_file_write_truncate(path: &str) -> Result<File, std::io::Error> {
    let file_write: File = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .await?;

    Ok(file_write)
}

pub async fn get_content_as_string(mut file: File) -> Result<String, std::io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content).await?;

    Ok(content)
}

pub async fn get_content_lines(file: File) -> Result<Vec<String>, std::io::Error> {
    let reader = BufReader::new(file);

    // Read the file line by line asynchronously
    let mut lines = reader.lines();
    let mut vec_content = Vec::new();
    while let Some(line) = lines.next_line().await? {
        vec_content.push(line);
    }

    Ok(vec_content)
}

pub async fn write_to_file(file: &mut File, content: String) -> Result<(), std::io::Error> {
    file.write_all(content.as_bytes()).await?;
    Ok(())
}
