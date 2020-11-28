install_rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- --profile minimal -y
	source $HOME/.cargo/env && cargo install wasm-pack
	cd hello-wasm && wasm-pack build