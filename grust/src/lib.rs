#[allow(non_snake_case)]
#[allow(unused_variables)]

mod logger;
mod server;
mod session;

use std::sync::Mutex;
use jni::{JNIEnv, objects::JObject};
use jni::objects::{JByteArray, JString};
use jni::sys::JNI_VERSION_1_6;
use lazy_static::lazy_static;
use log::info;
use tokio::runtime::Runtime;
use crate::server::Server;

lazy_static! {
    static ref TOKIO_RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new().unwrap());
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    _env: JNIEnv,
    _: *mut std::ffi::c_void
) -> i32 {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    info!("Rust library initialized.");

    JNI_VERSION_1_6
}

#[no_mangle]
pub extern "system" fn Java_io_grasscutter_net_impl_NetworkTransportImpl_listen(
    mut env: JNIEnv,
    _: JObject,
    address: JString,
    port: i32
) {
    let address = env.get_string(&address)
        .expect("failed to get address")
        .to_str().unwrap().to_string();

    let runtime = TOKIO_RUNTIME.lock().unwrap();
    runtime.block_on(async {
        let mut server = Server::new(&address, port as u16).await.unwrap();
        server.listen().await.unwrap();
    });
}

#[no_mangle]
pub extern "system" fn Java_io_grasscutter_net_impl_NetworkTransportImpl_shutdown0(
    mut env: JNIEnv,
    object: JObject
) {
    info!("Shutting down Rust native network transport...");
}

#[no_mangle]
pub extern "system" fn Java_io_grasscutter_net_impl_NetworkTransportImpl_send(
    mut env: JNIEnv,
    object: JObject,
    conv_id: i64,
    data: JByteArray
) {
    info!("Sending data to conv_id: {}", conv_id);
}

#[no_mangle]
pub extern "system" fn Java_io_grasscutter_net_impl_KcpSessionImpl_close(
    mut env: JNIEnv,
    object: JObject,
    conv_id: i64
) {
    info!("Closing KCP session...");
}
