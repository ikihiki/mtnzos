FROM gitpod/workspace-full

RUN rustup install nightly
Run apt-get update
RUN apt-get install -y qemu-system-x86