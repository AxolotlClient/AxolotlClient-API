let
	rust_overlay = import (builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/master.tar.gz);
in
	with import <nixpkgs> { overlays = [ rust_overlay ]; };
	
	mkShell {
		buildInputs = [
			openssl
			pkg-config
			rust-bin.stable.latest.default
		];
	}
