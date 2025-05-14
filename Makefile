all:
	@echo "please run 'make <target>'"

run:
	# this lets the user select a file, then pipe the value into dora
	cargo run --bin dora-explorer | xargs cargo run --bin dora

build_release:
	cargo build --release

fmt:
	cargo fmt

lint:
	cargo check

fix:
	cargo fix --lib -p dora