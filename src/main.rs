// MIT License
//
// Copyright (c) 2017 Toshiki Teramura
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate libc;

use libc::*;

extern "C" {
    pub fn LAPACKE_dsyev(matrix_layout: c_int,
                         jobz: c_char,
                         uplo: c_char,
                         n: c_int,
                         a: *mut c_double,
                         lda: c_int,
                         w: *mut c_double)
                         -> c_int;
}

pub const LAPACK_ROW_MAJOR: c_int = 101;
pub const LAPACK_COL_MAJOR: c_int = 102;

fn main() {
    let matrix_layout = LAPACK_COL_MAJOR;
    let jobz = b'V' as i8;
    let uplo = b'U' as i8;
    let n = 1;
    let mut a = vec![0.0];
    let lda = 1;
    let mut w = vec![0.0];

    unsafe {
        LAPACKE_dsyev(matrix_layout,
                      jobz,
                      uplo,
                      n,
                      a.as_mut_ptr(),
                      lda,
                      w.as_mut_ptr());
    }
}
