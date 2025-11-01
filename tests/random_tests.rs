#[cfg(test)]
mod tests {
    use chrono::Utc;
    use emix::random;

    #[test]
    fn test_random_string() {
        let s = random::string(10);
        assert_eq!(s.len(), 10, "String should be exactly 10 characters");
    }

    #[test]
    fn test_random_alphanum_str() {
        let s = random::alphanum_str(10);
        assert_eq!(s.len(), 10, "Alphanumeric string should be exactly 10 characters");
        assert!(s.chars().all(|c| c.is_alphanumeric()), "String should contain only alphanumeric characters");
    }

    #[test]
    fn test_random_boolean() {
        let b = random::boolean();
        assert!(b == true || b == false, "Boolean should be true or false");
    }

    #[test]
    fn test_random_float() {
        let f = random::float();
        assert!(f >= 0.0 && f < 1.0, "Float should be between 0.0 and 1.0");
    }

    #[test]
    fn test_random_numeric() {
        let n = random::numeric(1..10);
        assert!(n >= 1 && n < 10, "Numeric should be in range 1..10");
    }

    #[test]
    fn test_random_uuid() {
        let uuid = random::uuid();
        assert!(!uuid.is_empty(), "UUID should not be empty");
        assert_eq!(uuid.len(), 36, "Standard UUID should be 36 characters");
    }

    #[test]
    fn test_random_uuid_v3() {
        let uuid = random::uuid_v(random::UuidVersion::V3);
        assert!(!uuid.is_empty(), "UUID v3 should not be empty");
    }

    #[test]
    fn test_random_uuid_v5() {
        let uuid = random::uuid_v(random::UuidVersion::V5);
        assert!(!uuid.is_empty(), "UUID v5 should not be empty");
    }

    #[test]
    fn test_random_address_building() {
        let building = random::address::building();
        assert!(!building.is_empty(), "Building should not be empty");
    }

    #[test]
    fn test_random_address_street() {
        let street = random::address::street();
        assert!(!street.is_empty(), "Street should not be empty");
    }

    #[test]
    fn test_random_address_city() {
        let city = random::address::city();
        assert!(!city.is_empty(), "City should not be empty");
    }

    #[test]
    fn test_random_address_state() {
        let state = random::address::state();
        assert!(!state.is_empty(), "State should not be empty");
    }

    #[test]
    fn test_random_address_country() {
        let country = random::address::country();
        assert!(!country.is_empty(), "Country should not be empty");
    }

    #[test]
    fn test_random_address_zipcode() {
        let zipcode = random::address::zipcode();
        assert!(!zipcode.is_empty(), "Zipcode should not be empty");
    }

    #[test]
    fn test_random_datetime() {
        let date = Utc::now();
        let before = random::datetime::before(date);
        let after = random::datetime::after(date);
        
        assert!(before < date, "Before date should be in the past");
        assert!(after > date, "After date should be in the future");
        
        let between = random::datetime::between(before, after);
        assert!(between >= before && between <= after, "Between date should be within range");
    }

    #[test]
    fn test_random_internet_ipv4() {
        let ip = random::internet::ipv4();
        assert!(!ip.is_empty(), "IPv4 should not be empty");
        // Basic validation that it looks like an IP
        let parts: Vec<&str> = ip.split('.').collect();
        assert_eq!(parts.len(), 4, "IPv4 should have 4 parts");
    }

    #[test]
    fn test_random_internet_ipv6() {
        let ip = random::internet::ipv6();
        assert!(!ip.is_empty(), "IPv6 should not be empty");
    }

    #[test]
    fn test_random_internet_username() {
        let username = random::internet::username();
        assert!(!username.is_empty(), "Username should not be empty");
    }

    #[test]
    fn test_random_internet_email() {
        let email = random::internet::free_email();
        assert!(!email.is_empty(), "Email should not be empty");
        assert!(email.contains('@'), "Email should contain @");
    }

    #[test]
    fn test_random_lorem_word() {
        let word = random::lorem::word();
        assert!(!word.is_empty(), "Word should not be empty");
    }

    #[test]
    fn test_random_lorem_sentence() {
        let sentence = random::lorem::sentence(1..10);
        assert!(!sentence.is_empty(), "Sentence should not be empty");
    }

    #[test]
    fn test_random_person_name() {
        let name = random::person::name();
        assert!(!name.is_empty(), "Name should not be empty");
    }

    #[test]
    fn test_random_person_first_name() {
        let first_name = random::person::first_name();
        assert!(!first_name.is_empty(), "First name should not be empty");
    }

    #[test]
    fn test_random_person_last_name() {
        let last_name = random::person::last_name();
        assert!(!last_name.is_empty(), "Last name should not be empty");
    }
}

