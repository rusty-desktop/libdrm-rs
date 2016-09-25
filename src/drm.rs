// Copyright 2016 The libdrm-rs project developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::os::unix::io;

use ffi;

pub use event_handler::{EventContext, handle_event};

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
pub fn get_cap(fd: io::RawFd, cap: Capability) -> Result<u64, i32> {
    let mut value = 0;
    let result = unsafe { ffi::xf86drm::drmGetCap(fd, cap as u64, &mut value) };

    if result == 0 { Ok(value) } else { Err(result) }
}
