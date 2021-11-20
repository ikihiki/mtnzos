FROM gitpod/workspace-full

RUN RUN bash -cl "rustup toolchain install nightly"
Run apt-get update
RUN apt-get install -y qemu-system-x86