#!/usr/bin/env bash
set -euxo pipefail

if ! command -v rustup >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
    export PATH="$HOME/.cargo/bin:$PATH"
fi

rustup toolchain install stable
rustup toolchain install nightly

rustup component add clippy rustfmt llvm-tools-preview --toolchain stable
rustup component add miri llvm-tools-preview rust-src --toolchain nightly
export PATH="$(rustc +nightly --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin:$PATH"
export PATH="$(rustc +stable --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin:$PATH"

cargo +stable  install --locked cargo-nextest cargo-edit
if ! command -v cargo-insta >/dev/null 2>&1; then
    curl -LsSf https://insta.rs/install.sh | sh
fi
go install github.com/rhysd/actionlint/cmd/actionlint@latest
cargo +nightly install --locked cargo-fuzz flamegraph

# Install Clang 19 and related packages for fuzzing
sudo apt-get update
sudo apt-get install -y wget gnupg lsb-release
wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | sudo tee /usr/share/keyrings/llvm.asc
echo "deb [signed-by=/usr/share/keyrings/llvm.asc] http://apt.llvm.org/$(lsb_release -cs)/ llvm-toolchain-$(lsb_release -cs)-19 main" | sudo tee /etc/apt/sources.list.d/llvm19.list
sudo apt-get update
sudo apt-get install -y clang-19 lldb-19 lld-19 llvm-19-dev
sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-19 100
sudo update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-19 100

# Install perf for profiling
sudo apt-get install -y linux-tools-common "linux-tools-$(uname -r)" \
  || sudo apt-get install -y linux-tools-generic
sudo bash -c 'echo 0 > /proc/sys/kernel/perf_event_paranoid' || true

echo "✅ Rust development environment ready."

chmod +x scripts/setup.sh
