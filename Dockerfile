### Base image <https://hub.docker.com/_/ubuntu>
ARG BASE_IMAGE_NAME="ubuntu"
ARG BASE_IMAGE_TAG="jammy"
FROM ${BASE_IMAGE_NAME}:${BASE_IMAGE_TAG}

### Use bash as the default shell
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

### Create a barebones entrypoint that is conditionally updated throughout the Dockerfile
RUN echo "#!/usr/bin/env bash" >> /entrypoint.bash && \
    chmod +x /entrypoint.bash

### Build OpenUSD
ARG OPENUSD_VERSION="22.11"
COPY ./pxr_sys/patches/src/build_scripts/build_usd.py.patch /tmp/build_usd.py.patch
# hadolint ignore=SC2016
RUN OPENUSD_DL_PATH="/tmp/OpenUSD-${OPENUSD_VERSION}.tar.gz" && \
    OPENUSD_SRC_DIR="/tmp/OpenUSD-${OPENUSD_VERSION}" && \
    OPENUSD_INSTALL_DIR="${HOME}/openusd" && \
    echo -e "\n# OpenUSD ${OPENUSD_VERSION}" >> /entrypoint.bash && \
    echo "export OPENUSD_PATH=\"${OPENUSD_INSTALL_DIR}\"" >> /entrypoint.bash && \
    echo '# export PATH="${OPENUSD_PATH}/bin${PATH:+:${PATH}}"' >> /entrypoint.bash && \
    echo '# export LD_LIBRARY_PATH="${OPENUSD_PATH}/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"' >> /entrypoint.bash && \
    echo '# export PYTHONPATH="${OPENUSD_PATH}/lib/python${PYTHONPATH:+:${PYTHONPATH}}"' >> /entrypoint.bash && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yq --no-install-recommends \
    build-essential \
    ca-certificates \
    clang \
    cmake \
    curl \
    libarchive-dev \
    libgl-dev \
    libglfw3-dev \
    libglib2.0-dev \
    libglu-dev \
    libglu1-mesa-dev \
    libilmbase-dev \
    libssl-dev \
    libtbb2-dev \
    libx11-dev \
    libxt-dev \
    pkg-config \
    python3-dev \
    python3-pip && \
    rm -rf /var/lib/apt/lists/* && \
    python3 -m pip install --no-cache-dir PyOpenGL==3.1.7 pyside6==6.6.0 && \
    curl --proto "=https" --tlsv1.2 -sSfL "https://github.com/PixarAnimationStudios/OpenUSD/archive/refs/tags/v${OPENUSD_VERSION}.tar.gz" -o "${OPENUSD_DL_PATH}" && \
    mkdir -p "${OPENUSD_SRC_DIR}" && \
    tar xf "${OPENUSD_DL_PATH}" -C "${OPENUSD_SRC_DIR}" --strip-components=1 && \
    rm "${OPENUSD_DL_PATH}" && \
    if [[ "${OPENUSD_VERSION}" = "22.11" ]]; then \
    patch --unified --strip=1 --batch --follow-symlinks --ignore-whitespace --input=/tmp/build_usd.py.patch --directory="${OPENUSD_SRC_DIR}" ; \
    fi && \
    rm /tmp/build_usd.py.patch && \
    python3 "${OPENUSD_SRC_DIR}/build_scripts/build_usd.py" \
    --build-shared \
    --build-variant=release --prefer-speed-over-safety \
    --use-cxx11-abi=0 \
    --build-args USD,"-DPXR_LIB_PREFIX=lib" \
    --no-tests --no-examples --no-tutorials --no-tools --no-docs \
    --usdview \
    --python --no-debug-python \
    --usd-imaging \
    --no-ptex \
    --openvdb \
    --no-embree \
    --no-prman \
    --no-openimageio \
    --no-opencolorio \
    --alembic \
    --no-hdf5 \
    --draco \
    --materialx \
    "${OPENUSD_INSTALL_DIR}" && \
    rm -rf "${OPENUSD_SRC_DIR}" "${OPENUSD_INSTALL_DIR}/src"

### Install Rust
ARG RUST_VERSION="1.75"
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yq --no-install-recommends \
    ca-certificates \
    curl \
    mold && \
    rm -rf /var/lib/apt/lists/* && \
    curl --proto "=https" --tlsv1.2 -sSfL "https://sh.rustup.rs" | sh -s -- --no-modify-path -y --default-toolchain "${RUST_VERSION}" --profile default && \
    echo -e "\n# Rust ${RUST_VERSION}" >> /entrypoint.bash && \
    echo "source \"${HOME}/.cargo/env\" --" >> /entrypoint.bash && \
    echo "export CARGO_TARGET_DIR=\"${HOME}/ws_target\"" >> /entrypoint.bash && \
    echo "export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS=\"-Clink-arg=-fuse-ld=mold -Ctarget-cpu=native\"" >> /entrypoint.bash

### Finalize the entrypoint and source it in the ~/.bashrc
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

### Copy the source
COPY . "${WORKSPACE}"

### Build the project
RUN source /entrypoint.bash -- && \
    cargo build --release --all-targets

### Set the default command
CMD ["bash"]
