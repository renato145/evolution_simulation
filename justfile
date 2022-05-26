serve:
	@if ! [ -x "$(command -v http)" ]; then echo "Install http server: cargo install https"; exit 1; fi
	http web/

wasm-build:
	cd macroquad_version && cargo build --target wasm32-unknown-unknown --release
	cp macroquad_version/target/wasm32-unknown-unknown/release/evolution_simulation.wasm web/
