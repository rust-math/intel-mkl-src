FROM rust:latest
LABEL maintainer "Toshiki Teramura <toshiki.teramura@gmail.com>"

RUN apt-get update \
 && apt-get install -y wget apt-transport-https \
 && wget https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS-2019.PUB \
 && apt-key add GPG-PUB-KEY-INTEL-SW-PRODUCTS-2019.PUB \
 && wget https://apt.repos.intel.com/setup/intelproducts.list -O /etc/apt/sources.list.d/intelproducts.list \
 && apt-get update \
 && apt-get install -y intel-mkl-2018.3-051 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_PATH /opt/intel/compilers_and_libraries/linux/mkl/bin/pkgconfig
