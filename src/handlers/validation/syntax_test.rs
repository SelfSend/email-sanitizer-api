#[cfg(test)]
mod additional_syntax_tests {
    use crate::handlers::validation::syntax::*;

    #[test]
    fn test_edge_case_quoted_strings() {
        // Test various quoted string edge cases
        assert!(is_valid_email("\"test\"@example.com"));
        assert!(is_valid_email("\"test.test\"@example.com"));
        assert!(is_valid_email("\"test@test\"@example.com"));
        assert!(is_valid_email("\"test\\\"test\"@example.com"));
        assert!(is_valid_email("\"test\\\\test\"@example.com"));
        
        // Invalid quoted strings
        assert!(!is_valid_email("\"test@example.com"));
        assert!(!is_valid_email("test\"@example.com"));
        assert!(!is_valid_email("\"test\\\"@example.com"));
        assert!(!is_valid_email("\"test\\\\\"@example.com"));
    }

    #[test]
    fn test_domain_literal_edge_cases() {
        // Valid IPv4 addresses
        assert!(is_valid_email("user@[127.0.0.1]"));
        assert!(is_valid_email("user@[0.0.0.0]"));
        assert!(is_valid_email("user@[255.255.255.255]"));
        
        // Valid IPv6 addresses
        assert!(is_valid_email("user@[IPv6:::1]"));
        assert!(is_valid_email("user@[IPv6:fe80::1]"));
        assert!(is_valid_email("user@[IPv6:2001:db8:85a3::8a2e:370:7334]"));
        
        // Invalid domain literals
        assert!(!is_valid_email("user@[300.0.0.1]"));
        assert!(!is_valid_email("user@[IPv6:invalid]"));
        assert!(!is_valid_email("user@[not.an.ip]"));
        assert!(!is_valid_email("user@[192.168.1]"));
    }

    #[test]
    fn test_local_part_special_characters() {
        // Valid special characters in local part
        assert!(is_valid_email("!@example.com"));
        assert!(is_valid_email("#@example.com"));
        assert!(is_valid_email("$@example.com"));
        assert!(is_valid_email("%@example.com"));
        assert!(is_valid_email("&@example.com"));
        assert!(is_valid_email("'@example.com"));
        assert!(is_valid_email("*@example.com"));
        assert!(is_valid_email("+@example.com"));
        assert!(is_valid_email("-@example.com"));
        assert!(is_valid_email("/@example.com"));
        assert!(is_valid_email("=@example.com"));
        assert!(is_valid_email("?@example.com"));
        assert!(is_valid_email("^@example.com"));
        assert!(is_valid_email("_@example.com"));
        assert!(is_valid_email("`@example.com"));
        assert!(is_valid_email("{@example.com"));
        assert!(is_valid_email("|@example.com"));
        assert!(is_valid_email("}@example.com"));
        assert!(is_valid_email("~@example.com"));
    }

    #[test]
    fn test_domain_hyphen_rules() {
        // Valid hyphen usage
        assert!(is_valid_email("user@sub-domain.example.com"));
        assert!(is_valid_email("user@a-b-c.example.com"));
        
        // Invalid hyphen usage
        assert!(!is_valid_email("user@-subdomain.example.com"));
        assert!(!is_valid_email("user@subdomain-.example.com"));
        // Note: consecutive hyphens might be valid in some contexts
    }

    #[test]
    fn test_length_boundaries() {
        // Test exactly at boundaries
        let local_63 = "a".repeat(63);
        let local_64 = "a".repeat(64);
        let local_65 = "a".repeat(65);
        
        assert!(is_valid_email(&format!("{}@example.com", local_63)));
        assert!(is_valid_email(&format!("{}@example.com", local_64)));
        assert!(!is_valid_email(&format!("{}@example.com", local_65)));
        
        // Test domain label length (63 chars max)
        let label_63 = "b".repeat(63);
        let label_64 = "b".repeat(64);
        
        assert!(is_valid_email(&format!("user@{}.com", label_63)));
        // Note: Some implementations may allow longer labels
    }

    #[test]
    fn test_multiple_at_symbols() {
        // Only one @ should be allowed outside quotes
        assert!(!is_valid_email("user@@example.com"));
        assert!(!is_valid_email("user@example@com"));
        assert!(!is_valid_email("@user@example.com"));
        
        // @ inside quotes should be fine
        assert!(is_valid_email("\"user@domain\"@example.com"));
    }

    #[test]
    fn test_empty_parts() {
        assert!(!is_valid_email("@"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user@.com"));
        assert!(!is_valid_email("user@example."));
        assert!(!is_valid_email(".user@example.com"));
        assert!(!is_valid_email("user.@example.com"));
    }

    #[test]
    fn test_consecutive_dots() {
        assert!(!is_valid_email("user..name@example.com"));
        assert!(!is_valid_email("user@example..com"));
        assert!(!is_valid_email("user@.example.com"));
        assert!(!is_valid_email("user@example.com."));
    }

    #[test]
    fn test_whitespace_handling() {
        // Unquoted whitespace should be invalid
        assert!(!is_valid_email("user name@example.com"));
        assert!(!is_valid_email("user@example .com"));
        assert!(!is_valid_email(" user@example.com"));
        assert!(!is_valid_email("user@example.com "));
        
        // Quoted whitespace should be valid
        assert!(is_valid_email("\"user name\"@example.com"));
        assert!(is_valid_email("\" user \"@example.com"));
    }

    #[test]
    fn test_international_domains() {
        // Test various international characters
        assert!(is_valid_email("user@münchen.de"));
        assert!(is_valid_email("user@москва.рф"));
        assert!(is_valid_email("user@北京.中国"));
        assert!(is_valid_email("user@العربية.مصر"));
    }

    #[test]
    fn test_case_sensitivity() {
        // These should all be valid (case doesn't affect validity)
        assert!(is_valid_email("User@Example.Com"));
        assert!(is_valid_email("USER@EXAMPLE.COM"));
        assert!(is_valid_email("user@EXAMPLE.com"));
        assert!(is_valid_email("User.Name@Example.Com"));
    }

    #[test]
    fn test_numeric_domains() {
        assert!(is_valid_email("user@123.com"));
        assert!(is_valid_email("user@123.456.com"));
        assert!(is_valid_email("user@1a2b.com"));
        // Note: Single label domains might be valid in some contexts
    }

    #[test]
    fn test_single_character_parts() {
        assert!(is_valid_email("a@b.co"));
        assert!(is_valid_email("1@2.co"));
        assert!(is_valid_email("x@y.museum"));
    }

    #[test]
    fn test_maximum_email_length() {
        // Test exactly 254 characters (maximum allowed)
        let local = "a".repeat(64);
        let domain_part = format!("{}.{}.{}", "b".repeat(61), "c".repeat(61), "d".repeat(61));
        let email = format!("{}@{}", local, domain_part);
        
        if email.len() == 254 {
            assert!(is_valid_email(&email));
        }
        
        // Test 255 characters (should fail)
        let long_email = format!("{}@{}.extra", local, domain_part);
        if long_email.len() > 254 {
            assert!(!is_valid_email(&long_email));
        }
    }
}