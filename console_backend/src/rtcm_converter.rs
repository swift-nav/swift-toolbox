#[cfg(target_os = "windows")]
#[windows_subsystem = "console"]
extern "C" {}

use libc::{c_char, c_void};
use std::io::{Read, Write};

use gnss_converters::*;

#[link(name = "ubx2sbp", kind = "static")]
#[link(name = "gnss_converters", kind = "static")]
#[link(name = "swiftnav", kind = "static")]
#[link(name = "sbp", kind = "static")]
#[link(name = "ixcom", kind = "static")]
extern "C" {
    fn rtcm3tosbp(
        argc: i32,
        argv: *const *const i8,
        addition_opts_help: *const c_char,
        readfn: unsafe extern "C" fn(*mut u8, usize, *mut c_void) -> i32,
        writefn: unsafe extern "C" fn(*const u8, u32, *mut c_void) -> i32,
        context: *mut c_void,
    ) -> i32;
}

pub fn convert<R: Read + 'static, W: Write + 'static>(reader: R, writer: W) -> i32 {
    let mut context = Context::new(Box::new(reader), Box::new(writer));
    let cargs = CArgs::new();
    let argv = cargs.argv();
    let (argc, argv) = (cargs.len(), argv.as_ptr());
    unsafe {
        rtcm3tosbp(
            argc,
            argv,
            ADDITIONAL_OPTS_HELP.as_ptr(),
            readfn,
            writefn_u32,
            &mut context as *mut Context as *mut c_void,
        )
    }
}
