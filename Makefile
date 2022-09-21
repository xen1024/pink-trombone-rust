json:
	cargo run --example pink-trombone

schema:
	cargo run --example pink-trombone --features jsonse

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all
	
clean:
	cargo clean
