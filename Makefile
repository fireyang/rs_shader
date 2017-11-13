
demo1: fmt
	cargo run --example demo1

run:
	cargo run
	# cargo run --release

install_fmt:
	cargo install rustfmt

fmt:
	cargo fmt -- --write-mode=overwrite

get_submodule:
	git submodule update --init --recursive
