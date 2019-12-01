FROM rust:latest
LABEL maintainer "Toshiki Teramura <toshiki.teramura@gmail.com>"

COPY install.sh /
RUN /install.sh && apt-get clean && rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_PATH /opt/intel/compilers_and_libraries/linux/mkl/bin/pkgconfig
ENV LD_LIBRARY_PATH /opt/intel/compilers_and_libraries/linux/mkl/lib/intel64

WORKDIR /src
