use std::net::{IpAddr, Ipv6Addr};
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
pub fn is_valid_email(email: &str) -> bool {
    // Check overall length constraint (RFC 5321 + 5322)
    if email.len() > 254 {
        return false;
    }

    // Find the @ separator, ignoring quoted @ symbols
    let mut in_quotes = false;
    let mut escape = false;
    let mut split_index = None;

    for (i, c) in email.char_indices() {
        match c {
            '"' if !escape => in_quotes = !in_quotes,
            '\\' if in_quotes => escape = true,
            '@' if !in_quotes => {
                split_index = Some(i);
                break;
            }
            _ => escape = false,
        }
    }

    let split_index = match split_index {
        Some(i) => i,
        None => return false, // No @ found
    };

    let (local_part, domain_part) = email.split_at(split_index);
    let domain_part = &domain_part[1..]; // Skip @

    // Validate local part length (RFC 5321)
    if local_part.len() > 64 {
        return false;
    }

    // Validate local part syntax
    if !is_valid_local_part(local_part) {
        return false;
    }

    // Validate domain part syntax
    is_valid_domain_part(domain_part)
}

/// Validates the local-part component of an email address
///
/// Supports both dot-atom (RFC 5322) and quoted-string (RFC 5322) formats
fn is_valid_local_part(local: &str) -> bool {
    if local.starts_with('"') && local.ends_with('"') {
        // Quoted string (RFC 5322 Section 3.4.1)
        is_valid_quoted_string(local)
    } else {
        // Dot-atom form (RFC 5322 Section 3.4.1)
        is_valid_dot_atom(local, false)
    }
}

/// Validates the domain part component of an email address
///
/// Handles both domain names and domain literals (IP addresses)
fn is_valid_domain_part(domain: &str) -> bool {
    if let Some(domain_literal) = domain.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        // Domain literal (RFC 5322 Section 3.4.1)
        is_valid_domain_literal(domain_literal)
    } else {
        // Regular domain (RFC 1035 + RFC 5890 + RFC 6531)
        is_valid_domain_name(domain)
    }
}

// Helper functions Below

/// Validates quoted-string format from RFC 5322 section 3.4.1
fn is_valid_quoted_string(quoted: &str) -> bool {
    let content = &quoted[1..quoted.len() - 1];
    let mut escape = false;

    for c in content.chars() {
        if escape {
            if !matches!(c, '\\' | '"') {
                return false;
            }
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == '"' {
            return false; // Unescaped quote
        }
    }
    !escape // Ensure no dangling escape
}

/// Validates dot-atom format from RFC 5322 section 3.4.1
///
/// * `is_domain` - Enforces stricter rules for domain validation
fn is_valid_dot_atom(s: &str, is_domain: bool) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.is_empty() || parts.iter().any(|&p| p.is_empty()) {
        return false;
    }

    parts.iter().all(|part| {
        part.chars().all(|c| match c {
            '-' => !is_domain || (!part.starts_with('-') && !part.ends_with('-')),
            c if is_domain => c.is_alphanumeric() || c == '-',
            _ => c.is_alphanumeric() || "!#$%&'*+/=?^_`{|}~".contains(c),
        })
    })
}

/// Validates domain literals (IP addresses) from RFC 5322 section 3.4.1
fn is_valid_domain_literal(literal: &str) -> bool {
    literal.parse::<IpAddr>().is_ok()
        || literal
            .strip_prefix("IPv6:")
            .and_then(|ip| ip.parse::<Ipv6Addr>().ok())
            .is_some()
}

/// Validates internationalized domain names per RFC 5890 and RFC 6531
fn is_valid_domain_name(domain: &str) -> bool {
    let labels: Vec<&str> = domain.split('.').collect();
    !labels.is_empty()
        && labels.iter().all(|label| {
            label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && is_valid_dot_atom(label, true)
        })
}

#[cfg(test)]
#[path = "../../tests/syntax_tests.rs"]

mod tests {
    use super::*;

    #[test]
    fn valid_standard_emails() {
        assert!(is_valid_email("simple@example.com"));
        assert!(is_valid_email("very.common@example.com"));
        assert!(is_valid_email("x@example.com")); // Short local
        assert!(is_valid_email("a.b@example.com"));
    }

    #[test]
    fn valid_special_chars() {
        assert!(is_valid_email("!#$%&'*+-/=?^_`{}|~@example.com"));
        assert!(is_valid_email("\"quoted@local\"@example.com"));
        assert!(is_valid_email("\"escaped\\\"quote\"@example.com"));
        assert!(is_valid_email("\"with space\"@example.com"));
    }

    #[test]
    fn valid_domain_literals() {
        assert!(is_valid_email("user@[192.168.0.1]"));
        assert!(is_valid_email("user@[IPv6:2001:db8::1]")); // Compressed format
        assert!(is_valid_email(
            "user@[IPv6:2001:0db8:85a3:0000:0000:ac1f:8001:1234]"
        )); // 8 hextets
    }

    #[test]
    fn valid_international() {
        assert!(is_valid_email("Pelé@exämple.中国"));
        assert!(is_valid_email("用户@例子.中国"));
        assert!(is_valid_email("ἀρχαῖα@δόκιμη.κπ"));
    }

    #[test]
    fn valid_edge_cases() {
        // Max length local part
        let max_local = "a".repeat(64);
        assert!(is_valid_email(&format!("{}@example.com", max_local)));

        // Max length email (254 chars) with valid domain labels
        let local = "a".repeat(64);
        let label = "b".repeat(63); // Max label length
        let domain = format!("{}.{}.{}", label, label, "c".repeat(61)); // 63 + 63 + 61 + 2 dots = 189
        assert_eq!(local.len() + 1 + domain.len(), 254);
        assert!(is_valid_email(&format!("{}@{}", local, domain)));
    }

    #[test]
    fn invalid_missing_at() {
        assert!(!is_valid_email("missing.example.com"));
        assert!(!is_valid_email("missing@"));
        assert!(!is_valid_email("@missing.com"));
    }

    #[test]
    fn invalid_lengths() {
        // Local part too long
        let long_local = "a".repeat(65);
        assert!(!is_valid_email(&format!("{}@example.com", long_local)));

        // Email too long
        let local = "a".repeat(64);
        let domain = "b".repeat(190); // 64 + 1 + 190 = 255
        assert!(!is_valid_email(&format!("{}@{}", local, domain)));
    }

    #[test]
    fn invalid_local_parts() {
        assert!(!is_valid_email("no..dots@example.com"));
        assert!(!is_valid_email(".leading@example.com"));
        assert!(!is_valid_email("trailing.@example.com"));
        assert!(!is_valid_email("un\"quoted@example.com"));
        assert!(!is_valid_email("\"unclosed@example.com"));
        assert!(!is_valid_email("spaces unquoted@example.com"));
    }

    #[test]
    fn invalid_domains() {
        assert!(!is_valid_email("user@-hyphenstart.com"));
        assert!(!is_valid_email("user@hyphenend-.com"));
        assert!(!is_valid_email("user@.leadingdot.com"));
        assert!(!is_valid_email("user@double..dot.com"));
        assert!(!is_valid_email("user@_invalidchar.com"));
    }

    #[test]
    fn invalid_domain_literals() {
        assert!(!is_valid_email("user@[invalid.ip]"));
        assert!(!is_valid_email("user@[IPv6:2001:db8:::1]"));
        assert!(!is_valid_email("user@[192.168.0.256]"));
        assert!(!is_valid_email("user@[missing.bracket"));
    }

    #[test]
    fn invalid_quoting() {
        assert!(!is_valid_email("\"invalid\\escape\"@example.com"));
        assert!(!is_valid_email("\"unbalanced@example.com"));
        assert!(!is_valid_email("quote\"in@middle.example.com"));
    }

    #[test]
    fn invalid_special_cases() {
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("   "));
        assert!(!is_valid_email("null@"));
        assert!(!is_valid_email("@"));
    }

    #[test]
    fn case_handling() {
        // Domain should be case-insensitive (valid regardless of case)
        assert!(is_valid_email("USER@EXAMPLE.COM"));
        assert!(is_valid_email("User@Example.com"));

        // Local part case sensitivity (preserved but still valid)
        assert!(is_valid_email("CaseSensitive@example.com"));
    }
}
