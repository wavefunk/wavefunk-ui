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

gallery:
    cargo run --features axum --example axum_gallery

vendor-design:
    cp ../design/css/wavefunk.css static/wavefunk/css/wavefunk.css
    cp ../design/css/01-tokens.css static/wavefunk/css/01-tokens.css
    cp ../design/css/02-base.css static/wavefunk/css/02-base.css
    cp ../design/css/03-layout.css static/wavefunk/css/03-layout.css
    cp ../design/css/04-components.css static/wavefunk/css/04-components.css
    cp ../design/css/05-utilities.css static/wavefunk/css/05-utilities.css
    cp ../design/css/06-marketing.css static/wavefunk/css/06-marketing.css
    cp ../design/css/fonts/MartianGrotesk-VF.woff2 static/wavefunk/css/fonts/
    cp ../design/css/fonts/MartianMono-VF.woff2 static/wavefunk/css/fonts/
