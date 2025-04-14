fix:
	cargo fix --allow-dirty --allow-staged || exit 1
	cargo clippy --fix --no-deps --allow-dirty --allow-staged || exit 2
	cargo +nightly fmt || exit 3

check:
	cargo clippy --no-deps --all-targets -- -Dwarnings || exit 1
	cargo +nightly fmt --check || exit 2
	cargo test || exit 3