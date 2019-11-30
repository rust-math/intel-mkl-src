#!/bin/bash
# Intel MKL installer for ubuntu/debian
set -eux

apt-get update
apt-get install -y wget apt-transport-https
curl -sfL https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS-2019.PUB | apt-key add -
curl -sfL https://apt.repos.intel.com/setup/intelproducts.list -o /etc/apt/sources.list.d/intelproducts.list
apt-get update
apt-get install -y intel-mkl-core-rt-2019.5  # Please update version number
