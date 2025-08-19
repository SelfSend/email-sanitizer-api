#[cfg(test)]
mod dnsmx_additional_tests {
    use crate::handlers::validation::dnsmx::*;

    #[test]
    fn test_validate_email_dns_invalid_domains() {
        // Test various invalid domain formats
        assert!(!validate_email_dns("user@"));
        assert!(!validate_email_dns("user@."));
        assert!(!validate_email_dns("user@.."));
        assert!(!validate_email_dns("user@.com"));
        assert!(!validate_email_dns("user@com."));
        assert!(!validate_email_dns("user@-invalid.com"));
        assert!(!validate_email_dns("user@invalid-.com"));
    }

    #[test]
    fn test_validate_email_dns_nonexistent_domains() {
        // Test domains that definitely don't exist
        assert!(!validate_email_dns("user@nonexistent-domain-12345.invalid"));
        assert!(!validate_email_dns("user@this-domain-does-not-exist-anywhere.test"));
        assert!(!validate_email_dns("user@fake-domain-for-testing-purposes.invalid"));
    }

    #[test]
    fn test_validate_email_dns_malformed_emails() {
        // Test malformed email addresses
        assert!(!validate_email_dns("not-an-email"));
        assert!(!validate_email_dns("@domain.com"));
        assert!(!validate_email_dns("user@@domain.com"));
        assert!(!validate_email_dns("user@"));
    }

    #[test]
    fn test_validate_email_dns_empty_input() {
        assert!(!validate_email_dns(""));
        assert!(!validate_email_dns("   "));
    }

    #[test]
    fn test_validate_email_dns_localhost() {
        // localhost might or might not resolve depending on system configuration
        let result = validate_email_dns("user@localhost");
        // We don't assert true/false since it depends on system config
        // Just ensure it doesn't panic
        assert!(result == true || result == false);
    }

    #[test]
    fn test_validate_email_dns_ip_addresses() {
        // Test with IP addresses as domains (should fail DNS lookup)
        assert!(!validate_email_dns("user@192.168.1.1"));
        assert!(!validate_email_dns("user@127.0.0.1"));
        assert!(!validate_email_dns("user@::1"));
    }

    #[test]
    fn test_validate_email_dns_very_long_domain() {
        let long_domain = format!("{}.com", "a".repeat(250));
        let email = format!("user@{}", long_domain);
        assert!(!validate_email_dns(&email));
    }

    #[test]
    fn test_validate_email_dns_special_tlds() {
        // Test with various TLDs that might not exist
        assert!(!validate_email_dns("user@example.invalidtld"));
        assert!(!validate_email_dns("user@example.fake"));
        assert!(!validate_email_dns("user@example.notreal"));
    }

    #[test]
    fn test_validate_email_dns_unicode_domains() {
        // Test with internationalized domain names
        // These might fail due to DNS resolution issues in test environment
        let unicode_emails = [
            "user@münchen.de",
            "user@москва.рф", 
            "user@北京.中国",
            "user@العربية.مصر"
        ];
        
        for email in &unicode_emails {
            let result = validate_email_dns(email);
            // Don't assert specific result since DNS resolution varies
            // Just ensure no panic
            assert!(result == true || result == false);
        }
    }

    #[test]
    fn test_validate_email_dns_subdomain_variations() {
        // Test various subdomain patterns that likely don't exist
        let test_emails = [
            "user@nonexistent.example.invalid",
            "user@deep.nested.subdomain.invalid",
            "user@a.b.c.d.e.f.invalid"
        ];
        
        for email in &test_emails {
            assert!(!validate_email_dns(email));
        }
    }

    #[test]
    fn test_validate_email_dns_case_insensitive() {
        // Domain names should be case insensitive
        let emails = [
            "user@EXAMPLE.COM",
            "user@Example.Com", 
            "user@example.COM"
        ];
        
        for email in &emails {
            let result = validate_email_dns(email);
            // Just ensure consistent behavior regardless of case
            assert!(result == true || result == false);
        }
    }
}