# Cultivation

General purpose launcher utility for Grasscutter.

# Features

- [x] Launch game
- [x] Proxy game traffic
  - [x] Automatically enable/disable proxy
    - [ ] macOS support
    - [x] Windows support
    - [ ] Linux support
- [ ] Inject multiple DLLs

# Building

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Visual Studio](https://visualstudio.microsoft.com/downloads/)
- [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk/)

## Process

1. Clone the _entire_ repository (specifically Snowflake & Cultivation)
2. Build the `snowflake` project
3. Copy the output of `snowflake.dll` to `cultivation/resources/snowflake.dll`
4. Run `cargo build --release`
