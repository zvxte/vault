# Vault

**Password and note manager**.

Built with [Axum](https://github.com/tokio-rs/axum) and [Tauri](https://github.com/tauri-apps/tauri) frameworks.
Provides end-to-end encryption with [AES-GCM](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm).
Uses [Argon2id](https://github.com/RustCrypto/password-hashes/tree/master/argon2) to hash master passwords.

See [images](./images) directory for quick view on desktop application.

## Setup

To compile the project, run:
```bash
cargo build --release
cargo tauri build
```
Binaries will be located in `target/release` directory.

Server to run requires:
  - `SERVER_URL` environment variable,
  - PostgreSQL database with `DATABASE_URL` environment variable.

On Linux:
```bash
export SERVER_URL="{address}:{port}"
export DATABASE_URL="postgres://{username}:{password}@{address}:{port}/{databaseName}"
```

## Todo

 - sessions expiration time
 - sessions management from client

## Disclaimer

This is a personal project created for learning purposes and is **not suitable** for real-world usage.

## License

This project is licensed under [MIT License](./LICENSE).
