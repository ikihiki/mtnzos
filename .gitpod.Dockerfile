FROM gitpod/workspace-full

RUN bash -cl "rustup toolchain install nightly　|| true"
RUN sudo apt-get update
RUN sudo apt-get install -y qemu-system-x86
