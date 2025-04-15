use mongodb::bson::{Document, doc};
use mongodb::{Client, Collection};
use std::env;
use std::error::Error;

pub async fn is_disposable_email(email: &str) -> Result<bool, Box<dyn Error>> {
    // Extract domain from email
    let (_, domain_part) = email
        .split_once('@')
        .ok_or("Invalid email format: missing '@'")?;
    let domain = domain_part.to_lowercase();

    // Retrieve environment variables
    let mongo_uri = env::var("MONGODB_URI")?;
    let db_name = env::var("DB_NAME")?;
    let collection_name = env::var("DB_DISPOSABLE_EMAILS_COLLECTION")?;

    // Connect to MongoDB
    let client = Client::with_uri_str(&mongo_uri).await?;
    let database = client.database(&db_name);
    let collection: Collection<Document> = database.collection(&collection_name);

    // Check if domain exists in the collection
    let filter = doc! { "domain": domain };
    let exists = collection.find_one(filter).await?.is_some();

    Ok(exists)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::{Document, doc};
    use std::env;

    async fn setup_collection() -> Collection<Document> {
        // Load environment variables from .env file
        dotenv::dotenv().ok();

        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
        let collection_name = env::var("DB_DISPOSABLE_EMAILS_COLLECTION")
            .expect("DB_DISPOSABLE_EMAILS_COLLECTION must be set");

        let client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Failed to connect to MongoDB");
        client.database(&db_name).collection(&collection_name)
    }

    #[tokio::test]
    async fn test_disposable_email() {
        let collection = setup_collection().await;

        // Insert test disposable domain
        collection
            .insert_one(doc! { "domain": "0-00.usa.cc" })
            .await
            .expect("Failed to insert test data");

        // Test disposable email
        let result = is_disposable_email("example@0-00.usa.cc").await;
        assert!(result.unwrap(), "Should recognize disposable domain");

        // Cleanup
        collection
            .delete_many(doc! { "domain": "0-00.usa.cc" })
            .await
            .expect("Failed to clean up test data");
    }

    #[tokio::test]
    async fn test_non_disposable_email() {
        let collection = setup_collection().await;

        collection
            .delete_many(doc! { "domain": "gmail.com" })
            .await
            .expect("Failed to clean up test data");

        // Test valid email
        let result = is_disposable_email("johndoe@gmail.com").await;
        assert!(!result.unwrap(), "Should recognize non-disposable domain");
    }
}
