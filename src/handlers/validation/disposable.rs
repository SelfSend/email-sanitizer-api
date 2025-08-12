#[cfg(not(test))]
use mongodb::bson::{Document, doc};
#[cfg(not(test))]
use mongodb::{Client, Collection};
#[cfg(not(test))]
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
/// use email_sanitizer::handlers::validation::disposable::is_disposable_email;
/// let is_spam = is_disposable_email("example@mailinator.com").await?;
/// assert_eq!(is_spam, true);
/// # Ok(())
/// # }
/// ```
#[cfg(test)]
pub async fn is_disposable_email(email: &str) -> Result<bool, Box<dyn Error>> {
    is_disposable_email_mock(email).await
}

#[cfg(not(test))]
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

/// Mock implementation for testing without MongoDB
#[cfg(test)]
pub async fn is_disposable_email_mock(email: &str) -> Result<bool, Box<dyn Error>> {
    // Extract domain from email
    let (_, domain_part) = email
        .split_once('@')
        .ok_or("Invalid email format: missing '@'")?;
    let domain = domain_part.to_lowercase();

    // Mock disposable domains for testing
    let disposable_domains = vec![
        "mailinator.com",
        "0-00.usa.cc",
        "10minutemail.com",
        "guerrillamail.com",
        "tempmail.org",
    ];

    Ok(disposable_domains.contains(&domain.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    /// Tests recognition of disposable email domains using mock
    async fn test_disposable_email() {
        // Test disposable email using mock
        let result = is_disposable_email_mock("example@mailinator.com").await;
        assert!(result.unwrap(), "Should recognize disposable domain");
    }

    #[tokio::test]
    /// Tests recognition of valid email domains using mock
    async fn test_non_disposable_email() {
        // Test valid email using mock
        let result = is_disposable_email_mock("johndoe@gmail.com").await;
        assert!(!result.unwrap(), "Should recognize non-disposable domain");
    }

    #[tokio::test]
    /// Test invalid email format
    async fn test_invalid_email_format() {
        let result = is_disposable_email_mock("invalid-email").await;
        assert!(
            result.is_err(),
            "Should return error for invalid email format"
        );
    }
}
