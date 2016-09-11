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

use std;

use ffi;

/// Type of CRTC id.
pub type CrtcId = u32;

/// Structure representing CRTC.
pub struct Crtc {
    crtc: ffi::xf86drm_mode::drmModeCrtcPtr,
}

// General methods
impl Crtc {
    /// `Crtc` constructor.
    /// Does not check if passed arguments are valid.
    pub fn new(crtc: ffi::xf86drm_mode::drmModeCrtcPtr) -> Self {
        Crtc { crtc: crtc }
    }
}

// Getters for original members
impl Crtc {
    #[inline]
    pub fn get_crtc_id(&self) -> CrtcId {
        unsafe { (*self.crtc).crtc_id }
    }

    #[inline]
    pub fn get_buffer_id(&self) -> u32 {
        unsafe { (*self.crtc).buffer_id }
    }

    /// Get X-axis position on the frame buffer.
    #[inline]
    pub fn get_x(&self) -> u32 {
        unsafe { (*self.crtc).x }
    }

    /// Get Y-axis position on the frame buffer.
    #[inline]
    pub fn get_y(&self) -> u32 {
        unsafe { (*self.crtc).y }
    }

    #[inline]
    pub fn get_width(&self) -> u32 {
        unsafe { (*self.crtc).width }
    }

    #[inline]
    pub fn get_height(&self) -> u32 {
        unsafe { (*self.crtc).height }
    }

    #[inline]
    pub fn get_mode_valid(&self) -> bool {
        unsafe { (*self.crtc).mode_valid != 0 }
    }

    #[inline]
    pub fn get_gamma_size(&self) -> isize {
        unsafe { (*self.crtc).gamma_size as isize }
    }
}

impl Drop for Crtc {
    fn drop(&mut self) {
        unsafe { ffi::xf86drm_mode::drmModeFreeCrtc(self.crtc) };
    }
}

impl std::fmt::Debug for Crtc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Crtc {{ id: {}, buffer_id: {} }}", self.get_crtc_id(), self.get_buffer_id())
    }
}

