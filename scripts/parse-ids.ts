#!/usr/bin/env bun

const protoFile = Bun.file("all-in-one.proto");
if (!protoFile.exists()) {
    console.error("File not found: all-in-one.proto");
    process.exit(1);
}

const packetIds: { [key: string]: number } = {};

const content = await protoFile.text();
const lines = content.split("\n");

let nextId = -1;
for (const line of lines) {
    if (line.startsWith("message")) {
        if (nextId == -1) continue;

        packetIds[line.split(" ")[1]] = nextId;
        nextId = -1;
    }

    if (line.startsWith("// CmdId: ")) {
        nextId = parseInt(line.split(" ")[2]);
    }
}

const output = Bun.file("packet-ids.csv");
const sink = output.writer();

for (const [key, value] of Object.entries(packetIds)) {
    sink.write(`${key},${value}\n`);
}

sink.end();

export {};
