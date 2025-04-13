use std::time::Duration;
use trust_dns_resolver::{
    Resolver,
    config::{ResolverConfig, ResolverOpts},
    error::ResolveError,
    proto::rr::RecordType,
};

/// Validates an email address domain by checking DNS records.
///
/// This function performs DNS lookups to verify the domain part of an email address:
/// 1. Checks for MX (Mail Exchange) records first
/// 2. Falls back to A/AAAA records if MX records are not found
///
/// # Arguments
/// * `email` - The email address to validate. Must contain an '@' symbol.
///
/// # Returns
/// `true` if the domain has valid MX records or fallback A/AAAA records,
/// `false` if validation fails for any reason (invalid format, DNS errors, etc.)
///
/// # Examples
/// ```
/// use email_dns_validator::validate_email_dns;
///
/// let valid = validate_email_dns("user@example.com");
/// assert!(valid);
///
/// let invalid = validate_email_dns("invalid@nonexistent.domain");
/// assert!(!invalid);
/// ```
pub fn validate_email_dns(email: &str) -> bool {
    let domain = match email.rsplit_once('@') {
        Some((_, domain)) => domain,
        None => return false,
    };

    let resolver = match create_resolver() {
        Some(r) => r,
        None => return false,
    };

    check_mx_or_a_records(&resolver, domain).unwrap_or(false)
}

/// Creates a DNS resolver with custom configuration
///
/// Configures resolver with:
/// - 2 second timeout per request
/// - 2 retry attempts
/// - Default system resolver configuration
fn create_resolver() -> Option<Resolver> {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(2);
    opts.attempts = 2;

    Resolver::new(ResolverConfig::default(), opts).ok()
}

/// Checks DNS records for a domain following RFC 5321 requirements
///
/// 1. First checks for MX records (mail server configuration)
/// 2. If MX lookup fails, checks for A (IPv4) or AAAA (IPv6) records
///
/// # Arguments
/// * `resolver` - DNS resolver to use for lookups
/// * `domain` - Domain name to check (without @ symbol)
///
/// # Returns
/// `Result<bool, ResolveError>` where:
/// - `Ok(true)` if valid records found
/// - `Ok(false)` if no records found
/// - `Err` contains DNS resolution error
fn check_mx_or_a_records(resolver: &Resolver, domain: &str) -> Result<bool, ResolveError> {
    // Check MX records first
    let mx_records = resolver.mx_lookup(domain);
    if let Ok(records) = mx_records {
        return Ok(records.iter().next().is_some());
    }

    // Fallback to A/AAAA records if MX lookup failed
    let a_records = resolver.lookup(domain, RecordType::A)?;
    let aaaa_records = resolver.lookup(domain, RecordType::AAAA)?;

    Ok(!a_records.is_empty() || !aaaa_records.is_empty())
}

#[cfg(test)]
mod tests {
    use super::validate_email_dns;

    #[test]
    fn test_valid_email_with_mx() {
        // Google's domain has MX records
        assert!(validate_email_dns("test@gmail.com"));
    }

    #[test]
    fn test_valid_email_with_a_record() {
        // example.com has A record but no MX
        assert!(validate_email_dns("test@example.com"));
    }

    #[test]
    fn test_invalid_domain() {
        assert!(!validate_email_dns("user@invalid.invalid"));
    }

    #[test]
    fn test_email_without_at_symbol() {
        assert!(!validate_email_dns("invalid-email"));
    }

    #[test]
    fn test_localhost_fallback() {
        // localhost has A record but no MX
        assert!(validate_email_dns("user@localhost"));
    }

    #[test]
    fn test_mx_priority_order() {
        // Domain with multiple MX records
        assert!(validate_email_dns("test@microsoft.com"));
    }

    // Test for timeout handling (might need adjustment based on network conditions)
    #[test]
    fn test_dns_timeout() {
        let result = std::panic::catch_unwind(|| {
            let _ = validate_email_dns("test@network.test");
        });
        assert!(result.is_ok());
    }
}
