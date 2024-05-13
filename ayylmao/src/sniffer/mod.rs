pub mod capturer;
pub mod processor;
pub mod bruteforce;

use std::{sync::{Arc, Mutex}, thread};

use pcap::Device;

use crate::options::Options;

use self::capturer::Capturer;

/// Fetches the device specified in the configuration.
/// options: The configuration options.
pub fn get_device(options: &Options) -> Device {
    let devices = Device::list().unwrap();
    devices[options.device as usize].clone()
}

/// Captures all game packets from the specified device.
/// device: The device to capture packets from.
pub async fn capture(device: Device) {
    let hook = Arc::new(Mutex::new(false));

    // Create and start the packet capturer.
    let mut capturer = Capturer::new(hook.clone());
    thread::spawn(move || {
        capturer.run(device);
    });

    // Wait for the Ctrl + C signal.
    tokio::signal::ctrl_c().await.unwrap();
    // Shutdown the packet capturer.
    *hook.lock().unwrap() = true;
}