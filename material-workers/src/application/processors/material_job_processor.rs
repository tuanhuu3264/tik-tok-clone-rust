use crate::domain::entities::material_job::{MaterialJob, MaterialJobType, JobStatus};
use crate::domain::repositories::JobRepository;
use crate::domain::services::MaterialProcessor;
use crate::domain::repositories::MaterialRepository;
use crate::infrastructure::message_queue::MessageQueue;
use crate::application::errors::ApplicationError;
use std::sync::Arc;
use tracing;

pub struct MaterialJobProcessor<JR: JobRepository, MR: MaterialRepository> {
    job_repository: Arc<JR>,
    material_repository: Arc<MR>,
    material_processor: MaterialProcessor<MR>,
    message_queue: MessageQueue,
}

impl<JR: JobRepository, MR: MaterialRepository> MaterialJobProcessor<JR, MR> {
    pub async fn new(message_queue: MessageQueue) -> Result<Self, ApplicationError> {
        // TODO: Initialize repositories from infrastructure
        // For now, this is a placeholder
        Err(ApplicationError::InitializationError("Repositories not initialized".to_string()))
    }

    pub async fn start(&self) -> Result<(), ApplicationError> {
        tracing::info!("Starting material job processor...");

        // Consume messages from queue
        loop {
            match self.message_queue.consume("material_jobs").await {
                Ok(Some(message)) => {
                    if let Err(e) = self.process_job(message).await {
                        tracing::error!("Failed to process job: {}", e);
                    }
                }
                Ok(None) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
                Err(e) => {
                    tracing::error!("Error consuming message: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn process_job(&self, message: serde_json::Value) -> Result<(), ApplicationError> {
        let job: MaterialJob = serde_json::from_value(message)
            .map_err(|e| ApplicationError::ProcessingError(format!("Invalid job format: {}", e)))?;

        let mut job = self.job_repository.find_by_id(&job.id).await?
            .ok_or(ApplicationError::JobNotFound)?;

        job.mark_processing();
        self.job_repository.update(&job).await?;

        let result = match job.job_type {
            MaterialJobType::ProcessMaterial => {
                self.process_material_job(&mut job).await
            }
            MaterialJobType::UpdateInventory => {
                self.update_inventory_job(&mut job).await
            }
            MaterialJobType::SyncSupplier => {
                self.sync_supplier_job(&mut job).await
            }
        };

        match result {
            Ok(_) => {
                job.mark_completed();
                self.job_repository.update(&job).await?;
                tracing::info!("Job {} completed successfully", job.id.as_uuid());
            }
            Err(e) => {
                if job.can_retry(3) {
                    job.mark_failed();
                    self.job_repository.update(&job).await?;
                    tracing::warn!("Job {} failed, will retry: {}", job.id.as_uuid(), e);
                } else {
                    job.mark_failed();
                    self.job_repository.update(&job).await?;
                    tracing::error!("Job {} failed after max retries: {}", job.id.as_uuid(), e);
                }
            }
        }

        Ok(())
    }

    async fn process_material_job(&self, job: &mut MaterialJob) -> Result<(), ApplicationError> {
        // TODO: Implement material processing logic
        tracing::info!("Processing material job: {:?}", job.id);
        Ok(())
    }

    async fn update_inventory_job(&self, job: &mut MaterialJob) -> Result<(), ApplicationError> {
        // TODO: Implement inventory update logic
        tracing::info!("Updating inventory job: {:?}", job.id);
        Ok(())
    }

    async fn sync_supplier_job(&self, job: &mut MaterialJob) -> Result<(), ApplicationError> {
        // TODO: Implement supplier sync logic
        tracing::info!("Syncing supplier job: {:?}", job.id);
        Ok(())
    }
}

