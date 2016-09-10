use std::ffi::CString;
use libc::c_char;
use std::os::unix::io;

use ffi;
pub use resources::Resources;

/// Checks if mode setting for device describer by `busid` is supported.
///
/// Counterpart for `drmCheckModesettingSupported`.
pub fn check_modesetting_supported(busid: &str) -> Result<(), i32> {
    let busid_c_str = CString::new(busid.as_bytes()).unwrap();
    let busid_ptr: *const c_char = busid_c_str.as_bytes_with_nul().as_ptr() as *const _;

    let result = unsafe { ffi::xf86drm_mode::drmCheckModesettingSupported(busid_ptr) };

    if result == 0 { Ok(()) } else { Err(result) }
}

/// Returns device resources.
///
/// Counterpart for `drmModeGetResources`.
pub fn get_resources(fd: &io::RawFd) -> Option<Resources> {
    let resources = unsafe { ffi::xf86drm_mode::drmModeGetResources(*fd) };
    if resources.is_null() {
        None
    } else {
        Some(Resources { resources: resources })
    }
}
