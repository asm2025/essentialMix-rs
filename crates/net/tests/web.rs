#[cfg(test)]
mod tests {
    use emixcore::{Error, Result};
    use emixnet::web::*;
    use serde::Serialize;
    use serde_json::Value;
    use std::collections::HashMap;

    // In Rust integration tests, each file is a separate crate
    // So we define test utilities locally
    #[derive(Serialize)]
    struct Employee {
        id: u32,
        #[serde(rename = "employee_name")]
        name: String,
        #[serde(rename = "employee_age")]
        age: u8,
    }

    fn get_employees(count: usize) -> Vec<Employee> {
        (1..=count)
            .map(|i| Employee {
                id: i as u32,
                name: format!("Employee {}", i),
                age: (i % 100) as u8,
            })
            .collect()
    }

    #[test]
    fn test_url_creation() -> Result<()> {
        // Test absolute URL
        let url = "https://www.rust-lang.org".as_url()?;
        assert!(
            url.as_str().starts_with("https://"),
            "URL should be absolute"
        );
        assert!(
            url.as_str().contains("rust-lang.org"),
            "URL should contain domain"
        );

        // Test URL from parts
        let url = ("https://www.rust-lang.org", "en-US", "documentation").as_url()?;
        assert!(
            url.as_str().contains("documentation"),
            "URL should contain path"
        );

        // Test relative URL - gets converted to absolute URL with localhost
        let url = "/path/to/relative/url".as_url()?;
        assert!(
            url.as_str().contains("/path/to/relative/url"),
            "Relative URL should contain the path"
        );
        assert!(
            url.as_str().starts_with("https://"),
            "Relative URL should be converted to absolute"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_reqwest_get() -> Result<()> {
        const BASE_URL: &str = "https://httpbin.org";

        let client = emixnet::web::reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (BASE_URL, "get?p1=foo&p2=baz").as_url()?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(Error::from_std_error)?;

        assert!(response.status().is_success(), "GET request should succeed");

        let json: Value = response.json().await.map_err(Error::from_std_error)?;

        assert!(
            json.get("url").is_some(),
            "Response should contain 'url' field"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_reqwest_post() -> Result<()> {
        const BASE_URL: &str = "https://httpbin.org";

        let client = emixnet::web::reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (BASE_URL, "post").as_url()?;
        let body = get_employees(3);

        let response: Value = client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(Error::from_std_error)?
            .json()
            .await
            .map_err(Error::from_std_error)?;

        assert!(
            response.get("data").is_some(),
            "Response should contain 'data' field"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_reqwest_get_ip() -> Result<()> {
        const BASE_URL: &str = "https://httpbin.org";

        let client = emixnet::web::reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (BASE_URL, "ip").as_url()?;
        let response: HashMap<String, String> = client
            .get(url)
            .send()
            .await
            .map_err(Error::from_std_error)?
            .json()
            .await
            .map_err(Error::from_std_error)?;

        assert!(
            response.contains_key("origin"),
            "Response should contain 'origin' key"
        );
        let ip = response.get("origin").unwrap();
        assert!(!ip.is_empty(), "IP address should not be empty");

        Ok(())
    }

    #[test]
    fn test_blocking_reqwest_get() -> Result<()> {
        const BASE_URL: &str = "https://httpbin.org";

        let client = emixnet::web::reqwestx::build_blocking_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (BASE_URL, "get?p1=foo&p2=baz").as_url()?;
        let response = client.get(url).send().map_err(Error::from_std_error)?;

        assert!(response.status().is_success(), "GET request should succeed");

        let json: Value = response.json().map_err(Error::from_std_error)?;
        assert!(
            json.get("url").is_some(),
            "Response should contain 'url' field"
        );

        Ok(())
    }
}

