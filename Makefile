
demo1: fmt
	cargo run --example demo1

run: 
	cargo run

install_fmt:
	cargo install rustfmt

fmt:
	cargo fmt -- --write-mode=overwrite
