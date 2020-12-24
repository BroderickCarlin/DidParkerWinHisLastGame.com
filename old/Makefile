install_rust:
	rustup default stable
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build_gol:
	cd wasm-game-of-life; wasm-pack build
	cd wasm-game-of-life; npm install

build: install_rust build_gol