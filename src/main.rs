#![windows_subsystem = "windows"]

use std::process::{Command};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    let mut time = get_time_millis();
    let mut value:bool;
    loop {
        if get_time_millis() < time + 5000 { continue; }

        let response = reqwest::get("http://97.107.130.20:8888/value").await;
        if response.is_err() {
            time = get_time_millis();
            continue;
        }
        let text = response.unwrap().text().await.unwrap();
        if text.eq("yes") { value = true; }
        else { value = false; }

        if !value { kill_valorant() }

        time = get_time_millis();
    }
}

fn kill_valorant() {
    Command::new("taskkill")
        .arg("/IM")
        .arg("VALORANT-Win64-Shipping.exe")
        .arg("/F")
        .output()
        .expect("Failed to execute process");
    Command::new("taskkill")
        .arg("/IM")
        .arg("VALORANT.exe")
        .arg("/F")
        .output()
        .expect("Failed to execute process");
}

fn get_time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_millis()
}