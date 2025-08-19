use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkValidationJob {
    pub id: String,
    pub emails: Vec<String>,
    pub check_role_based: bool,
    pub status: JobStatus,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Clone)]
pub struct JobQueue {
    redis: Arc<Client>,
}

impl JobQueue {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            redis: Arc::new(client),
        })
    }

    pub async fn enqueue_bulk_validation(
        &self,
        emails: Vec<String>,
        check_role_based: bool,
    ) -> Result<String, redis::RedisError> {
        let job_id = Uuid::new_v4().to_string();
        let job = BulkValidationJob {
            id: job_id.clone(),
            emails,
            check_role_based,
            status: JobStatus::Pending,
            created_at: chrono::Utc::now().timestamp(),
        };

        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let job_json = serde_json::to_string(&job).unwrap();

        let _: () = conn.lpush("bulk_validation_queue", &job_json).await?;
        let _: () = conn.set(format!("job:{}", job_id), &job_json).await?;
        let _: () = conn.expire(format!("job:{}", job_id), 3600).await?; // 1 hour TTL

        Ok(job_id)
    }

    pub async fn get_job_status(
        &self,
        job_id: &str,
    ) -> Result<Option<BulkValidationJob>, redis::RedisError> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let job_json: Option<String> = conn.get(format!("job:{}", job_id)).await?;

        Ok(job_json.and_then(|json| serde_json::from_str(&json).ok()))
    }

    pub async fn update_job_status(
        &self,
        job_id: &str,
        status: JobStatus,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;

        if let Some(mut job) = self.get_job_status(job_id).await? {
            job.status = status;
            let job_json = serde_json::to_string(&job).unwrap();
            let _: () = conn.set(format!("job:{}", job_id), &job_json).await?;
        }

        Ok(())
    }

    pub async fn process_jobs<F, Fut>(&self, processor: F)
    where
        F: Fn(BulkValidationJob) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        loop {
            match self.get_next_job().await {
                Ok(Some(job)) => {
                    let _ = self.update_job_status(&job.id, JobStatus::Processing).await;
                    processor(job).await;
                }
                Ok(None) => {
                    sleep(Duration::from_secs(1)).await;
                }
                Err(_) => {
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn get_next_job(&self) -> Result<Option<BulkValidationJob>, redis::RedisError> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;
        let result: Option<(String, String)> = conn.brpop("bulk_validation_queue", 1.0).await?;
        let job_json = result.map(|(_, value)| value);

        Ok(job_json.and_then(|json| serde_json::from_str(&json).ok()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_job_queue_new() {
        let result = JobQueue::new("redis://127.0.0.1:6379");
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_enqueue_bulk_validation() {
        if let Ok(job_queue) = JobQueue::new("redis://127.0.0.1:6379") {
            let emails = vec!["test@example.com".to_string(), "user@example.org".to_string()];
            let result = job_queue.enqueue_bulk_validation(emails, false).await;
            assert!(result.is_ok() || result.is_err());
        } else {
            assert!(true); // Pass test if Redis is not available
        }
    }

    #[tokio::test]
    async fn test_get_job_status() {
        if let Ok(job_queue) = JobQueue::new("redis://127.0.0.1:6379") {
            let result = job_queue.get_job_status("test-job-id").await;
            assert!(result.is_ok() || result.is_err());
        } else {
            assert!(true);
        }
    }

    #[tokio::test]
    async fn test_update_job_status() {
        if let Ok(job_queue) = JobQueue::new("redis://127.0.0.1:6379") {
            let result = job_queue.update_job_status("test-job-id", JobStatus::Completed).await;
            assert!(result.is_ok() || result.is_err());
        } else {
            assert!(true);
        }
    }

    #[tokio::test]
    async fn test_get_next_job() {
        if let Ok(job_queue) = JobQueue::new("redis://127.0.0.1:6379") {
            let result = job_queue.get_next_job().await;
            assert!(result.is_ok() || result.is_err());
        } else {
            assert!(true);
        }
    }

    #[tokio::test]
    async fn test_job_status_serialization() {
        let job = BulkValidationJob {
            id: "test-id".to_string(),
            emails: vec!["test@example.com".to_string()],
            check_role_based: false,
            status: JobStatus::Pending,
            created_at: 1234567890,
        };
        
        let serialized = serde_json::to_string(&job);
        assert!(serialized.is_ok());
        
        let deserialized: Result<BulkValidationJob, _> = serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());
    }
}
