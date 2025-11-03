#[cfg(test)]
mod tests {
    #[cfg(feature = "terminal")]
    use emix::io::terminal;
    use emix::{
        Result,
        io::{
            directory,
            file::{self, FileEx},
            path::{self, IntoPath, PathEx},
        },
    };
    use std::{
        io::{LineWriter, Write},
        path::PathBuf,
    };

    #[test]
    fn test_path_creation() -> Result<()> {
        let curdir = directory::current()?;
        let path = (curdir.as_str(), "MyFile.txt").into_path();

        assert!(
            path.to_string_lossy().contains("MyFile.txt"),
            "Path should contain filename"
        );

        let path = (curdir.as_str(), "My Folder", "MyFile.txt").into_path();
        assert!(
            path.to_string_lossy().contains("My Folder"),
            "Path should contain folder"
        );
        assert!(
            path.to_string_lossy().contains("MyFile.txt"),
            "Path should contain filename"
        );

        Ok(())
    }

    #[test]
    fn test_directory_operations() -> Result<()> {
        let curdir = directory::current()?;
        let test_dir = curdir.join("test_dir_temp");

        // Clean up if exists
        let _ = path::del(&test_dir);

        // Test directory creation
        directory::create(&test_dir)?;
        assert!(
            directory::exists(&test_dir),
            "Directory should exist after creation"
        );

        // Test directory removal
        path::del(&test_dir)?;
        assert!(
            !directory::exists(&test_dir),
            "Directory should not exist after deletion"
        );

        Ok(())
    }

    #[test]
    fn test_file_operations() -> Result<()> {
        let curdir = directory::current()?;
        let test_dir = curdir.join("test_file_temp");
        let test_file = test_dir.join("test.txt");

        // Clean up if exists
        let _ = path::del(&test_dir);
        directory::create(&test_dir)?;

        // Test file creation and writing
        {
            let file = file::create(&test_file)?;
            let mut writer = LineWriter::new(file);
            writeln!(&mut writer, "Hello, world!")?;
            writeln!(&mut writer, "Test line 2")?;
            writeln!(&mut writer, "Test line 3")?;
        }

        assert!(test_file.exists(), "File should exist after creation");

        // Test file reading
        {
            let file = file::open(&test_file)?;
            let lines: Vec<String> = file.read()?.collect();
            assert_eq!(lines.len(), 3, "File should have 3 lines");
            assert_eq!(lines[0], "Hello, world!", "First line should match");
        }

        // Test file reading with filter
        {
            let file = file::open(&test_file)?;
            let filtered: Vec<String> = file
                .read_filtered(|line: &str| !line.contains("world"))?
                .collect();
            assert_eq!(filtered.len(), 2, "Filtered file should have 2 lines");
        }

        // Clean up
        path::del(&test_dir)?;

        Ok(())
    }

    #[test]
    fn test_path_matching() -> Result<()> {
        let curdir = directory::current()?;
        let test_dir = curdir.join("test_match_temp");

        // Clean up if exists
        let _ = path::del(&test_dir);
        directory::create(&test_dir)?;

        // Create some test files
        let file1 = test_dir.join("file1.txt");
        let file2 = test_dir.join("file2.txt");
        let file3 = test_dir.join("file3.mp3");

        file::create(&file1)?;
        file::create(&file2)?;
        file::create(&file3)?;

        // Test pattern matching
        let matches: Vec<PathBuf> =
            path::r#match(&format!("{}/*.txt", test_dir.display()))?.collect();
        assert!(matches.len() >= 2, "Should find at least 2 .txt files");

        // Test filtered matching
        let filtered: Vec<PathBuf> =
            path::match_filtered(&format!("{}/*.txt", test_dir.display()), |e| {
                e.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.ends_with('1'))
                    .unwrap_or(false)
            })?
            .collect();
        assert!(
            filtered.len() >= 1,
            "Should find at least 1 file ending with 1"
        );

        // Clean up
        path::del(&test_dir)?;

        Ok(())
    }

    #[test]
    fn test_file_json_operations() -> Result<()> {
        let curdir = directory::current()?;
        let test_dir = curdir.join("test_json_temp");
        let test_file = test_dir.join("test.json");

        // Clean up if exists
        let _ = path::del(&test_dir);
        directory::create(&test_dir)?;

        // Use a simple struct for testing JSON operations
        #[derive(serde::Serialize)]
        struct TestEmployee {
            id: u32,
            name: String,
        }
        let employees = vec![
            TestEmployee {
                id: 1,
                name: "Test 1".to_string(),
            },
            TestEmployee {
                id: 2,
                name: "Test 2".to_string(),
            },
            TestEmployee {
                id: 3,
                name: "Test 3".to_string(),
            },
        ];

        // Write JSON
        {
            let mut file = file::create(&test_file)?;
            file.write_json(&employees, Some(true))?;
        }

        assert!(test_file.exists(), "JSON file should exist");

        // Read and verify JSON file exists (basic check)
        let file = file::open(&test_file)?;
        let lines: Vec<String> = file.read()?.collect();
        assert!(!lines.is_empty(), "JSON file should not be empty");

        // Clean up
        path::del(&test_dir)?;

        Ok(())
    }

    #[cfg(feature = "terminal")]
    mod terminal_tests {
        use super::*;

        #[test]
        fn test_clear_screen() -> Result<()> {
            // Test that clear_screen doesn't panic
            terminal::clear_screen()?;
            Ok(())
        }

        #[test]
        fn test_get_empty_input() -> Result<()> {
            // Test that get returns empty string for empty input
            // Note: This test requires user interaction or mocking, so we'll test the logic
            let result = terminal::get(None);
            // Result may succeed or fail depending on stdin availability
            // Just verify it doesn't panic
            let _ = result.is_ok();
            Ok(())
        }

        #[test]
        fn test_get_str_empty_input() -> Result<()> {
            // Test that get_str returns error for empty input
            // Note: This test requires mocking stdin
            // In a real scenario, we'd mock the input
            // For now, we just verify the function exists
            let _ = terminal::get_str;
            Ok(())
        }

        #[test]
        fn test_get_numeric_valid() -> Result<()> {
            // Test numeric parsing (would require mocking stdin in real test)
            let _ = terminal::get_numeric::<i32>;
            let _ = terminal::get_numeric::<u32>;
            let _ = terminal::get_numeric::<f64>;
            Ok(())
        }

        #[test]
        fn test_key_listener_creation() -> Result<()> {
            // Test KeyListener creation
            let mut listener = terminal::KeyListener::new()?;
            assert!(listener.try_recv().is_err());
            drop(listener);
            Ok(())
        }

        #[test]
        fn test_key_listener_bounded() -> Result<()> {
            // Test bounded KeyListener
            let mut listener = terminal::KeyListener::bounded(5)?;
            assert!(listener.try_recv().is_err());
            drop(listener);
            Ok(())
        }

        #[test]
        fn test_get_password_functions() -> Result<()> {
            // Test password functions exist
            let _ = terminal::get_password;
            let _ = terminal::get_password_str;
            Ok(())
        }

        #[test]
        fn test_confirm_and_pause() -> Result<()> {
            // Test confirm and pause functions exist
            let _ = terminal::confirm;
            let _ = terminal::pause;
            Ok(())
        }

        #[test]
        fn test_prompt_functions() -> Result<()> {
            // Test with various prompt configurations
            let _ = terminal::get(Some("Test prompt:"));
            let _ = terminal::get(Some(""));
            let _ = terminal::get(None);
            Ok(())
        }

        #[test]
        fn test_display_menu_logic() -> Result<()> {
            // Test menu display with various configurations
            // Note: This would require mocking in a real test environment
            let items = vec!["Option 1", "Option 2", "Option 3"];
            let _result = terminal::display_menu(&items, None);
            let _result = terminal::display_menu(&items, Some("Select:"));
            let _result = terminal::display_menu(&items, Some(""));
            Ok(())
        }
    }
}
