
[<img alt="crates.io" src="https://img.shields.io/crates/v/lux-types.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/lux-types)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-lux_types-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/lux-types)
![Github Actions](https://github.com/luxfi/lux-rs/actions/workflows/test-and-release.yml/badge.svg)

## LuxGo Compatibility

| Crate Version(s) | LuxGo Version(s) | Protocol Version |
| ---------------- | ---------------------- | ---------------- |
| v0.0.134-155     | v1.9.2,v1.9.3          | 19               |
| v0.0.156-176     | v1.9.4                 | 20               |
| v0.0.177-200     | v1.9.5                 | 21               |
| v0.0.201-271     | v1.9.6,v1.9.7          | 22               |
| v0.0.272-291     | v1.9.8,v1.9.9          | 23               |
| v0.0.292-335     | v1.9.10,v1.9.16        | 24               |
| v0.0.336-390     | v1.10.0                | 25               |
| v0.0.391+        | v1.10.1+               | 26               |

## Introduction

The `lux-types` crate implements and is the canonical representation of Lux primitive types in Rust.  Lux types are separated by modules and are all under the `src` directory.

This crate also provides an SDK library for developing subnets in Rust. For the SDK functionality, see `src/subnet` which contains everything required to build a subnet VM in Rust.

The following VMs were built with the SDK:
* Simple Rust VM: [TimestampVM](https://github.com/luxfi/timestampvm-rs)
* Complex Rust VM: [SpacesVM](https://github.com/luxfi/spacesvm-rs)

## Getting Started

Examples can be found in [`examples`](./examples) and is a good first step to getting an understanding of general usage.

### Tutorials

- [How to Build a Simple Rust VM](https://docs.lux.network/subnets/create-a-simple-rust-vm) tutorial provides a basic example of using the Rust SDK.

### Rust Version

`lux-types` currently works on Rust `1.67` and above as it requires support for the 2021 edition. This project uses the stable toolchain.

## Getting Help

First please try find the answer to your question in the code documentation. If more clarification is required, try opening an [issue] with the question.

[issue]: https://github.com/luxfi/lux-rs/issues/new

## Features

- Ids (e.g., [`src/ids`](./src/ids))
- Transaction types/serialization (e.g., [`src/platformvm/txs`](./src/platformvm/txs))
- Certificates (e.g., [`src/key/cert`](./src/key/cert))
- Keys and addresses (e.g., [`src/key/secp256k1`](./src/key/secp256k1))
- Peer-to-peer messages (e.g., [`src/message`](./src/message))
- RPC chain VM (e.g., [`src/subnet/rpc`](./src/subnet/rpc))
- Genesis generate helper (e.g., [`src/subnet_evm`](./src/subnet_evm))
- Protobuf generated stubs and helpers (e.g., [`src/proto`](./src/proto))

The basic types available in this crate are used in other Lux Rust projects (e.g., distributed load tester [`blizzard`](https://talks.gyuho.dev/distributed-load-generator-lux-2022.html), [`lux-ops`](https://github.com/luxfi/lux-ops)).

## License

This project is licensed under the [BSD 3](LICENSE).
