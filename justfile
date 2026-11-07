# List available recipes
default:
  just --list

alias b := build
alias f := fmt
alias l := lint

# Format source code with cargo fmt
fmt:
  cargo +nightly fmt --all

# Run cargo build
build:
  cargo build --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf
  cargo build --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf

# Run cargo clean
clean:
  cargo clean --verbose

# Lint source code CI linter
lint:
  cargo clippy -p uf-bsp-core --all-targets -- -D warnings
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf -- -D warnings
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --examples -- -D warnings
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf -- -D warnings
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --examples -- -D warnings

# Run host-side unit tests
test:
  cargo test -p uf-bsp-core

# Check board examples
check-examples:
  cargo check --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --examples
  cargo check --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --examples

# Lint source code with strict linter
pedantic:
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf -- -W clippy::pedantic
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --examples -- -W clippy::pedantic
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf -- -W clippy::pedantic
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --examples -- -W clippy::pedantic

# Link examples to catch duplicate or missing interrupt vectors
build-examples:
  cargo build --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --examples
  cargo build --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --examples

# Run same testing commands as on CI server
ci: test build lint check-examples build-examples

screen:
 screen /dev/tty.usbmodem14101 115200
