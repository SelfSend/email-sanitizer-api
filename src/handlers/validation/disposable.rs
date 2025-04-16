use mongodb::bson::{Document, doc};
use mongodb::{Client, Collection};
use std::env;
use std::error::Error;

/// Checks if an email address uses a disposable domain by querying a MongoDB collection.
///
/// # Arguments
/// * `email` - A string slice containing the email address to check
///
/// # Returns
/// * `Ok(true)` if the domain is found in the disposable email collection
/// * `Ok(false)` if the domain is not found
/// * `Err` containing an error message if any step fails
///
/// # Errors
/// Returns an error if:
/// - The email is missing '@' symbol (invalid format)
/// - Environment variables are not properly configured
/// - MongoDB connection or query fails
///
/// # Example
/// ```
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use email_sanitizer::handlers::validation::disposable::is_disposable_email; // Add the correct use statement
/// let is_spam = is_disposable_email("example@0-00.usa.cc").await?;
/// assert_eq!(is_spam, true);
/// # Ok(())
/// # }
/// ```
pub async fn is_disposable_email(email: &str) -> Result<bool, Box<dyn Error>> {
    // Extract domain from email
    let (_, domain_part) = email
        .split_once('@')
        .ok_or("Invalid email format: missing '@'")?;
    let domain = domain_part.to_lowercase();

    // Retrieve environment variables
    let mongo_uri = env::var("MONGODB_URI")?;
    let db_name = env::var("DB_NAME_PRODUCTION")?;
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
mod tests {
    use super::*;
    use mongodb::bson::{Document, doc};
    use std::env;

    /// Helper function to set up test MongoDB collection
    async fn setup_collection() -> Collection<Document> {
        // Load environment variables from .env file
        dotenv::dotenv().ok();

        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let db_name = env::var("DB_NAME_TEST").expect("DB_NAME_TEST must be set");
        let collection_name = env::var("DB_DISPOSABLE_EMAILS_COLLECTION")
            .expect("DB_DISPOSABLE_EMAILS_COLLECTION must be set");

        let client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Failed to connect to MongoDB");
        client.database(&db_name).collection(&collection_name)
    }

    #[tokio::test]
    /// Tests recognition of disposable email domains
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
    /// Tests recognition of valid email domains
    async fn test_non_disposable_email() {
        let collection = setup_collection().await;

        // Ensure test domain is removed
        collection
            .delete_many(doc! { "domain": "gmail.com" })
            .await
            .expect("Failed to clean up test data");

        // Test valid email
        let result = is_disposable_email("johndoe@gmail.com").await;
        assert!(!result.unwrap(), "Should recognize non-disposable domain");
    }
}
