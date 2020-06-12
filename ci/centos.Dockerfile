FROM centos:7
ARG RUST_VERSION

# Setup Intel MKL
COPY install-yum.sh /
RUN /install-yum.sh
ENV PKG_CONFIG_PATH /opt/intel/compilers_and_libraries/linux/mkl/bin/pkgconfig
ENV LD_LIBRARY_PATH /opt/intel/compilers_and_libraries/linux/mkl/lib/intel64

# Setup Rust
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain ${RUST_VERSION}

WORKDIR /src

RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME /src
