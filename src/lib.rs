pub mod scoutnet;

use std::os::raw::*;
use std::ffi::CString;
/* libscoutnet wrapper for C */
#[repr(C)]
pub struct cscoutnet_res {
    pub service: *mut c_char,
	pub size: c_int,
    pub status: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn c_scoutnet_scanport(addr: *mut c_char, port: c_int, res: *mut cscoutnet_res) {
	let (serv, status) = scoutnet::scan_port(CString::from_raw(addr).into_string().unwrap(), port as i32, false);
	std::ptr::copy_nonoverlapping(CString::new(serv).unwrap().into_raw(), (*res).service, (*res).size.try_into().unwrap()); 

	if status {
		(*res).status = 0;
	} else {
		(*res).status = 1;
	}
}
