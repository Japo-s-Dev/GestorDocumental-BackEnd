#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
		"pwd": "welcome"
		}),
	);
	req_login.await?.print().await?;

	Ok(())
}

// cargo watch -q -c -w examples/ -x "run --example quick_dev"
