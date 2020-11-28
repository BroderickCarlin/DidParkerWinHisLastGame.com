install_rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- --profile minimal -y
	export PATH="$(HOME)/.cargo/bin:$PATH" && cargo install wasm-pack
	export PATH="$(HOME)/.cargo/bin:$PATH" && cd hello-wasm && wasm-pack build