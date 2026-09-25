#[cfg(test)]
mod tests {
    use emixcore::{Error, Result};
    use emixnet::web::*;
    use httpmock::prelude::*;
    use serde::Serialize;
    use serde_json::{Value, json};
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

    // The tests below run against a local mock server so they are fast and
    // deterministic; `test_live_httpbin_get` covers a real network round trip.

    #[tokio::test]
    async fn test_reqwest_get() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/get")
                    .query_param("p1", "foo")
                    .query_param("p2", "baz")
                    // Default headers from build_client()
                    .header("cache-control", "no-cache")
                    .header("pragma", "no-cache");
                then.status(200)
                    .json_body(json!({ "url": server.url("/get?p1=foo&p2=baz") }));
            })
            .await;

        let client = reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (server.base_url().as_str(), "get?p1=foo&p2=baz").as_url()?;
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

        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_reqwest_post() -> Result<()> {
        let body = get_employees(3);
        let expected_body = serde_json::to_value(&body).map_err(Error::from_std_error)?;

        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(POST).path("/post").json_body(expected_body.clone());
                then.status(200).json_body(json!({ "data": expected_body }));
            })
            .await;

        let client = reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (server.base_url().as_str(), "post").as_url()?;
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

        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_reqwest_get_ip() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/ip");
                then.status(200).json_body(json!({ "origin": "203.0.113.7" }));
            })
            .await;

        let client = reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (server.base_url().as_str(), "ip").as_url()?;
        let response: HashMap<String, String> = client
            .get(url)
            .send()
            .await
            .map_err(Error::from_std_error)?
            .json()
            .await
            .map_err(Error::from_std_error)?;

        assert_eq!(response.get("origin").map(String::as_str), Some("203.0.113.7"));

        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_api_client_sends_json_headers() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/api")
                    .header("accept", "application/json")
                    .header("cache-control", "no-cache");
                then.status(200).json_body(json!({ "ok": true }));
            })
            .await;

        let client = reqwestx::build_client_for_api()
            .build()
            .map_err(Error::from_std_error)?;

        let response = client
            .get(server.url("/api"))
            .send()
            .await
            .map_err(Error::from_std_error)?;
        assert!(response.status().is_success());

        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_client_with_user_agent() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/ua").header("user-agent", "emix-test/1.0");
                then.status(204);
            })
            .await;

        let client = reqwestx::build_client_with_user_agent("emix-test/1.0".to_string())
            .build()
            .map_err(Error::from_std_error)?;

        let response = client
            .get(server.url("/ua"))
            .send()
            .await
            .map_err(Error::from_std_error)?;
        assert_eq!(response.status().as_u16(), 204);

        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_reqwest_error_status_is_reported() -> Result<()> {
        let server = MockServer::start_async().await;
        server
            .mock_async(|when, then| {
                when.method(GET).path("/missing");
                then.status(404);
            })
            .await;

        let client = reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let response = client
            .get(server.url("/missing"))
            .send()
            .await
            .map_err(Error::from_std_error)?;
        assert_eq!(response.status().as_u16(), 404);
        assert!(response.error_for_status().is_err());

        Ok(())
    }

    #[test]
    fn test_blocking_reqwest_get() -> Result<()> {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/get")
                .query_param("p1", "foo")
                .query_param("p2", "baz")
                .header("cache-control", "no-cache");
            then.status(200)
                .json_body(json!({ "url": server.url("/get?p1=foo&p2=baz") }));
        });

        let client = reqwestx::build_blocking_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = (server.base_url().as_str(), "get?p1=foo&p2=baz").as_url()?;
        let response = client.get(url).send().map_err(Error::from_std_error)?;

        assert!(response.status().is_success(), "GET request should succeed");

        let json: Value = response.json().map_err(Error::from_std_error)?;
        assert!(
            json.get("url").is_some(),
            "Response should contain 'url' field"
        );

        mock.assert();
        Ok(())
    }

    // Live network smoke test against httpbin.org (a public service that stalls at times).
    // Run with: cargo test -p emixnet --test web -- --ignored
    #[tokio::test]
    #[ignore]
    async fn test_live_httpbin_get() -> Result<()> {
        let client = reqwestx::build_client()
            .build()
            .map_err(Error::from_std_error)?;

        let url = ("https://httpbin.org", "get?p1=foo&p2=baz").as_url()?;
        let json: Value = client
            .get(url)
            .send()
            .await
            .map_err(Error::from_std_error)?
            .json()
            .await
            .map_err(Error::from_std_error)?;

        assert_eq!(json["args"]["p1"], "foo");
        assert_eq!(json["args"]["p2"], "baz");
        Ok(())
    }
}
