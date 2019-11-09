// Link test

extern crate intel_mkl_src;
extern crate libc;

use libc::*;

extern "C" {
    fn LAPACKE_dsyev(
        matrix_layout: c_int,
        jobz: c_char,
        uplo: c_char,
        n: c_int,
        a: *mut c_double,
        lda: c_int,
        w: *mut c_double,
    ) -> c_int;
}

#[test]
fn link2lapacke() {
    let matrix_layout = 102;
    let jobz = b'V' as i8;
    let uplo = b'U' as i8;
    let n = 1;
    let mut a = vec![0.0];
    let lda = 1;
    let mut w = vec![0.0];
    unsafe {
        LAPACKE_dsyev(
            matrix_layout,
            jobz,
            uplo,
            n,
            a.as_mut_ptr(),
            lda,
            w.as_mut_ptr(),
        );
    }
}
