
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde_json::{json, Value};
use std::{fs, thread, time::Duration, sync::{Arc, Mutex}};

fn main() {
    let token = "MTyNg.GL83rn.";
    let server_id = "1411439140708421716";
    let password = "";
    let mfa_token = Arc::new(Mutex::new(String::new()));

    let mfa_token_clone = Arc::clone(&mfa_token);
    loop {
        if let Some(new_token) = get_mfa_token(token, server_id, password) {
            let mut mfa = mfa_token_clone.lock().unwrap();
            *mfa = new_token.clone();

            let token_data = json!({ "token": new_token });
            if let Ok(json_string) = serde_json::to_string(&token_data) {
                if fs::write("mfa_token.json", json_string).is_ok() {
                    println!("MFA token saved");
                }
            }
        } else {
            println!("Failed to get MFA token, retrying...");
        }
        thread::sleep(Duration::from_secs(90));
    }
}

fn get_mfa_token(token: &str, server_id: &str, password: &str) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .ok()?;

    // Step 1: PATCH vanity URL
    let patch_body = json!({ "code": null });
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"));
    headers.insert(
        "X-Super-Properties",
        HeaderValue::from_static("eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiQ2hyb21lIiwiZGV2aWNlIjoiIiwic3lzdGVtX2xvY2FsZSI6InRyLVRSIiwiYnJvd3Nlcl91c2VyX2FnZW50IjoiTW96aWxsYS81LjAgKFdpbmRvd3MgTlQgMTAuMDsgV2luNjQ7IHg2NCkiLCJicm93c2VyX3ZlcnNpb24iOiIxMjEuMC4wLjAiLCJvc192ZXJzaW9uIjoiMTAiLCJyZWZlcnJlciI6IiIsInJlZmVycmluZ19kb21haW4iOiIiLCJyZWZlcnJlcl9jdXJyZW50IjoiIiwicmVmZXJyaW5nX2RvbWFpbl9jdXJyZW50IjoiIiwicmVsZWFzZV9jaGFubmVsIjoic3RhYmxlIiwiY2xpZW50X2J1aWxkX251bWJlciI6MjAwODQyLCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsfQ==")
        .unwrap();

    let patch_resp = client
        .patch(&format!("https://discord.com/api/v9/guilds/{}/vanity-url", server_id))
        .headers(headers.clone())
        .json(&patch_body)
        .send();

    let patch_resp = match patch_resp {
        Ok(resp) => resp,
        Err(e) => {
            println!("URL request error: {}", e);
            return None;
        }
    };

    let patch_json: Value = match patch_resp.json() {
        Ok(v) => v,
        Err(e) => {
            println!("JSON parse error: {}", e);
            return None;
        }
    };

    let ticket = if let Some(mfa) = patch_json.get("mfa") {
        mfa.get("ticket").and_then(|t| t.as_str()).unwrap_or("")
    } else {
        patch_json.get("ticket").and_then(|t| t.as_str()).unwrap_or("")
    };

    if ticket.is_empty() {
        println!("Failed to get MFA ticket");
        return None;
    }

    println!("MFA ticket received");

    // Step 2: Finish MFA
    let finish_body = json!({
        "ticket": ticket,
        "mfa_type": "password",
        "data": password
    });

    let finish_resp = client
        .post("https://discord.com/api/v9/mfa/finish")
        .headers(headers)
        .json(&finish_body)
        .send();

    let finish_resp = match finish_resp {
        Ok(resp) => resp,
        Err(e) => {
            println!("Failed to send MFA request: {}", e);
            return None;
        }
    };

    let finish_json: Value = match finish_resp.json() {
        Ok(v) => v,
        Err(_) => {
            println!("JSON parse error on MFA finish");
            return None;
        }
    };

    if let Some(new_token) = finish_json.get("token").and_then(|t| t.as_str()) {
        println!("Successfully obtained MFA token");
        return Some(new_token.to_string());
    }

    println!("Failed to get MFA token");
    None
}
