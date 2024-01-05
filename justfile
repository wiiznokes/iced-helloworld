set windows-powershell := true


all:
	clear ; cargo run

test:
	cargo test --workspace --all-features


fix: fmt
	cargo clippy --workspace --all-features --fix --allow-dirty --allow-staged

fmt:
	cargo fmt --all

expand:
	cargo expand


