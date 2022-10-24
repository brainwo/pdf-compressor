default:
    just cli-run

cli-run:
    cargo run --release

wasm-build:
    wasm-bindgen
