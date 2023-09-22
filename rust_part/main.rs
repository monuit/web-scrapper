extern crate reqwest;
extern crate uuid;

use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use uuid::Uuid;
use serde_json::json;
use tokio::net::{TcpListener, TcpStream};
use sha2::{Sha256, Digest};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::io::{Error, ErrorKind};
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::time::{sleep, Duration};
use arrow::array::{StringArray, Int32Array};
use arrow::record_batch::RecordBatch;
use parquet::file::properties::WriterProperties;
use parquet::write::{FileWriter, ArrowWriter};



async fn make_request(url: &str) -> reqwest::Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

async fn save_request_log(request: &str, url: &str) {
    let timestamp = chrono::Local::now().to_rfc3339();
    let log_data = json!({
        "request": request,
        "url": url,
        "timestamp": timestamp
    });
    let file_name = format!("requests/{}.json", Uuid::new_v4());
    let path = Path::new(&file_name);
    let mut file = File::create(&path).unwrap();
    file.write_all(log_data.to_string().as_bytes()).await.unwrap();
}

async fn save_temp_data(data: &str, identifier: &str) {
    let file_name = format!("temp_data/{}.txt", identifier);
    let path = Path::new(&file_name);
    let mut file = File::create(&path).unwrap();
    file.write_all(data.as_bytes()).await.unwrap();
}



const MAX_RETRIES: u8 = 3;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let mut attempt = 0;

    loop {
        match stream.read(&mut buffer).await {
            Ok(_) => break,
            Err(e) if attempt < MAX_RETRIES => {
                attempt += 1;
                println!("Failed to read from socket. Attempt: {}. Error: {}", attempt, e);
                sleep(Duration::from_secs(5)).await;
                continue;
            }
            Err(e) => return Err(Box::new(Error::new(ErrorKind::Other, format!("Failed after {} attempts. Last error: {}", MAX_RETRIES, e)))),
        }
    }

    // Decrypt using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let decrypted_data = hasher.finalize();

    // Process decrypted data here
    process_decrypted_data(decrypted_data).await?;

    // Save the request log
    save_request_log("GET", "http://example.com").await?;

    // Save the data temporarily
    save_temp_data("YourDataHere", "YourIdentifierHere").await?;

    Ok(())
}

async fn process_decrypted_data(data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the decrypted data from bytes to a UTF-8 string
    let data_string = String::from_utf8(data)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Invalid UTF-8 sequence: {}", e)))?;

    // Print the data string
    println!("Decrypted data: {}", data_string);

    Ok(())
}


async fn save_log_to_json(log_data: LogData) -> std::io::Result<()> {
    // Convert the log data to a JSON string
    let json = serde_json::to_string(&log_data)
        .expect("Failed to serialize log data");

    // Open a file in write-only mode
    let mut file = File::create("log.json")?;

    // Write the JSON data to the file
    file.write_all(json.as_bytes())?;

    Ok(())
}


// Assume we get this data by scraping the website
let scraped_data = vec![
    ("field1", DataType::Utf8),
    ("field2", DataType::Int32),
    // Add more fields as needed
];

let mut fields = Vec::new();
for (name, data_type) in scraped_data {
    fields.push(Field::new(name, data_type, false));
}

let schema = Arc::new(Schema::new(fields));


async fn save_temp_data_to_parquet() -> Result<(), Box<dyn std::error::Error>> {
    let field1 = StringArray::from(vec!["value1", "value2"]);
    let field2 = Int32Array::from(vec![1, 2]);

    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(field1),
            Arc::new(field2),
            // Add more columns as needed
        ],
    )?;

    let file = File::create("temp.parquet")?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(props))?;

    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Could not bind socket");
    loop {
        let (stream, _) = listener.accept().await.expect("Could not accept client");
        tokio::spawn(handle_client(stream));
    }
}
