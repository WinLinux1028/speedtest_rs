#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::{io::Write, sync::OnceLock, time::Instant};

use rand::RngCore;
use reqwest::{Body, Client};

type Error = Box<dyn std::error::Error>;

static ENDPOINT: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    print!("endpoint> ");
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let _ = ENDPOINT.set(buf);

    println!("Download");
    print!("100KB: ");
    download(100 * 1024).await.unwrap();
    print!("  1MB: ");
    download(1024 * 1024).await.unwrap();
    print!(" 10MB: ");
    download(10 * 1024 * 1024).await.unwrap();
    print!(" 25MB: ");
    download(25 * 1024 * 1024).await.unwrap();
    print!("100MB: ");
    download(100 * 1024 * 1024).await.unwrap();
    println!("Upload");
    print!("100KB: ");
    upload(100 * 1024).await.unwrap();
    print!("  1MB: ");
    upload(1024 * 1024).await.unwrap();
    print!(" 10MB: ");
    upload(10 * 1024 * 1024).await.unwrap();
    print!(" 25MB: ");
    upload(25 * 1024 * 1024).await.unwrap();
    print!("100MB: ");
    upload(100 * 1024 * 1024).await.unwrap();
}

async fn download(size: usize) -> Result<(), Error> {
    let client = Client::new();
    let response = client
        .get(format!("{}?size={}", ENDPOINT.get().ok_or("")?, size))
        .send()
        .await?;
    let start = Instant::now();
    let data = response.bytes().await;
    let end = Instant::now();

    let time = end - start;
    let speed_bps = (data?.len() as f64 * 8.0) / time.as_secs_f64();
    println!("{}", convert_bps_readable(speed_bps));

    Ok(())
}

async fn upload(size: usize) -> Result<(), Error> {
    let mut buf = vec![0; size];
    rand::thread_rng().fill_bytes(&mut buf);

    let client = Client::new();

    let request = client.post(ENDPOINT.get().ok_or("")?).body(Body::from(buf));

    let start = Instant::now();
    request.send().await?;
    let end = Instant::now();

    let time = end - start;
    let speed_bps = (size as f64 * 8.0) / time.as_secs_f64();
    println!("{}", convert_bps_readable(speed_bps));

    Ok(())
}

fn convert_bps_readable(speed_bps: f64) -> String {
    let speed_kbps = speed_bps / 1024.0;
    if speed_kbps < 1.0 {
        return format!("{:5.1}bps", speed_bps);
    }

    let speed_mbps = speed_kbps / 1024.0;
    if speed_mbps < 1.0 {
        return format!("{:5.1}Kbps", speed_kbps);
    }

    let speed_gbps = speed_mbps / 1024.0;
    if speed_gbps < 1.0 {
        return format!("{:5.1}Mbps", speed_mbps);
    }

    let speed_tbps = speed_gbps / 1024.0;
    if speed_tbps < 1.0 {
        return format!("{:5.1}Gbps", speed_gbps);
    }

    format!("{:5.1}Tbps", speed_tbps)
}
