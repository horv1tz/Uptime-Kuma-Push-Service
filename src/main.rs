use std::env;
use std::time::Duration;
use tokio::time::interval;
use tokio::signal;
use chrono::Local;

fn log(level: &str, message: &str) {
    let now = Local::now();
    println!("[{}] [{}] {}", now.format("%Y-%m-%d %H:%M:%S"), level, message);
}

#[tokio::main]
async fn main() {
    let push_url = env::var("UPTIME_KUMA_PUSH_URL")
        .expect("ERROR: UPTIME_KUMA_PUSH_URL environment variable is not set.");

    let interval_seconds: u64 = env::var("UPTIME_KUMA_PUSH_INTERVAL_SECONDS")
        .unwrap_or_else(|_| "60".to_string())
        .parse()
        .expect("ERROR: Invalid interval value.");

    log("INFO", &format!("Starting Uptime Kuma push service. URL: {}, Interval: {} seconds", push_url, interval_seconds));

    let mut ticker = interval(Duration::from_secs(interval_seconds));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                match reqwest::get(&push_url).await {
                    Ok(response) => {
                        if response.status().is_success() {
                            log("INFO", "Heartbeat sent successfully.");
                        } else {
                            log("ERROR", &format!("Received non-2xx status code: {}", response.status()));
                        }
                    }
                    Err(err) => {
                        log("ERROR", &format!("Failed to send heartbeat: {}", err));
                    }
                }
            }
            _ = signal::ctrl_c() => {
                log("INFO", "Received SIGINT, shutting down gracefully.");
                break;
            }
            _ = async {
                #[cfg(unix)]
                {
                    let mut term_signal = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
                    term_signal.recv().await;
                }
                #[cfg(not(unix))]
                {
                    // On non-unix platforms, we can't listen for SIGTERM in the same way.
                    // We'll just wait forever here. The ctrl_c handler will still work.
                    std::future::pending::<()>().await;
                }
            } => {
                log("INFO", "Received SIGTERM, shutting down gracefully.");
                break;
            }
        }
    }
}
