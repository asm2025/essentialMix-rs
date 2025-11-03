#[cfg(test)]
mod tests {
    use emix::string::StringEx;

    #[test]
    fn test_trim_single_char() {
        let s = "xxxhello worldxxx";
        let ch = 'x';
        let result = s.trim_char(&ch);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_trim_single_char_no_match() {
        let s = "hello world";
        let ch = 'x';
        let result = s.trim_char(&ch);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_trim_empty_string() {
        let s = "";
        let ch = 'x';
        let result = s.trim_char(&ch);
        assert_eq!(result, "");
    }

    #[test]
    fn test_trim_start_single_char() {
        let s = "xxxhello world";
        let ch = 'x';
        let result = s.trim_start_char(&ch);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_trim_end_single_char() {
        let s = "hello worldxxx";
        let ch = 'x';
        let result = s.trim_end_char(&ch);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_trim_many_chars() {
        let s = "xyzhello worldxyz";
        let result = s.trim_many(&['x', 'y', 'z']);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_trim_start_many_chars() {
        let s = "xyzhello world";
        let result = s.trim_start_many(&['x', 'y', 'z']);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_trim_end_many_chars() {
        let s = "hello worldxyz";
        let result = s.trim_end_many(&['x', 'y', 'z']);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_prefix_adds_prefix() {
        let s = "hello";
        let result = s.prefix('/');
        assert_eq!(result, "/hello");
    }

    #[test]
    fn test_prefix_already_has_prefix() {
        let s = "/hello";
        let result = s.prefix('/');
        assert_eq!(result, "/hello");
    }

    #[test]
    fn test_suffix_adds_suffix() {
        let s = "hello";
        let result = s.suffix('/');
        assert_eq!(result, "hello/");
    }

    #[test]
    fn test_suffix_already_has_suffix() {
        let s = "hello/";
        let result = s.suffix('/');
        assert_eq!(result, "hello/");
    }

    #[test]
    fn test_find_first() {
        let s = "hello world";
        let result = s.find_first(|c| c.is_uppercase());
        assert_eq!(result, None);

        let result = s.find_first(|c| c == 'o');
        assert_eq!(result, Some(('o', 4)));
    }

    #[test]
    fn test_find_first_empty() {
        let s = "";
        let result = s.find_first(|c| c == 'x');
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_last() {
        let s = "hello world";
        let result = s.find_last(|c| c == 'o');
        assert_eq!(result, Some(('o', 7)));

        let result = s.find_last(|c| c == 'l');
        assert_eq!(result, Some(('l', 9)));
    }

    #[test]
    fn test_find_last_empty() {
        let s = "";
        let result = s.find_last(|c| c == 'x');
        assert_eq!(result, None);
    }

    #[test]
    fn test_trim_many_empty_string() {
        let s = "";
        let result = s.trim_many(&['x', 'y']);
        assert_eq!(result, "");
    }
}
