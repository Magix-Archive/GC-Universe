use std::sync::Arc;

use crossbeam_channel::Receiver;

use crate::{capture::processor::Packet, options::Options};

/// Packet handler for attemping to deobfuscate packets.
pub async fn run(app: Options, hook: Arc<std::sync::Mutex<bool>>, rx: Receiver<Packet>) {

}
