all:
	cargo run ./sample/b.parquet

build_release:
	cargo build --release

fmt:
	cargo format

lint:
	cargo check

fix:
	cargo fix --lib -p dora