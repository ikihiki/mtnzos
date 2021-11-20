FROM gitpod/workspace-full

RUN bash -cl "rustup toolchain install nightly　|| true"
Run apt-get update
RUN apt-get install -y qemu-system-x86