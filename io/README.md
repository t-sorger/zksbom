# Run with

`RUST_LOG=info cargo run -- upload --sbom /path/to/sbom.json`

`RUST_LOG=info cargo run -- get-commitment --product my_product --version 1.0.0`

`RUST_LOG=info cargo run -- get-zk-proof --api-key my_api_key --commitment abc123 --vulnerability CVE-2021-34527`

`RUST_LOG=info cargo run -- verify --commitment abc123 --zkproof xyz456`

