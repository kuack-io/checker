use anyhow::{Context, Result};
use kuack_checker::measure_endpoint;
use std::env;

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<()> {
    match run().await {
        Ok(success) => {
            if !success {
                std::process::exit(1);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = run().await {
            web_sys::console::error_1(&format!("Error: {}", e).into());
        }
    });
}

#[cfg(all(target_family = "wasm", not(target_os = "unknown")))]
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime");

    rt.block_on(async {
        if let Err(e) = run().await {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    });
}

async fn run() -> Result<bool> {
    let target_url =
        env::var("TARGET_URL").context("TARGET_URL environment variable is required")?;

    let metrics = measure_endpoint(&target_url).await;

    let json_output =
        serde_json::to_string_pretty(&metrics).context("Failed to serialize metrics to JSON")?;

    println!("{}", json_output);

    Ok(metrics.success)
}
