export AWS_ACCESS_KEY_ID = awsdummy
export AWS_SECRET_ACCESS_KEY = awsdummy

.PHONY: dynamo
dynamo:
	- docker compose down --volumes
	docker compose up -d --wait
	deno run --allow-net=localhost:8000 --allow-env --allow-read --allow-sys --no-check ./setup/setup.ts

.PHONY: test
test:
	make dynamo
	cargo test -- --test-threads=1

.PHONY: lint
lint:
	cargo clippy --all-targets -- -D warnings
	cargo clippy --all-targets --no-default-features --features rustls -- -D warnings
	cargo clippy --all-targets --features tracing -- -D warnings

.PHONY: check-deps
check-deps:
	cargo machete
	cargo +nightly udeps --all-targets
	cargo +nightly udeps --all-targets --no-default-features --features rustls
	cargo +nightly udeps --all-targets --features tracing

.PHONY: licenses
licenses:
	cargo bundle-licenses --format toml --output THIRDPARTY.toml

.PHONY: check-licenses
check-licenses:
	RUST_LOG=error cargo bundle-licenses --format toml --output __CHECK --previous THIRDPARTY.toml --check-previous
	rm __CHECK || true
