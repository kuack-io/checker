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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elapsed_ms() {
        let start = Instant::now();
        let elapsed = elapsed_ms(&start);
        assert!(elapsed >= 0.0);
    }

    #[test]
    fn test_build_http_client() {
        let client = build_http_client();
        assert!(client.get("http://localhost").build().is_ok());
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
#[wasm_bindgen]
#[cfg(not(tarpaulin_include))]
pub async fn main(env: JsValue) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let env_map: std::collections::HashMap<String, String> = serde_wasm_bindgen::from_value(env)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse env: {}", e)))?;

    let target_url = env_map
        .get("TARGET_URL")
        .ok_or_else(|| JsValue::from_str("TARGET_URL environment variable is required"))?;

    let metrics = measure_endpoint(target_url).await;

    let json = serde_json::to_string(&metrics).map_err(|e| JsValue::from_str(&e.to_string()))?;

    web_sys::console::log_1(&json.into());

    if !metrics.success {
        return Err(JsValue::from_str("Check failed"));
    }

    Ok(())
}
