use kuack_checker::{Metrics, measure_endpoint};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_measure_endpoint_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let url = format!("{}/test", mock_server.uri());
    let metrics = measure_endpoint(&url).await;

    assert!(metrics.success);
    assert!(metrics.error.is_none());
    assert_eq!(metrics.url, url);
    assert!(metrics.total_time_ms >= 0.0);
}

#[tokio::test]
async fn test_measure_endpoint_error_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    let url = format!("{}/test", mock_server.uri());
    let metrics = measure_endpoint(&url).await;

    assert!(!metrics.success);
    assert!(metrics.error.is_some());
    assert!(metrics.error.unwrap().contains("404"));
}

#[tokio::test]
async fn test_measure_endpoint_invalid_url() {
    let url = "invalid-url";
    let metrics = measure_endpoint(url).await;

    assert!(!metrics.success);
    assert!(metrics.error.is_some());
    assert!(metrics.error.unwrap().contains("Invalid URL"));
}

#[tokio::test]
async fn test_measure_endpoint_connection_error() {
    // Use a port that is definitely closed.
    // Port 1 is usually reserved and closed.
    let url = "http://127.0.0.1:1";

    let metrics = measure_endpoint(url).await;

    assert!(!metrics.success);
    assert!(metrics.error.is_some());
    // Ensure it's not a 404 or similar HTTP error, but a connection error.
    // Connection errors usually don't start with "HTTP".
    assert!(!metrics.error.as_ref().unwrap().starts_with("HTTP"));
}

#[test]
fn test_metrics_traits() {
    let metrics = Metrics {
        url: "http://example.com".to_string(),
        total_time_ms: 100.0,
        success: true,
        error: None,
    };

    let cloned = metrics.clone();
    assert_eq!(metrics.url, cloned.url);

    let debug_str = format!("{:?}", metrics);
    assert!(debug_str.contains("Metrics"));

    let json = serde_json::to_string(&metrics).unwrap();
    let deserialized: Metrics = serde_json::from_str(&json).unwrap();
    assert_eq!(metrics.url, deserialized.url);
    assert_eq!(metrics.success, deserialized.success);
}
