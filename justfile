default:
    @just --list

check:
    cargo check --all-features

test:
    cargo test --all-features

clippy:
    cargo clippy --all-features --all-targets -- -D warnings

fmt:
    cargo fmt

watch:
    bacon

package:
    cargo package --allow-dirty

publish-dry-run:
    cargo publish --dry-run --locked --allow-dirty

release-check:
    cargo fmt --check
    cargo test --all-features --locked
    cargo clippy --all-features --all-targets --locked -- -D warnings
    cargo package --allow-dirty --locked
    cargo publish --dry-run --locked --allow-dirty

gallery:
    cargo run --features axum --example axum_gallery
