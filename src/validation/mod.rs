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
pub mod dnsmx;

/// Validates an email address according to RFC 5322 and RFC 6531 specifications.
///
/// This function performs syntax checking of both local-part and domain parts with:
/// - Full quoted-string/local-part support
/// - Domain literal (IP address) validation
/// - Internationalized email (UTF-8) support
/// - Length constraints enforcement
///
/// # Examples
/// ```
/// use email_sanitizer::validation::syntax::is_valid_email;
///
/// assert!(is_valid_email("user.name+tag@example.com"));
/// assert!(is_valid_email("Pelé@exämple.中国"));
/// assert!(!is_valid_email("invalid@ex_mple.com"));
/// ```
///
/// # Arguments
/// * `email` - A string slice containing the email address to validate
///
/// # Returns
/// `true` if the email address meets all syntax requirements, `false` otherwise
pub mod syntax;
