default:
    just cli-run

cli-run:
    cargo run --release

#  web

wasm-build:
    cd lib && wasm-pack build --target nodejs --out-dir=../lib-node
