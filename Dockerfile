ARG RUST_VERSION=1.74
FROM rust:${RUST_VERSION}

### Use bash as the default shell
SHELL ["/bin/bash", "-c"]

### Create a barebones entrypoint that is conditionally updated throughout the Dockerfile
RUN echo "#!/usr/bin/env bash" >> /entrypoint.bash && \
    chmod +x /entrypoint.bash
## Configure Rust
ARG RUST_VERSION="1.74"
RUN echo -e "\n# Rust ${RUST_VERSION}" >> /entrypoint.bash && \
    echo "export CARGO_TARGET_DIR=\"${HOME}/ws_target\"" >> /entrypoint.bash && \
    echo "export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS=\"-Clink-arg=-fuse-ld=mold -Ctarget-cpu=native\"" >> /entrypoint.bash
## Finalize the entrypoint and source it in the ~/.bashrc
# hadolint ignore=SC2016
RUN echo -e "\n# Execute the command" >> /entrypoint.bash && \
    echo -en 'exec "${@}"\n' >> /entrypoint.bash && \
    echo -e "\n# Source the entrypoint" >> "${HOME}/.bashrc" && \
    echo -en "source /entrypoint.bash --\n" >> "${HOME}/.bashrc"
ENTRYPOINT ["/entrypoint.bash"]

### Configure the workspace
ARG WORKSPACE="/root/ws"
ENV WORKSPACE="${WORKSPACE}"
WORKDIR ${WORKSPACE}

### Install dependencies
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yq --no-install-recommends \
    build-essential \
    clang \
    cmake \
    libarchive-dev \
    libgl-dev \
    libglu-dev \
    libilmbase-dev \
    libssl-dev \
    libx11-dev \
    libxt-dev \
    mold \
    pkg-config \
    python3-dev && \
    rm -rf /var/lib/apt/lists/*

### Copy the source
COPY . "${WORKSPACE}"

### Build the project
RUN source /entrypoint.bash -- && \
    cargo build --release --all-targets

### Set the default command
CMD ["bash"]
