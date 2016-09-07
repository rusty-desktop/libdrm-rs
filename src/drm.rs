use std::os::unix::io;

use ffi;

#[repr(u64)]
pub enum Capability {
    DumbBuffer = ffi::drm::DRM_CAP_DUMB_BUFFER,
    VBlankHighCrtc = ffi::drm::DRM_CAP_VBLANK_HIGH_CRTC,
    DumbPreferredDepth = ffi::drm::DRM_CAP_DUMB_PREFERRED_DEPTH,
    DumbPreferShadow = ffi::drm::DRM_CAP_DUMB_PREFER_SHADOW,
    Prime = ffi::drm::DRM_CAP_PRIME,
    TimestampMonotonic = ffi::drm::DRM_CAP_TIMESTAMP_MONOTONIC,
    AsyncPageFlip = ffi::drm::DRM_CAP_ASYNC_PAGE_FLIP,
    CursorWidth = ffi::drm::DRM_CAP_CURSOR_WIDTH,
    CursorHeight = ffi::drm::DRM_CAP_CURSOR_HEIGHT,
    AddFB2Modifiers = ffi::drm::DRM_CAP_ADDFB2_MODIFIERS,
}

/// Get devices capability.
///
/// Counterpart for `drmGetCap`.
pub fn get_cap(fd: &io::RawFd, cap: Capability) -> Result<u64, i32> {
    let mut value = 0;
    let result = unsafe { ffi::xf86drm::drmGetCap(*fd, cap as u64, &mut value) };

    if result == 0 { Ok(value) } else { Err(result) }
}
