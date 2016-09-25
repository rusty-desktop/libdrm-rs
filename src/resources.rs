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

use ffi;
use encoder;
use connector;
use crtc;

/// Resources structure.
/// Can be obtained by call to `drm_mode::get_mode_resources`
pub struct Resources {
    resources: ffi::xf86drm_mode::drmModeResPtr,
}

/// General methods
impl Resources {
    /// `Resources` constructor.
    /// Does not check if passed arguments are valid.
    pub fn new(resources: ffi::xf86drm_mode::drmModeResPtr) -> Self {
        Resources { resources: resources }
    }
}

/// Getters for original members
impl Resources {
    /// Returns count of frame buffers.
    #[inline]
    pub fn get_count_fbs(&self) -> i32 {
        unsafe { (*self.resources).count_fbs }
    }

    /// Returns count of CRTCs
    #[inline]
    pub fn get_count_crtcs(&self) -> i32 {
        unsafe { (*self.resources).count_crtcs }
    }

    /// Return count of connectors.
    #[inline]
    pub fn get_count_connectors(&self) -> i32 {
        unsafe { (*self.resources).count_connectors }
    }

    /// Return count of encoders.
    #[inline]
    pub fn get_count_encoders(&self) -> i32 {
        unsafe { (*self.resources).count_encoders }
    }

    /// Return vector of CRTC ids.
    pub fn get_crtcs(&self) -> Vec<crtc::CrtcId> {
        let count = self.get_count_crtcs();
        let mut vec = Vec::with_capacity(count as usize);
        for pos in 0..count as isize {
            vec.push(unsafe { *(*self.resources).crtcs.offset(pos) });
        }
        vec
    }

    /// Return vector of encoder ids.
    pub fn get_encoders(&self) -> Vec<encoder::EncoderId> {
        let count = self.get_count_encoders();
        let mut vec = Vec::with_capacity(count as usize);
        for pos in 0..count as isize {
            vec.push(unsafe { *(*self.resources).encoders.offset(pos) });
        }
        vec
    }

    /// Return vector of connector ids.
    pub fn get_connectors(&self) -> Vec<connector::ConnectorId> {
        let count = self.get_count_connectors();
        let mut vec = Vec::with_capacity(count as usize);
        for pos in 0..count as isize {
            vec.push(unsafe { *(*self.resources).connectors.offset(pos) });
        }
        vec
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        unsafe { ffi::xf86drm_mode::drmModeFreeResources(self.resources) };
    }
}
