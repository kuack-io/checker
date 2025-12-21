use serde::{Deserialize, Serialize};
use url::Url;
use web_time::Instant;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metrics {
    pub url: String,
    pub total_time_ms: f64,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
#[wasm_bindgen]
#[cfg(not(tarpaulin_include))]
pub async fn check_endpoint(url: String) -> JsValue {
    console_error_panic_hook::set_once();

    let metrics = measure_endpoint(&url).await;
    serde_wasm_bindgen::to_value(&metrics).unwrap()
}

fn invalid_url_metrics(url_str: &str, parse_err: &url::ParseError) -> Metrics {
    Metrics {
        url: url_str.to_string(),
        total_time_ms: 0.0,
        success: false,
        error: Some(format!("Invalid URL format: {}", parse_err)),
    }
}

/// Measures endpoint performance with truly platform-agnostic code.
/// Identical implementation runs on all platforms (native, browser WASM, WASI).
/// No conditional compilation in business logic.
pub async fn measure_endpoint(url_str: &str) -> Metrics {
    if let Err(e) = Url::parse(url_str) {
        return invalid_url_metrics(url_str, &e);
    }

    let start_time = Instant::now();
    let client = build_http_client();

    match client.get(url_str).send().await {
        Ok(response) => {
            let status: reqwest::StatusCode = response.status();
            let total_time = elapsed_ms(&start_time);

            Metrics {
                url: url_str.to_string(),
                total_time_ms: total_time,
                success: status.is_success(),
                error: if status.is_success() {
                    None
                } else {
                    Some(format!("HTTP {}", status))
                },
            }
        }
        Err(e) => {
            let total_time = elapsed_ms(&start_time);

            Metrics {
                url: url_str.to_string(),
                total_time_ms: total_time,
                success: false,
                error: Some(format!("{}", e)),
            }
        }
    }
}

fn build_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .build()
        .expect("Failed to build reqwest client")
}

fn elapsed_ms(start_time: &Instant) -> f64 {
    start_time.elapsed().as_secs_f64() * 1000.0
}
