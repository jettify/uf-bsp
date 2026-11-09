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
  cargo clippy -p uf-bsp-core --all-targets --all-features -- -D warnings
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf -- -D warnings
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --all-features --examples -- -D warnings
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf -- -D warnings
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --all-features --examples -- -D warnings

# Run host-side unit tests
test:
  cargo test -p uf-bsp-core

# Check board examples
check-examples:
  cargo check --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --all-features --examples
  cargo check --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --all-features --examples

# Lint source code with strict linter
pedantic:
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf -- -W clippy::pedantic
  cargo clippy --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --all-features --examples -- -W clippy::pedantic
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf -- -W clippy::pedantic
  cargo clippy --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --all-features --examples -- -W clippy::pedantic

# Link examples to catch duplicate or missing interrupt vectors
build-examples:
  cargo build --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --all-features --examples
  cargo build --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --all-features --examples

# Run same testing commands as on CI server
ci: test self-check-test build lint check-examples build-examples self-check-build

screen:
 screen /dev/tty.usbmodem14101 115200

# Build and flash self_check over DFU; board: lucid-h7 or speedybee-f405-v4
self-check board:
  #!/usr/bin/env bash
  set -euo pipefail
  case {{quote(board)}} in
    lucid-h7)          crate="tbs-lucid-h7-bsp" ;;
    speedybee-f405-v4) crate="speedybee-f405-v4-bsp" ;;
    *) echo "Unknown board: {{board}} (expected lucid-h7 or speedybee-f405-v4)" >&2; exit 1 ;;
  esac
  target="thumbv7em-none-eabihf"
  target_dir="${CARGO_TARGET_DIR:-target}/self-check/{{board}}"
  elf="${target_dir}/${target}/release/examples/self_check"
  bin="${target_dir}/${target}/release/examples/${crate}-self-check.bin"

  cargo build --release -p "$crate" --target "$target" --target-dir "$target_dir" \
    --example self_check --features self-test
  rust-objcopy -O binary "$elf" "$bin"

  serial_args=()
  if [[ -n "${DFU_SERIAL:-}" ]]; then
    serial_args=(--serial "$DFU_SERIAL")
  fi
  dfu-util --device 0483:df11 "${serial_args[@]}" --alt 0 \
    --dfuse-address 0x08000000:leave --download "$bin"

# Host tests for self-test module.
self-check-test:
  cargo test -p uf-bsp-core --features self-test

# Verify the baseline separately, without enabling sensor driver features.
self-check-build:
  #!/usr/bin/env bash
  set -euo pipefail
  for board in tbs-lucid-h7-bsp speedybee-f405-v4-bsp; do
    cargo build --release -p "$board" --target thumbv7em-none-eabihf --example self_check --features self-test
    cargo clippy --release -p "$board" --target thumbv7em-none-eabihf --features self-test --example self_check -- -D warnings
  done
