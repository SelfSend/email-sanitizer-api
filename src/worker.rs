use crate::job_queue::{BulkValidationJob, JobQueue, JobStatus};
use crate::routes::email::{RedisCache, validate_single_email};
use futures::future::join_all;

pub struct ValidationWorker {
    job_queue: JobQueue,
    redis_cache: RedisCache,
}

impl ValidationWorker {
    pub fn new(job_queue: JobQueue, redis_cache: RedisCache) -> Self {
        Self {
            job_queue,
            redis_cache,
        }
    }

    pub async fn start(&self) {
        let job_queue = self.job_queue.clone();
        let redis_cache = self.redis_cache.clone();

        job_queue
            .clone()
            .process_jobs(move |job| {
                let redis_cache = redis_cache.clone();
                let job_queue = job_queue.clone();
                async move {
                    Self::process_bulk_validation(job, redis_cache, job_queue).await;
                }
            })
            .await;
    }

    async fn process_bulk_validation(
        job: BulkValidationJob,
        redis_cache: RedisCache,
        job_queue: JobQueue,
    ) {
        let validation_futures =
            job.emails
                .iter()
                .map(|email| {
                    let email_clone = email.clone();
                    let redis_cache = redis_cache.clone();
                    let check_role_based = job.check_role_based;
                    async move {
                        validate_single_email(&email_clone, check_role_based, &redis_cache).await
                    }
                })
                .collect::<Vec<_>>();

        let _results = join_all(validation_futures).await;

        // Mark job as completed
        let _ = job_queue
            .update_job_status(&job.id, JobStatus::Completed)
            .await;
    }
}
