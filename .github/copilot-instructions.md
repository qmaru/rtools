# Copilot Instructions for rtools

## Project Overview

**rtools** is a Rust library compiled to **WebAssembly (WASM)** via `wasm-pack`, published as npm package `@qmaru/rtools`. It exposes cryptographic, encoding, and utility functions to JavaScript/TypeScript consumers. There is no native Rust binary — the sole compilation target is `wasm32-unknown-unknown`.

## Architecture

```
src/lib.rs          → single module declaration (mod tools)
src/tools/mod.rs    → re-exports all feature modules
src/tools/<feature>.rs → one file per feature domain
pkg/                → wasm-pack build output (npm package)
```

Each feature file (`hash.rs`, `secret.rs`, `qrcode.rs`, etc.) follows the same pattern:
- A **zero-sized marker struct** annotated with `#[wasm_bindgen]` acts as a namespace (e.g., `pub struct Hash {}`)
- All public methods live inside `#[wasm_bindgen] impl <Struct> { ... }` blocks
- Private helper methods (no `pub`) stay in the same `impl` block but are NOT exported to JS
- Functions return `Result<T, JsValue>` for fallible operations or plain types for infallible ones

Example (from `hash.rs`):
```rust
#[wasm_bindgen]
pub struct Hash {}

#[wasm_bindgen]
impl Hash {
    fn generate_hash<D: Digest>(message: &str) -> String { /* private helper */ }
    pub fn gen_md5(message: &str) -> String { /* exported to JS */ }
}
```

## Build & Test Commands

```shell
# Run tests (always use --nocapture so println! output is visible)
cargo test -- --nocapture

# Build for browser (ES module)
wasm-pack build -t web

# Build for npm/bundler (release)
wasm-pack build -t bundler --release --scope qmaru
```

- Crate type is `cdylib` only — no `rlib`, no binary targets.
- Release profile uses `lto = true` and `opt-level = 'z'` (size-optimized).

## Conventions

### Dependencies
- All crates use `default-features = false` where possible to minimize WASM binary size.
- `getrandom` must have the `"js"` feature for WASM random support.
- Avoid adding crates that depend on `std::fs`, `std::net`, or other OS-specific APIs.

### Error Handling
- Use `Result<T, JsValue>` for public WASM-exported functions that can fail.
- Convert errors with `.map_err(|e| JsValue::from_str(&format!("...: {}", e)))`.
- Some older functions return `Option<String>` — prefer `Result<T, JsValue>` for new code.

### Tests
- Tests are written as `#[test]` functions at the bottom of each feature file (not in a `tests/` directory).
- Tests call the public `struct` methods directly (e.g., `Hash::gen_md5("hello")`).
- Do NOT use `#[cfg(test)] mod tests { ... }` wrapper — tests are flat at module level.

### Adding a New Feature Module
1. Create `src/tools/<feature>.rs`
2. Add `pub mod <feature>;` to `src/tools/mod.rs`
3. Define a `#[wasm_bindgen] pub struct` and `#[wasm_bindgen] impl` block
4. Add `#[test]` functions at the bottom of the file
5. Update `README.md` feature list

### Documentation
- Every public struct and method has a `///` doc comment.
- Doc comments for methods start with the method name in backtick-quoted format: `` /// `method_name` description ``
