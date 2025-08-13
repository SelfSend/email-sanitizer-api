use mongodb::{Client, Collection, bson::doc};
use std::env;

/// Checks if an email address uses a role-based local part by querying a MongoDB collection.
///
/// # Arguments
/// * `email` - A string slice containing the email address to check
///
/// # Returns
/// * `Ok(true)` if the local part is found in the role-based collection
/// * `Ok(false)` if the local part is not found
/// * `Err` containing an error message if any step fails
///
/// # Examples
/// ```
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use email_sanitizer::handlers::validation::role_based::is_role_based_email;
/// let is_role = is_role_based_email("admin@example.com").await?;
/// assert_eq!(is_role, true);
/// # Ok(())
/// # }
/// ```
pub async fn is_role_based_email(email: &str) -> Result<bool, String> {
    let at_pos = email.find('@').ok_or("Invalid email format")?;
    let local_part = email[..at_pos].to_lowercase();

    let mongo_uri =
        env::var("MONGODB_URI").map_err(|_| "MONGODB_URI environment variable not set")?;
    let database_name = env::var("DB_NAME_PRODUCTION")
        .map_err(|_| "DB_NAME_PRODUCTION environment variable not set")?;

    let client = Client::with_uri_str(&mongo_uri)
        .await
        .map_err(|e| format!("Failed to connect to MongoDB: {}", e))?;
    let db = client.database(&database_name);
    let collection: Collection<mongodb::bson::Document> = db.collection("role_based_emails");

    let filter = doc! { "prefix": &local_part };
    match collection.find_one(filter).await {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(format!("Database query failed: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_email_format() {
        assert!(is_role_based_email("invalid-email").await.is_err());
        assert!(is_role_based_email("@example.com").await.is_err());
    }

    #[tokio::test]
    async fn test_empty_email() {
        assert!(is_role_based_email("").await.is_err());
    }
}
