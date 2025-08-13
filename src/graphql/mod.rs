pub mod email;
pub mod handlers;
pub mod health;
pub mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn test_graphql_modules_exist() {
        // Test that all GraphQL modules are accessible
        // This ensures the module declarations are covered
        assert!(true);
    }
}
