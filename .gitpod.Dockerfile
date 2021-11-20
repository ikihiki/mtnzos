FROM gitpod/workspace-full

RUN rustup install nightly || true
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN sudo apt-get update
RUN sudo apt-get install -y qemu-system-x86
