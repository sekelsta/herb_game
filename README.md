# Herb Game

## Developing

Install prerequisites:

- Cargo (Rust)
- pnpm 11
- `rustup target add wasm32-unknown-unknown`
- `cargo install wasm-bindgen-cli`

Run locally in terminal:

```
cargo run
```

Run locally in browser: In www folder,

```
pnpm dev
```

Build for browser: In www folder,

```
pnpm build
```

Note: You will need to run `pnpm wasm` before running `pnpm lint`.
