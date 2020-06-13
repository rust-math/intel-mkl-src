ARG RUST_VERSION
FROM rust:${RUST_VERSION}
ARG MKL_VERSION

WORKDIR /mkl

RUN apt update \
 && apt install -y cpio \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

COPY ./silent.cfg /mkl/
RUN curl -sfLO http://registrationcenter-download.intel.com/akdlm/irc_nas/tec/16533/l_mkl_${MKL_VERSION}.tgz \
 && tar xf l_mkl_${MKL_VERSION}.tgz \
 && cd l_mkl_${MKL_VERSION}         \
 && ./install.sh -s ../silent.cfg   \
 && rm -rf /mkl
ENV PKG_CONFIG_PATH /opt/intel/mkl/bin/pkgconfig
RUN sed -i "s/MKLROOT/prefix/g" ${PKG_CONFIG_PATH}/*.pc

WORKDIR /src
