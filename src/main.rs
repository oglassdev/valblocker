#![windows_subsystem = "windows"]

use std::{thread, time};
use sysinfo::{PidExt, ProcessExt, SystemExt};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_TERMINATE, TerminateProcess};

#[tokio::main]
async fn main() {
    let mut system = sysinfo::System::new();
    system.refresh_all();

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
            let ps = system.processes().iter().filter(|(_, p)| p.name().starts_with("VALORANT"));
            for (pid, _p) in ps {
                //println!("{}:{}", pid, p.name());
                kill_process(pid.as_u32());
            }
        }
        system.refresh_processes();
        thread::sleep(five);
    }
}

fn kill_process(pid: u32) {
    unsafe {
        let handy = OpenProcess(PROCESS_TERMINATE, true, pid).unwrap();
        TerminateProcess(handy, 0);
    }
}