use kinetics::macros::worker;
use kinetics::tools::config::Config as KineticsConfig;
use kinetics::tools::queue::{Record as QueueRecord, Retries as QueueRetries};
use std::collections::HashMap;
use tower::BoxError;

#[worker(concurrency = 3, fifo = true)]
pub async fn worker(
    _records: Vec<QueueRecord>,
    _secrets: &HashMap<String, String>,
    _config: &KineticsConfig,
) -> Result<QueueRetries, BoxError> {
    let retries = QueueRetries::new();
    Ok(retries)
}
