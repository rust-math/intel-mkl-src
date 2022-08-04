# Ubuntu 20.04 with apt

FROM ubuntu:20.04

# workaround for tzdata
ENV DEBIAN_FRONTEND=noninteractive

RUN apt update \
 && apt install -y \
      apt-utils \
      curl \
      gcc \
      gnupg \
      intel-mkl \
      libssl-dev \
      pkg-config \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

# Setup Rust
# From official setting in https://github.com/rust-lang/docker-rust
ARG RUST_VERSION
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain ${RUST_VERSION}

WORKDIR /src

# this may concern security issue for production use, but this container is designed for development use.
RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME /src

# Setup basic rust development tools
RUN cargo install cargo-tarpaulin
RUN rustup component add rustfmt clippy
