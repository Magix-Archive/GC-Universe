pub mod capturer;
pub mod processor;
pub mod bruteforce;

use std::{future::Future, sync::{Arc, Mutex}, thread};

use crossbeam_channel::Receiver;
use pcap::Device;

use crate::options::Options;

use self::{capturer::Capturer, processor::Packet};

/// Fetches the device specified in the configuration.
/// options: The configuration options.
pub fn get_device(options: &Options) -> Device {
    let devices = Device::list().unwrap();
    devices[options.device as usize].clone()
}

/// Captures all game packets from the specified device.
/// device: The device to capture packets from.
pub async fn capture<Fut: Future<Output = ()> + Send + 'static>(
    device: Device,
    options: Options,
    callback: Box<dyn Fn(
        Options,
        Arc<Mutex<bool>>,
        Receiver<Packet>
    ) -> Fut + Send>
) {
    let hook = Arc::new(Mutex::new(false));
    let (tx, rx) = crossbeam_channel::unbounded();

    // Create and start the packet capturer.
    let mut capturer = Capturer::new(hook.clone());
    thread::spawn(move || {
        capturer.run(device, tx);
    });

    // Start the packet handler.
    let shutdown_hook = hook.clone();
    tokio::spawn(callback(options, shutdown_hook, rx));

    // Wait for the Ctrl + C signal.
    tokio::signal::ctrl_c().await.unwrap();
    // Shutdown the packet capturer.
    *hook.lock().unwrap() = true;
}
