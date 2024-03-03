use registry::{Data, Hive, Security};
use crate::options::Options;
use crate::system::install_location;
use crate::utils::file_exists;

#[cfg(windows)]
static INTERNET_SETTINGS: &str = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";

#[cfg(windows)]
pub fn snowflake_path() -> String {
    let path = format!("{}\\snowflake.dll", install_location());

    // Check if the snowflake DLL exists.
    if !file_exists(&path) {
        // Copy the snowflake DLL to the installation directory.
        let bytes = include_bytes!("../resources/snowflake.dll");
        std::fs::write(&path, bytes).unwrap();
    }

    path
}

#[cfg(linux)]
pub fn snowflake_path() -> String {
    "".to_string()
}

#[cfg(macos)]
pub fn snowflake_path() -> String {
    "".to_string()
}

#[cfg(windows)]
pub fn enable_proxy(options: &Options) {
    println!("Enabling proxy...");

    // Prepare the proxy string.
    let server = format!(
        "http=127.0.0.1:{};https=127.0.0.1:{}",
        options.proxy.port, options.proxy.port
    ).parse().unwrap();

    // Apply changes to the registry.
    let internet_settings = Hive::CurrentUser
        .open(INTERNET_SETTINGS, Security::AllAccess)
        .unwrap();
    internet_settings.set_value("ProxyServer", &Data::String(server)).unwrap();
    internet_settings.set_value("ProxyEnable", &Data::U32(1)).unwrap();
}

#[cfg(linux)]
pub fn enable_proxy(options: &Options) {
    println!("Enabling proxy...");
}

#[cfg(macos)]
pub fn enable_proxy(options: &Options) {
    println!("Enabling proxy...");
}

#[cfg(windows)]
pub fn disable_proxy() {
    println!("Disabling proxy...");

    // Apply changes to the registry.
    let internet_settings = Hive::CurrentUser
        .open(INTERNET_SETTINGS, Security::AllAccess)
        .unwrap();
    internet_settings.set_value("ProxyEnable", &Data::U32(0)).unwrap();
}

#[cfg(linux)]
pub fn disable_proxy() {
    println!("Disabling proxy...");
}

#[cfg(macos)]
pub fn disable_proxy() {
    println!("Disabling proxy...");
}
