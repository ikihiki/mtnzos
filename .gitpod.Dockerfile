FROM gitpod/workspace-full

RUN bash -cl "rustup toolchain install nightly　|| true"
Run sudo apt-get update
RUN sudo apt-get install -y qemu-system-x86
