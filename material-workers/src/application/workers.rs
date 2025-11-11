use crate::infrastructure::config::Config;
use crate::application::processors::MaterialJobProcessor;
use crate::infrastructure::message_queue::MessageQueue;
use tracing;

pub async fn start_workers(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting material workers...");

    // Initialize message queue
    let message_queue = MessageQueue::new(&config.message_queue).await?;

    // Initialize job processor
    let job_processor = MaterialJobProcessor::new(message_queue.clone()).await?;

    // Start consuming messages
    let mut handles = vec![];

    // Start material processing worker
    let handle = tokio::spawn(async move {
        if let Err(e) = job_processor.start().await {
            tracing::error!("Worker error: {}", e);
        }
    });
    handles.push(handle);

    // Wait for all workers
    for handle in handles {
        handle.await?;
    }

    Ok(())
}

