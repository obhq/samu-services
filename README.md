# PS4 SAMU Services

This is a PS4 payload to run on a jailbroken PS4. It provide a set of REST API for invoking SAMU on the PS4.

## Building from source

### Prerequisites

- Rust on the latest stable channel.

### Install additional Rust target

```sh
rustup target add x86_64-unknown-none
```

### Install additional Cargo commands

```sh
cargo install cargo-binutils
```

`cargo-binutils` required additional dependency which can be installed with the following command:

```sh
rustup component add llvm-tools-preview
```

### Build

```sh
cargo objcopy --release -- -O binary samu-services.bin
```

The above command will output `samu-services.bin`, which is a payload you can send to the jailbroken PS4.

## License

MIT
