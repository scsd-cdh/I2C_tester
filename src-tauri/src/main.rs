// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serial::Device;
use std::sync::Mutex;

mod serial;

// Create a static mutable Device instance

const PORT_PATH: &str = "/dev/ttyACM0";
const MESSAGE1: &str = "first";
const MESSAGE2: &str = "second";
const MESSAGE3: &str = "third";
const MESSAGE4: &str = "fourth";

static DEVICE: Mutex<Option<Device>> = Mutex::new(None);

// Also in main.rs
fn main() {
    init_device();
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![
            my_custom_command,
            play_with_piko,
            write_to_pico
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

#[tauri::command]
fn play_with_piko() -> String {
    let mut device_lock = DEVICE.lock().unwrap();
    if let Some(ref mut device) = *device_lock {
        device.write(MESSAGE1);
        device.read();
        device.write(MESSAGE2);
        device.read();
        device.write(MESSAGE3);
        device.read();
        device.write(MESSAGE4);
        device.read();
    }
    "sus".to_string()
}

#[tauri::command]
fn write_to_pico(command: &str) -> String {
    print!("ATTEMP WRITE CAUTION");
    let mut device_lock = DEVICE.lock().unwrap();
    if let Some(ref mut device) = *device_lock {
        println!("writing: {command}");
        device.write(command);
        device.read();
    }
    "nothing yet".to_string()
}

#[tauri::command]
fn my_custom_command(message: &str) {
    println!("I'm trying to send to a serial, MESSAGE: {message}");
}
fn init_device() {
    let mut device_lock = DEVICE.lock().unwrap();
    if device_lock.is_none() {
        *device_lock = Some(Device::new(PORT_PATH.to_string(), 9600));
    }
}
