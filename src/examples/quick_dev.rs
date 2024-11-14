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
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use reqwest::Client;

    // This test is already provided for login
    #[tokio::test]
    async fn test_login() -> Result<()> {
        let _m = mock("POST", "/api/login")
            .with_status(200)
            .with_body("Login Success")
            .create();

        let client = Client::new();
        let res = client.post(&format!("{}/api/login", &server_url()))
            .json(&json!({
                "username": "demo1",
                "pwd": "welcome"
            }))
            .send().await?;
        assert_eq!(res.text().await?, "Login Success");

        Ok(())
    }

    // Test for listing users
    #[tokio::test]
    async fn test_list_users() -> Result<()> {
        let _m = mock("POST", "/api/rpc")
            .match_body(mockito::Matcher::Json(json!({
                "id": 1,
                "method": "list_users"
            })))
            .with_status(200)
            .with_body("Users Listed")
            .create();

        let client = Client::new();
        let res = client.post(&format!("{}/api/rpc", &server_url()))
            .json(&json!({
                "id": 1,
                "method": "list_users"
            }))
            .send().await?;
        assert_eq!(res.text().await?, "Users Listed");

        Ok(())
    }

    // Test for updating a user
    #[tokio::test]
    async fn test_update_user() -> Result<()> {
        let _m = mock("POST", "/api/rpc")
            .match_body(mockito::Matcher::Json(json!({
                "id": 1,
                "method": "update_user",
                "params": {
                    "id": 43,
                    "data": {
                        "username": "Japo",
                        "email": "pue22296@uvg.edu.gt"
                    }
                }
            })))
            .with_status(200)
            .with_body("User Updated")
            .create();

        let client = Client::new();
        let res = client.post(&format!("{}/api/rpc", &server_url()))
            .json(&json!({
                "id": 1,
                "method": "update_user",
                "params": {
                    "id": 43,
                    "data": {
                        "username": "Japo",
                        "email": "pue22296@uvg.edu.gt"
                    }
                }
            }))
            .send().await?;
        assert_eq!(res.text().await?, "User Updated");

        Ok(())
    }

    // Test for deleting a user
    #[tokio::test]
    async fn test_delete_user() -> Result<()> {
        let _m = mock("POST", "/api/rpc")
            .match_body(mockito::Matcher::Json(json!({
                "id": 1,
                "method": "delete_user",
                "params": {
                    "id": 43
                }
            })))
            .with_status(200)
            .with_body("User Deleted")
            .create();

        let client = Client::new();
        let res = client.post(&format!("{}/api/rpc", &server_url()))
            .json(&json!({
                "id": 1,
                "method": "delete_user",
                "params": {
                    "id": 43
                }
            }))
            .send().await?;
        assert_eq!(res.text().await?, "User Deleted");

        Ok(())
    }

    // This test is already provided for logoff
    #[tokio::test]
    async fn test_logoff() -> Result<()> {
        let _m = mock("POST", "/api/logoff")
            .with_status(200)
            .with_body("Logged Off")
            .create();

        let client = Client::new();
        let res = client.post(&format!("{}/api/logoff", &server_url()))
            .json(&json!({"logoff": true}))
            .send().await?;
        assert_eq!(res.text().await?, "Logged Off");

        Ok(())
    }
}
