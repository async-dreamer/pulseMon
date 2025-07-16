mod cpu_metrics;
mod memory_metrics;
mod disk_metrics;

use tokio::time::{sleep, Duration};
use serde_json::Value;
use reqwest::Error;
use serde::Serialize;
use std::env;
use dotenvy::dotenv;

#[derive(Debug, Serialize)]
struct SystemMetrics {
    cpu_load: f64,
    disk_metrics: Value,
    memory_metrics: Value
}

async fn send_metrics_to_api(metrics: &SystemMetrics, url: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client.post(url)
        .json(metrics)
        .send()
        .await?;
    if response.status().is_success() {
        println!("Metrics sent successfully!");
    } else {
        println!("Failed to send metrics: HTTP {}", response.status());
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // Load .env variables
    dotenv().ok();
    let url = env::var("METRICS_API_URL")
        .expect("METRICS_API_URL must be set in .env");

    let time_to_collect_metrics:u64 = env::var("METRICS_TIME_TO_COLLECT")
        .expect("METRICS_TIME_TO_COLLECT must be set in .env").parse().unwrap();

    loop {
        // Collect CPU stats
        // Update previous stats for the next iteration
        let prev_stat = cpu_metrics::CpuStat::read_from_file()
            .expect("Failed to read CPU stats");

        // Wait for 5 second to calculate CPU load
        sleep(Duration::from_secs(5)).await;

        // Read current CPU stats
        let curr_stat = cpu_metrics::CpuStat::read_from_file()
            .expect("Failed to read CPU stats");

        let cpu_load = cpu_metrics::CpuStat::calculate_cpu_load(&prev_stat, &curr_stat);

        // Fetch memory metrics
        let memory_metrics = memory_metrics::get_memory_info().unwrap_or_else(|_| serde_json::json!({"error": "Failed to collect memory metrics"}));

        // Collect disk stats
        let disk_metrics = disk_metrics::get_disk_usage().unwrap_or_else(|_| serde_json::json!({"error": "Failed to collect disk metrics"}));

        // Create metrics object with the new JSON field
        let metrics = SystemMetrics {
            cpu_load,
            disk_metrics,
            memory_metrics,
        };

        // Send metrics to API
        if let Err(e) = send_metrics_to_api(&metrics, &url).await {
            eprintln!("Error: {}", e);
        }

        // collect metrics every seconds by .env file
        sleep(Duration::from_secs(time_to_collect_metrics)).await;
    }
}