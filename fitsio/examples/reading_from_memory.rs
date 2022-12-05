// `fitsio` does not currently support opening files from memory, `cfitsio` _does_. This means we
// can use `Fitsfile::from_raw` to load a `FitsFile` from a file that was opened via
// `fits_open_memfile` in `cfitsio`.

use fitsio::{FileOpenMode, FitsFile};
#[cfg(feature = "default")]
use fitsio_sys as sys;
#[cfg(feature = "bindgen")]
use fitsio_sys_bindgen as sys;
use std::io::Read;

fn main() {
    // read the bytes into memory and return a pointer and length to the file
    let (bytes, mut ptr_size) = {
        let filename = "../testdata/full_example.fits";
        let mut f = std::fs::File::open(filename).unwrap();
        let mut bytes = Vec::new();
        let num_bytes = f.read_to_end(&mut bytes).unwrap();

        (bytes, num_bytes as u64)
    };

    let mut ptr = bytes.as_ptr();

    // now we have a pointer to the data, let's open this in `fitsio_sys`
    let mut fptr = std::ptr::null_mut();
    let mut status = 0;

    let c_filename = std::ffi::CString::new("full_example.fits").unwrap();
    unsafe {
        sys::ffomem(
            &mut fptr as *mut *mut _,
            c_filename.as_ptr(),
            sys::READONLY as _,
            &mut ptr as *const _ as *mut *mut libc::c_void,
            &mut ptr_size as *mut u64,
            0,
            None,
            &mut status,
        );
    }

    if status != 0 {
        unsafe { sys::ffrprt(sys::stderr, status) };
        panic!("bad status");
    }

    let mut f =
        unsafe { FitsFile::from_raw("full_example.fits", fptr, FileOpenMode::READONLY) }.unwrap();
    f.pretty_print().expect("pretty printing fits file");
}
