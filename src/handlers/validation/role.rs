use mongodb::bson::{Document, doc};
use mongodb::{Client, Collection};
use std::env;
use std::error::Error;

/// Checks if an email address uses a role-based alias by querying a MongoDB collection.
///
/// # Arguments
/// * `email` - A string slice containing the email address to check
///
/// # Returns
/// * `Ok(true)` if the alias is found in the role-based alias collection
/// * `Ok(false)` if the alias is not found
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
/// use email_sanitizer::handlers::validation::role_based::is_role_based_alias;
/// let is_role_alias = is_role_based_alias("admin@example.com").await?;
/// assert_eq!(is_role_alias, true);
/// # Ok(())
/// # }
/// ```
pub async fn is_role_based_alias(email: &str) -> Result<bool, Box<dyn Error>> {
    // Extract alias from email
    let (alias_part, _) = email
        .split_once('@')
        .ok_or("Invalid email format: missing '@'")?;
    let alias = alias_part.to_lowercase();

    // Retrieve environment variables
    let mongo_uri = env::var("MONGODB_URI")?;
    let db_name = env::var("DB_NAME_PRODUCTION")?;
    let collection_name = env::var("DB_ROLE_BASED_ALIAS_COLLECTION")?;

    // Connect to MongoDB
    let client = Client::with_uri_str(&mongo_uri).await?;
    let database = client.database(&db_name);
    let collection: Collection<Document> = database.collection(&collection_name);

    // Check if alias exists in the collection
    let filter = doc! { "alias": alias };
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
        let collection_name = env::var("DB_ROLE_BASED_ALIAS_COLLECTION")
            .expect("DB_ROLE_BASED_ALIAS_COLLECTION must be set");

        let client = Client::with_uri_str(&mongo_uri)
            .await
            .expect("Failed to connect to MongoDB");
        client.database(&db_name).collection(&collection_name)
    }

    #[tokio::test]
    /// Tests recognition of role-based email aliases
    async fn test_role_based_alias() {
        let collection = setup_collection().await;

        // Insert test role-based alias
        collection
            .insert_one(doc! { "alias": "admin" })
            .await
            .expect("Failed to insert test data");

        // Test role-based alias
        let result = is_role_based_alias("admin@example.com").await;
        assert!(result.unwrap(), "Should recognize role-based alias");

        // Cleanup
        collection
            .delete_many(doc! { "alias": "admin" })
            .await
            .expect("Failed to clean up test data");
    }

    #[tokio::test]
    /// Tests recognition of non-role-based email aliases
    async fn test_non_role_based_alias() {
        let collection = setup_collection().await;

        // Ensure test alias is removed
        collection
            .delete_many(doc! { "alias": "johndoe" })
            .await
            .expect("Failed to clean up test data");

        // Test valid email
        let result = is_role_based_alias("johndoe@gmail.com").await;
        assert!(!result.unwrap(), "Should recognize non-role-based alias");
    }
}
