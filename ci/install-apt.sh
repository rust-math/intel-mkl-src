#!/bin/bash
# Based on https://software.intel.com/content/www/us/en/develop/articles/installing-intel-free-libs-and-python-apt-repo.html
set -eux

apt-get update
apt-get install -y curl apt-transport-https gnupg pkg-config libssl-dev gcc

# Setup repository
curl -sfL https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS-2019.PUB | apt-key add -
curl -sfL https://apt.repos.intel.com/setup/intelproducts.list -o /etc/apt/sources.list.d/intelproducts.list

# Install MKL
apt-get update
apt-get install -y intel-mkl-core-rt-2020.1  # Please update version number

# Cleanup apt-cache
apt-get clean
rm -rf /var/lib/apt/lists/*
