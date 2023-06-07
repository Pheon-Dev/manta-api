#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );

    req_login.await?.print().await?;

    let req_create_send_req = hc.do_post(
        "/api/rpc",
        json!({
            "id": null,
            "method": "create_send_req",
            "params": {
                "data": {
                    "amount": "100",
                    "currency": "KES",
                    "receiver": "John",
                    "description": "Lunch",
                }
            }
        }),
    );

    req_create_send_req.await?.print().await?;

    let req_update_send_req = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_send_req",
            "params": {
                "id": 1,
                "data": {
                    "currency": "USD",
                }
            }
        }),
    );

    req_update_send_req.await?.print().await?;

    let req_delete_send_req = hc.do_post(
        "/api/rpc",
        json!({
            "id": null,
            "method": "delete_send_req",
            "params": {
                "id": 1,
            }
        }),
    );

    // req_delete_send_req.await?.print().await?;

    let req_list_send_req = hc.do_post(
        "/api/rpc",
        json!({
            "id": 2,
            "method": "list_send_req",
        }),
    );

    req_list_send_req.await?.print().await?;

    Ok(())
}
