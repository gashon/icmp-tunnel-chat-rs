# ICMP Tunnel Chat

A peer-to-peer chat application that tunnels messages through ICMP echo packets (ping protocol).

## Overview

ICMP Tunnel Chat enables covert communication by encoding messages inside ICMP echo request/reply packets. 

## Features

- Communication over ICMP
- Message fragmentation and reassembly for larger messages
- Bidirectional communication

## Installation

### Prerequisites

- Rust and Cargo (1.56.0 or later)
- Administrator/root privileges (required for raw socket access)

### Building from Source

```bash
git clone https://github.com/gashon/icmp-tunnel-chat-rs.git
cd icmp-tunnel-chat-rs
cargo build --release
```

The compiled binary will be available at `target/release/p2p_icmp_chat`.

## Usage

Run the application with root/administrator privileges:

```bash
sudo ./target/release/p2p_icmp_chat <destination_ip>
```

### Example

```bash
# Terminal 1 (on machine A)
sudo ./target/release/p2p_icmp_chat 192.168.1.2

# Terminal 2 (on machine B)
sudo ./target/release/p2p_icmp_chat 192.168.1.1
```

Type your messages and press Enter to send. Messages from the other party will appear in your terminal as they arrive.

## How It Works

1. The application creates a raw socket to send and receive ICMP packets
2. Messages are fragmented into chunks that fit within the payload of ICMP packets
3. Each fragment contains metadata (message ID, fragment ID) for reassembly
4. Received fragments are reassembled into complete messages

- ICMP traffic is **not encrypted** by default in this implementation
- Network administrators can detect and block ICMP tunneling

<!-- Analytics  -->
![](https://analytics-fawn-nine.vercel.app/api/analytics/github/beacon?api_key=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhdXRob3JfaWQiOiJlOWJhM2U0ZC0yOTI4LTQxZTYtOTQ2ZS1lNTAwZWUyNzRkYTciLCJwcm9qZWN0X2lkIjoiMzk3OTM2NGUtNjA0Yi00YWU2LTkxNWUtZGIyYjk3MDEwYjQ1IiwiY3JlYXRlZF9hdCI6IjIwMjQtMDEtMTBUMDM6NTQ6MzQuMzU0WiIsImlhdCI6MTcwNDg1ODg3NH0.k_tddRmgKImJ8ROqgNHUAXiW_BP_FlZFdlx8-VYPyh8)
