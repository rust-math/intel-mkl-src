#!/bin/bash
set -eux

if [ ! -d ${MKL_DIR:=/opt/intel/mkl} ]; then
  echo >&2 "MKL_DIR=${MKL_DIR} does not exists"
fi

# Get Intel MKL build version from mkl_version.h header file
MKL_VERSION_STRING=$(grep INTEL_MKL_VERSION ${MKL_DIR}/include/mkl_version.h | awk '{ print $3 }')
MKL_VERSION_MAJOR=${MKL_VERSION_STRING:0:4}
MKL_VERSION_MINOR=$((${MKL_VERSION_STRING:4}))  # Remove zeros `0005` -> `5`

STATIC_ARCHIVE="${PWD}/mkl_linux_static_${MKL_VERSION_MAJOR}_${MKL_VERSION_MINOR}.tar.xz"
SHARED_ARCHIVE="${PWD}/mkl_linux_shared_${MKL_VERSION_MAJOR}_${MKL_VERSION_MINOR}.tar.xz"

cd ${MKL_DIR}/lib/intel64
tar c --no-same-owner --no-same-permissions -Ipixz -f ${STATIC_ARCHIVE} *.a
tar c --no-same-owner --no-same-permissions -Ipixz -f ${SHARED_ARCHIVE} *.so
cd -
