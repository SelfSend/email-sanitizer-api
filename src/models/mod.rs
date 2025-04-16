/// # Health Status Response
///
/// Represents the operational status of the service with a timestamp.
/// Used as the response format for health check endpoints.
///
/// ## Fields
/// - `status`: String indicating service availability ("UP" or "DOWN")
/// - `timestamp`: ISO 8601 formatted timestamp of the status check
///
/// ## Serialization
/// Automatically implements `Serialize` and `Deserialize` for JSON format.
///
/// ## Example JSON
/// ```json
/// {
///   "status": "UP",
///   "timestamp": "2024-03-10T15:30:45.123456789Z"
/// }
/// ```
pub mod health;
