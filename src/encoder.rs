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
use crtc;

/// Type of encoder id.
pub type EncoderId = u32;

/// Type of encoder type.
pub type EncoderType = u32;

/// Structure representing encoder.
pub struct Encoder {
    encoder: ffi::xf86drm_mode::drmModeEncoderPtr,
}

/// General methods
impl Encoder {
    /// `Encoder` constructor.
    /// Does not check if passed arguments are valid.
    pub fn new(encoder: ffi::xf86drm_mode::drmModeEncoderPtr) -> Self {
        Encoder { encoder: encoder }
    }
}

/// Getters for original members
impl Encoder {
    #[inline]
    pub fn get_encoder_id(&self) -> EncoderId {
        unsafe { (*self.encoder).encoder_id }
    }

    #[inline]
    pub fn get_encoder_type(&self) -> EncoderType {
        unsafe { (*self.encoder).encoder_type }
    }

    #[inline]
    pub fn get_crtc_id(&self) -> crtc::CrtcId {
        unsafe { (*self.encoder).crtc_id }
    }

    #[inline]
    pub fn get_possible_crtcs(&self) -> u32 {
        unsafe { (*self.encoder).possible_crtcs }
    }

    #[inline]
    pub fn get_possible_clones(&self) -> u32 {
        unsafe { (*self.encoder).possible_clones }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { ffi::xf86drm_mode::drmModeFreeEncoder(self.encoder) };
    }
}

impl std::fmt::Debug for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Encoder {{ id: {}, crtc_id: {} }}", self.get_encoder_id(), self.get_crtc_id())
    }
}
