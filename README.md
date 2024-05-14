# Grasscutter Universe

A mono-repo for Magix's version of Grasscutter (and additional projects).

# Contents

- grasscutter (gameserver)
- universe (gateserver)
- magic-gc (sdk)
- cultivation (launcher)
- ayylmao (packet sniffer)
- snowflake (runtime patcher)

## Other Content

- kcp - A fork of the KCP protocol, reimplemented for Rust and modified for miHoYo's proprietary protocol.
- meadow-server - The frontend for the SDK server.
- rs-common - Common code for Rust-based projects.
- scripts - A collection of scripts.

# Dictionary

- `gameserver` - A server which handles the game's packets.
- `gateserver` - A server which clients request a gameserver IP address from.
- `sdk` - A general purpose server which handles user authentication, analytics, and client configuration.
- `launcher` - Client utility responsible for local account management, game patching, traffic proxying, etc.
- `packet sniffer` - A tool for analyzing and deobfuscating game packes over the wire.
- `runtime patcher` - A tool for modifying game behavior at runtime. (usually for RSA key patching)
