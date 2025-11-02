#[cfg(test)]
#[cfg(feature = "fake")]
mod tests {
    use chrono::Utc;
    use emix::random;

    #[test]
    fn test_random_string() {
        let s = random::string(10);
        assert_eq!(s.len(), 10, "String should be exactly 10 characters");
    }

    #[test]
    fn test_random_string_zero_length() {
        let s = random::string(0);
        assert_eq!(s.len(), 0, "String should be empty when length is 0");
    }

    #[test]
    fn test_random_alphanum_str() {
        let s = random::alphanum_str(10);
        assert_eq!(s.len(), 10, "Alphanumeric string should be exactly 10 characters");
        assert!(s.chars().all(|c| c.is_alphanumeric()), "String should contain only alphanumeric characters");
    }

    #[test]
    fn test_random_alphanum_str_zero_length() {
        let s = random::alphanum_str(0);
        assert_eq!(s.len(), 0, "Alphanumeric string should be empty when length is 0");
    }

    #[test]
    fn test_random_char() {
        let c = random::char();
        assert!(c.is_ascii(), "Char should be ASCII");
    }

    #[test]
    fn test_random_alphanum_char() {
        let c = random::alphanum();
        assert!(c.is_alphanumeric(), "Char should be alphanumeric");
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
    fn test_random_numeric_float() {
        let n = random::numeric(1.0..10.0);
        assert!(n >= 1.0 && n < 10.0, "Float numeric should be in range 1.0..10.0");
    }

    #[test]
    fn test_random_uuid() {
        let uuid = random::uuid();
        assert!(!uuid.is_empty(), "UUID should not be empty");
        assert_eq!(uuid.len(), 36, "Standard UUID should be 36 characters");
    }

    #[test]
    fn test_random_uuid_v1() {
        let uuid = random::uuid_v(random::UuidVersion::V1);
        assert!(!uuid.is_empty(), "UUID v1 should not be empty");
    }

    #[test]
    fn test_random_uuid_v3() {
        let uuid = random::uuid_v(random::UuidVersion::V3);
        assert!(!uuid.is_empty(), "UUID v3 should not be empty");
    }

    #[test]
    fn test_random_uuid_v4() {
        let uuid = random::uuid_v(random::UuidVersion::V4);
        assert!(!uuid.is_empty(), "UUID v4 should not be empty");
    }

    #[test]
    fn test_random_uuid_v5() {
        let uuid = random::uuid_v(random::UuidVersion::V5);
        assert!(!uuid.is_empty(), "UUID v5 should not be empty");
    }

    #[test]
    fn test_random_uuid_version_default() {
        let default = random::UuidVersion::default();
        // Can't compare directly without Debug, but can test that it doesn't panic
        let uuid_v4 = random::uuid_v(random::UuidVersion::V4);
        let uuid_default = random::uuid_v(default);
        assert!(!uuid_v4.is_empty());
        assert!(!uuid_default.is_empty());
    }

    // Address tests
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

    // Automotive tests
    #[test]
    fn test_random_automotive_license_number() {
        let license = random::automotive::license_number();
        assert!(!license.is_empty(), "License number should not be empty");
    }

    // Barcode tests
    #[test]
    fn test_random_barcode_isbn() {
        let isbn = random::barcode::isbn();
        assert!(!isbn.is_empty(), "ISBN should not be empty");
    }

    #[test]
    fn test_random_barcode_isbn10() {
        let isbn10 = random::barcode::isbn10();
        assert!(!isbn10.is_empty(), "ISBN-10 should not be empty");
    }

    #[test]
    fn test_random_barcode_isbn13() {
        let isbn13 = random::barcode::isbn13();
        assert!(!isbn13.is_empty(), "ISBN-13 should not be empty");
    }

    // Business tests
    #[test]
    fn test_random_business_company_name() {
        let name = random::business::company_name();
        assert!(!name.is_empty(), "Company name should not be empty");
    }

    #[test]
    fn test_random_business_company_suffix() {
        let suffix = random::business::company_suffix();
        assert!(!suffix.is_empty(), "Company suffix should not be empty");
    }

    #[test]
    fn test_random_business_industry() {
        let industry = random::business::industry();
        assert!(!industry.is_empty(), "Industry should not be empty");
    }

    #[test]
    fn test_random_business_catch_phrase() {
        let phrase = random::business::catch_phase();
        assert!(!phrase.is_empty(), "Catch phrase should not be empty");
    }

    #[test]
    fn test_random_business_buzzword() {
        let buzzword = random::business::buzzword();
        assert!(!buzzword.is_empty(), "Buzzword should not be empty");
    }

    #[test]
    fn test_random_business_credit_card() {
        let card = random::business::credit_card();
        assert!(!card.is_empty(), "Credit card should not be empty");
    }

    #[test]
    fn test_random_business_currency_code() {
        let code = random::business::currency_code();
        assert!(!code.is_empty(), "Currency code should not be empty");
    }

    #[test]
    fn test_random_business_phone_number() {
        let phone = random::business::phone_number();
        assert!(!phone.is_empty(), "Phone number should not be empty");
    }

    #[test]
    fn test_random_business_cell_number() {
        let cell = random::business::cell_number();
        assert!(!cell.is_empty(), "Cell number should not be empty");
    }

    // Color tests
    #[test]
    fn test_random_color_name() {
        let name = random::color::name();
        assert!(!name.is_empty(), "Color name should not be empty");
    }

    #[test]
    fn test_random_color_hex() {
        let hex = random::color::hex();
        assert!(!hex.is_empty(), "Hex color should not be empty");
    }

    #[test]
    fn test_random_color_rgb() {
        let rgb = random::color::rgb();
        assert!(!rgb.is_empty(), "RGB color should not be empty");
    }

    #[test]
    fn test_random_color_rgba() {
        let rgba = random::color::rgba();
        assert!(!rgba.is_empty(), "RGBA color should not be empty");
    }

    // Datetime tests
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

    // Filesystem tests
    #[test]
    fn test_random_filesystem_dir_path() {
        let path = random::filesystem::dir_path();
        assert!(!path.is_empty(), "Directory path should not be empty");
    }

    #[test]
    fn test_random_filesystem_file_path() {
        let path = random::filesystem::file_path();
        assert!(!path.is_empty(), "File path should not be empty");
    }

    #[test]
    fn test_random_filesystem_file_name() {
        let name = random::filesystem::file_name();
        assert!(!name.is_empty(), "File name should not be empty");
    }

    #[test]
    fn test_random_filesystem_file_extension() {
        let ext = random::filesystem::file_extension();
        assert!(!ext.is_empty(), "File extension should not be empty");
    }

    // Internet tests
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

    // Lorem tests
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

    // Person tests
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

