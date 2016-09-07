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

#[repr(C)]
pub struct drm_mode_create_dumb {
    pub height: u32,
    pub width: u32,
    pub bpp: u32,
    pub flags: u32,
    /* handle, pitch, size will be returned */
    pub handle: u32,
    pub pitch: u32,
    pub size: u64
}
impl ::std::default::Default for drm_mode_create_dumb {
    fn default() -> drm_mode_create_dumb { unsafe { ::std::mem::zeroed() } }
}

/* set up for mmap of a dumb scanout buffer */
#[repr(C)]
pub struct drm_mode_map_dumb {
    /** Handle for the object being mapped. */
    pub handle: u32,
    pub pad: u32,
    /**
    * Fake offset to use for subsequent mmap call
    *
    * This is a fixed-size type for 32/64 compatibility.
    */
    pub offset: u64
}
impl ::std::default::Default for drm_mode_map_dumb {
    fn default() -> drm_mode_map_dumb { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
pub struct drm_mode_destroy_dumb {
    pub handle: u32
}
impl ::std::default::Default for drm_mode_destroy_dumb {
    fn default() -> drm_mode_destroy_dumb { unsafe { ::std::mem::zeroed() } }
}
