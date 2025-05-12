all:
	@echo "please run 'make <target>'"

build_release:
	cargo build --release

fmt:
	cargo fmt

lint:
	cargo check

fix:
	cargo fix --lib -p dora