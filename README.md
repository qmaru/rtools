# rtools

some rust wasm tools

## List

+ Blake2b512
+ Blake2s256
+ Blake3
+ Md5
+ Murmur3_32
+ Murmur3_x64_128
+ Sha256
+ Sha3_256
+ Sm3
+ password strength
+ pinyin
+ uuid

## Install

```shell
cargo install wasm-pack
```

## Build

```shell
# Build wasm
wasm-pack build --target web

# Build npm pkg
wasm-pack build --target bundler
cd pkg
npm link
```

## Publish

npm publish

## Dependencies

[hashes](https://github.com/RustCrypto/hashes)

[murmur3](https://github.com/stusmall/murmur3)

[BLAKE3](https://github.com/BLAKE3-team/BLAKE3)

[zxcvbn](https://github.com/shssoichiro/zxcvbn-rs)
