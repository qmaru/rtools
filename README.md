# rtools

[![rtools](https://github.com/qmaru/rtools/workflows/wasm/badge.svg)](https://github.com/qmaru/rtools/actions)

some rust wasm tools

## List

+ hash
  + MD5
  + SHA-256
  + SHA3-256
  + SM3
  + BLAKE2s
  + BLAKE2b
  + BLAKE3
  + MurmurHash3-32
  + MurmurHash3-128
  + hkdf

+ otp
  + totp

+ parse
  + encoding / decoding
    + base64
    + base32
    + punycode
  + sqids

+ password

+ pinyin
  + han_to_pinyin
  + han_to_pinyin_with_tone

+ qrcode

+ random
  + uuid
    + v4
    + v7
  + nanoid
  + safebytes

+ secret
  + xchacha
  + chacha

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
