// #![allow(unused)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions)),
    target_os = "windows",
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
#[tauri::command]
async fn login_request(username: String, password: String) -> String {
    let url = format!("http://127.0.0.1:8080/api/login");

    let mut map = HashMap::new();
    map.insert("username", &username);
    map.insert("password", &password);

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return res;
}

#[tauri::command]
async fn create_request(amount: String, receiver: String, description: String) -> String {
    let url = format!("http://127.0.0.1:8080/api/create");
    let mut map = HashMap::new();
    map.insert("amount", amount.parse().unwrap());
    map.insert("receiver", receiver);
    map.insert("description", description);

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return res;
}

#[tauri::command]
async fn list_requests() -> String {
    let url = format!("http://127.0.0.1:8080/api/send");
    let res = reqwest::get(&url).await.unwrap().text().await.unwrap();
    return res;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            login_request,
            create_request,
            list_requests
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
