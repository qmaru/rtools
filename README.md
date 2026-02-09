# rtools

[![rtools](https://github.com/qmaru/rtools/workflows/Publish%20Package/badge.svg)](https://github.com/qmaru/rtools/actions)

some rust wasm tools

## List

Below is a concise, function-first feature map (exports are `pub` methods on wasm-visible structs):

- **hash** 🔐
  - `Hash::gen_md5`, `gen_sha256`, `gen_sha3_256`, `gen_sm3`
  - `gen_blake2s256`, `gen_blake2b512`, `gen_blake3`
  - Murmur: `gen_murmur32`, `gen_murmur128`
  - HKDF: `gen_hkdf`, `gen_hkdf_b64`
  - HMAC: `gen_hmac_sha256` / `verify_hmac_sha256`, `gen_hmac_blake2b512` / `verify_hmac_blake2b512`
  - Ed25519 keypair & signing: `gen_ed25519_keypair`, `gen_ed25519_keypair_b64`, `gen_ed25519_sign`, `verify_ed25519_sign`

- **otp** ⏱️
  - `OTPAuth::generate_code(secret, timestamp, period)` — TOTP generator (6-digit)

- **parse** 🔁
  - Base64 / Base32 / Base64URL variants: `encode64`, `decode64`, `encode64_nopad`, `decode64_nopad`, etc.
  - Base62 helpers: `encode62`, `decode62` (and bytes variants)
  - Hex: `encode_hex`, `decode_hex`
  - Punycode / IDNA helpers: `encode_punycode`, `decode_punycode`
  - SQIDs: `encode` / `decode` (SQID encode/decode helpers)

- **password** 🔑
  - `Password` struct (stateful; holds `OsRng`) with `get_random_password(...)`

- **pinyin** 🀄
  - `Pinyin::han_to_pinyin`, `han_to_pinyin_with_tone`

- **qrcode** 🔳
  - Generation: `QRCode::raw*` (packed/unpacked), `raw_auto*`, and `text*` renderers
  - Decoding: `QRCode::decode_luma(gray_bytes, width, height)` accepts greyscale bytes (0-255) and returns decoded contents

- **random** 🎲
  - UUID: `UUID::v4()`, `UUID::v7(ms)`, `UUID::v7_now()`
  - NanoID: `NanoID::generate(length, alphabet)`, `generate_auto()`
  - `SafeBytes` helpers: `gen_bytes`, `gen_secret_16_bytes`, `gen_secret_20_bytes`, `gen_secret_32_bytes`, `gen_nonce_*` and serialization helpers `to_base62`, `to_base32`, `to_base64`, `to_hex`, etc.

- **secret** 🔐
  - Authenticated encryption helpers (base64 inputs/outputs): `Secret::xchacha_encrypt/decrypt`, `Secret::chacha_encrypt/decrypt`

Notes & patterns:

- All wasm-visible APIs use a zero-sized struct + `#[wasm_bindgen] impl` pattern, except a few stateful structs (e.g., `Password`).
- Tests live as `#[test]` functions at the bottom of each feature file — run with `cargo test -- --nocapture` to see prints.
- Prefer `Result<T, JsValue>` for fallible wasm exports; some older functions still use `Option` or `Result<String, String>` (see `secret.rs` and parts of `parse.rs`).

## Install

```shell
cargo install wasm-pack
```

## Test

```shell
cargo test -- --nocapture
```

## Build

```shell
# Build wasm
wasm-pack build -t web

# Build npm pkg
# --scope YourName
wasm-pack build -t bundler --release --scope qmaru
```

## Publish

```shell
wasm-pack publish
```

## Dependencies

[hashes](https://github.com/RustCrypto/hashes)

[murmur3](https://github.com/stusmall/murmur3)

[BLAKE3](https://github.com/BLAKE3-team/BLAKE3)

[zxcvbn](https://github.com/shssoichiro/zxcvbn-rs)

[pinyin](https://github.com/mozillazg/rust-pinyin)
