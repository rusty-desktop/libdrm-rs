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

/// Structure representing connector.
#[derive(Clone)]
pub struct ModeInfo {
    mode_info: ffi::xf86drm_mode::drmModeModeInfo,
}

/// General methods
impl ModeInfo {
    /// `ModeInfo` constructor.
    pub fn new(mode_info: ffi::xf86drm_mode::drmModeModeInfo) -> Self {
        ModeInfo { mode_info: mode_info }
    }

    /// Returns pointer to raw C structure.
    pub fn as_ptr(&self) -> ffi::xf86drm_mode::drmModeModeInfoPtr {
        &self.mode_info
    }
}

/// Getters for original members
impl ModeInfo {
    #[inline]
    pub fn get_clock(&self) -> u32 {
        self.mode_info.clock
    }

    #[inline]
    pub fn get_hdisplay(&self) -> u16 {
        self.mode_info.hdisplay
    }

    #[inline]
    pub fn get_hsync_start(&self) -> u16 {
        self.mode_info.hsync_start
    }

    #[inline]
    pub fn get_hsync_end(&self) -> u16 {
        self.mode_info.hsync_end
    }

    #[inline]
    pub fn get_htotal(&self) -> u16 {
        self.mode_info.htotal
    }

    #[inline]
    pub fn get_hskew(&self) -> u16 {
        self.mode_info.hskew
    }

    #[inline]
    pub fn get_vdisplay(&self) -> u16 {
        self.mode_info.vdisplay
    }

    #[inline]
    pub fn get_vsync_start(&self) -> u16 {
        self.mode_info.vsync_start
    }

    #[inline]
    pub fn get_vsync_end(&self) -> u16 {
        self.mode_info.vsync_end
    }

    #[inline]
    pub fn get_vtotal(&self) -> u16 {
        self.mode_info.vtotal
    }

    #[inline]
    pub fn get_vscan(&self) -> u16 {
        self.mode_info.vscan
    }

    #[inline]
    pub fn get_vrefresh(&self) -> u32 {
        self.mode_info.vrefresh
    }

    #[inline]
    pub fn get_flags(&self) -> u32 {
        self.mode_info.flags
    }

    #[inline]
    pub fn get_mode_type(&self) -> u32 {
        self.mode_info.mode_type
    }
}

impl std::fmt::Debug for ModeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "ModeInfo {{ width: {}, height: {} }}",
               self.get_hdisplay(),
               self.get_vdisplay())
    }
}
