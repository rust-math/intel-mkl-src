#!/bin/bash
set -eux

yum-config-manager --add-repo https://yum.repos.intel.com/mkl/setup/intel-mkl.repo
rpm --import https://yum.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS-2019.PUB
yum install -y intel-mkl-2020.0-088 gcc

# Add prefix into pkg-config settings
find /opt/intel -name "mkl-*.pc" -exec sed -i "1s#^#prefix=/opt/intel/mkl\n#" {};
