# ayylmao

Anime game packet capture utility

# Usage

ayylmao has two modes:
1. Sniffer mode
2. Deobfuscation mode

## Sniffer mode

Sniffer mode opens a websocket server for Iridium (or similar tools) to connect to.\
Packets are merely captured, nothing else happens to them.

## Deobfuscation mode

Deobfuscation mode captures all packets and attempts to deobfuscate them.

# Troubleshooting

## Running on Windows

If you are encountering `STATUS_DLL_NOT_FOUND` on Windows, copy:
- `C:\Windows\System32\Npcap\Packet.dll`
- `C:\Windows\System32\Npcap\wpcap.dll`

to the same directory as `ayylmao.exe`.