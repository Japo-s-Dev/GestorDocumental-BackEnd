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

	let req_create_user = hc.do_post(
		"/api/rpc",
		json!({
		"id": 1,
				"method": "create_user",
			  "params": {
				  "data": {
						"username": "Japo",
						"pwd_clear": "Japo123",
						"email": "japo@japo.com"
					}
			}
		}),
	);
	req_create_user.await?.print().await?;

	let req_list_users = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_users"
		}),
	);
	req_list_users.await?.print().await?;

	let req_update_user = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_user",
			"params": {
				"id": 43,
				"data": {
					"username": "Japo",
					"email": "pue22296@uvg.edu.gt"
				}
			}
		}),
	);
	req_update_user.await?.print().await?;

	let req_delete_user = hc.do_post(
		"/api/rpc",
		json!({
		"id": 1,
			  "method": "delete_user",
			  "params": {
				  "id": 43
			  }
		  }),
	);
	req_delete_user.await?.print().await?;

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	req_logoff.await?.print().await?;
	//

	Ok(())
}

// cargo watch -q -c -w examples/ -x "run --example quick_dev"
