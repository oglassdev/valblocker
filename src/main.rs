#![windows_subsystem = "windows"]

use std::process::{Command};
use std::{process, thread, time};
use sysinfo::Process;

#[tokio::main]
async fn main() {
    let five = time::Duration::from_secs(5);
    let mut value:bool;

    loop {
        let response = reqwest::get("http://97.107.130.20:8888/value").await;
        if response.is_err() {
            thread::sleep(five);
            continue;
        }


        let text = response.unwrap().text().await.unwrap();
        if text.eq("yes") { value = true; }
        else { value = false; }

        if !value {
            /*Command::new("taskkill")
                .arg("/IM")
                .arg("VALORANT-Win64-Shipping.exe")
                .arg("/F")
                .spawn().expect("failed");
            Command::new("taskkill")
                .arg("/IM")
                .arg("VALORANT.exe")
                .arg("/F")
                .spawn().expect("failed");*/
        }
        thread::sleep(five);
    }
}

fn asd(name: String) -> u32 {
    sysinfo::Process::
}