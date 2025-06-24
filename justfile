default: build
    @just serve

build:
    wasm-pack build -d web/pkg --target web --locked --release --no-default-features --features wasm

serve:
    live-server web

