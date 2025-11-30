#[cfg(test)]
mod tests {

    #[cfg(feature = "mail")]
    use emixnet::web::mail::*;

    #[test]
    #[cfg(feature = "mail")]
    fn test_tempmail_provider_enum() {
        // Test TempMailProvider enum variants
        let tempmail = TempMailProvider::Tempmail;
        assert_eq!(tempmail, TempMailProvider::default());

        let email_fake = TempMailProvider::EmailFake;
        assert_eq!(email_fake, TempMailProvider::EmailFake);

        let secmail = TempMailProvider::SecMail(SecMailDomain::SecMailCom);
        match secmail {
            TempMailProvider::SecMail(SecMailDomain::SecMailCom) => {}
            _ => panic!("Should be SecMail(SecMailCom)"),
        }
    }

    #[test]
    #[cfg(feature = "mail")]
    fn test_secmail_domain_display() {
        // Test SecMailDomain Display trait
        let domain = SecMailDomain::SecMailCom;
        let domain_str = format!("{}", domain);
        assert_eq!(domain_str, "1secmail.com");

        let domain = SecMailDomain::SecMailOrg;
        let domain_str = format!("{}", domain);
        assert_eq!(domain_str, "1secmail.org");

        let domain = SecMailDomain::SecMailNet;
        let domain_str = format!("{}", domain);
        assert_eq!(domain_str, "1secmail.net");
    }

    #[test]
    #[cfg(feature = "mail")]
    fn test_tempmail_new() {
        // Test TempMail::new()
        let email = TempMail::new(TempMailProvider::Tempmail, "testuser", "tempmail.io");
        assert_eq!(email.username(), "testuser");
        assert_eq!(email.domain(), "tempmail.io");
        assert_eq!(email.address(), "testuser@tempmail.io");
    }

    #[test]
    #[cfg(feature = "mail")]
    fn test_tempmail_parse() {
        // Test TempMail::parse()
        let email = TempMail::parse(TempMailProvider::Tempmail, "testuser@tempmail.io");
        assert_eq!(email.username(), "testuser");
        assert_eq!(email.domain(), "tempmail.io");
        assert_eq!(email.address(), "testuser@tempmail.io");
    }

    #[test]
    #[cfg(feature = "mail")]
    fn test_tempmail_parse_various_formats() {
        // Test parsing various email formats
        let test_cases = vec![
            ("user@domain.com", TempMailProvider::Tempmail),
            ("test.email@example.org", TempMailProvider::EmailFake),
            (
                "a@b.net",
                TempMailProvider::SecMail(SecMailDomain::SecMailCom),
            ),
        ];

        for (email_str, provider) in test_cases {
            let email = TempMail::parse(provider, email_str);
            assert_eq!(email.address(), email_str);

            let parts: Vec<&str> = email_str.split('@').collect();
            assert_eq!(email.username(), parts[0]);
            assert_eq!(email.domain(), parts[1]);
        }
    }

    #[test]
    #[cfg(feature = "mail")]
    fn test_tempmail_from() {
        // Test TempMail::from()
        let original = TempMail::new(TempMailProvider::EmailFake, "original", "test.com");
        let cloned = TempMail::from(&original);

        assert_eq!(cloned.username(), original.username());
        assert_eq!(cloned.domain(), original.domain());
        assert_eq!(cloned.address(), original.address());
    }

    #[test]
    #[cfg(feature = "mail")]
    fn test_tempmail_address_formatting() {
        // Test address() method formatting
        let email = TempMail::new(TempMailProvider::Tempmail, "user", "domain.com");
        let address = email.address();
        assert!(address.contains('@'));
        assert!(address.starts_with("user"));
        assert!(address.ends_with("domain.com"));
    }

    #[tokio::test]
    #[cfg(feature = "mail")]
    async fn test_tempmail_generate_tempmail() -> Result<()> {
        // Test generating email via temp-mail.io
        match TempMail::generate(TempMailProvider::Tempmail).await {
            Ok(email) => {
                println!("Generated temp-mail.io email: {}", email.address());
                assert!(!email.username().is_empty());
                assert!(!email.domain().is_empty());
                assert!(email.address().contains('@'));
                Ok(())
            }
            Err(e) => {
                println!("TempMail generation failed (this is ok for tests): {:?}", e);
                Ok(()) // Don't fail tests if external service is unavailable
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "mail")]
    async fn test_tempmail_generate_email_fake() -> Result<()> {
        // Test generating email via email-fake.com
        match TempMail::generate(TempMailProvider::EmailFake).await {
            Ok(email) => {
                println!("Generated email-fake.com email: {}", email.address());
                assert!(!email.username().is_empty());
                assert!(!email.domain().is_empty());
                assert!(email.address().contains('@'));
                Ok(())
            }
            Err(e) => {
                println!(
                    "EmailFake generation failed (this is ok for tests): {:?}",
                    e
                );
                Ok(()) // Don't fail tests if external service is unavailable
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "mail")]
    async fn test_tempmail_generate_secmail() -> Result<()> {
        // Test generating email via 1secmail (multiple domains)
        let domains = vec![
            SecMailDomain::SecMailCom,
            SecMailDomain::SecMailOrg,
            SecMailDomain::SecMailNet,
        ];

        for domain in domains {
            let provider = TempMailProvider::SecMail(domain);
            match TempMail::generate(provider).await {
                Ok(email) => {
                    println!("Generated 1secmail email: {}", email.address());
                    assert!(!email.username().is_empty());
                    assert!(!email.domain().is_empty());
                    assert!(email.address().contains('@'));
                }
                Err(e) => {
                    println!("SecMail generation failed: {:?}", e);
                }
            }
        }

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "mail")]
    async fn test_tempmail_random() -> Result<()> {
        // Test generating random email (uses random provider)
        match TempMail::random().await {
            Ok(email) => {
                println!("Generated random email: {}", email.address());
                assert!(!email.username().is_empty());
                assert!(!email.domain().is_empty());
                assert!(email.address().contains('@'));
                Ok(())
            }
            Err(e) => {
                println!(
                    "Random email generation failed (this is ok for tests): {:?}",
                    e
                );
                Ok(()) // Don't fail tests if external service is unavailable
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "mail")]
    async fn test_tempmail_secmail_generate_sync() {
        // Test synchronous sec_mail_generate
        let domains = vec![
            SecMailDomain::SecMailCom,
            SecMailDomain::EsiixCom,
            SecMailDomain::TxcctCom,
        ];

        for domain in domains {
            let email = TempMail::generate(TempMailProvider::SecMail(domain)).await;
            if let Ok(email) = email {
                println!("Generated SecMail email: {}", email.address());
                assert!(!email.username().is_empty());
                assert!(email.address().contains('@'));
            }
        }
    }

    #[test]
    #[cfg(not(feature = "mail"))]
    fn test_mail_feature_disabled() {
        // Test that mail tests are properly skipped when feature is disabled
        println!("Mail feature is disabled - tests skipped");
    }
}
