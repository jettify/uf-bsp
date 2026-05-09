# List available recipes
default:
  just --list

alias b := build

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
  cargo clippy -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf -- -D warnings
  cargo clippy -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf -- -D warnings

# Check board examples
check-examples:
  cargo check -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --examples
  cargo check -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --examples

# Lint source code with strict linter
pedantic:
  cargo clippy -- -W clippy::pedantic

# Run same testing commands as on CI server
ci: build lint check-examples
