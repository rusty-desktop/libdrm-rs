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

use std::ffi::CString;
use libc::c_char;
use std::os::unix::io;

use ffi;
pub use resources::Resources;
pub use crtc::{Crtc, CrtcId};
pub use connector::{Connector, ConnectorId, Connection, ConnectorType, ConnectorTypeId};
pub use encoder::{Encoder, EncoderId, EncoderType};
pub use mode_info::ModeInfo;

pub const PAGE_FLIP_EVENT: u32 = 0x01;
pub const PAGE_FLIP_ASYNC: u32 = 0x02;

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
/// Allocated resources are automatically dropped.
///
/// Counterpart for `drmModeGetResources`.
pub fn get_resources(fd: io::RawFd) -> Option<Resources> {
    let resources = unsafe { ffi::xf86drm_mode::drmModeGetResources(fd) };
    if resources.is_null() {
        None
    } else {
        Some(Resources::new(resources))
    }
}

/// Return CRTC.
///
/// Allocated resources are automatically dropped.
///
/// Counterpart for `drmModeGetCrtc`.
pub fn get_crtc(fd: io::RawFd, id: CrtcId) -> Option<Crtc> {
    let crtc = unsafe { ffi::xf86drm_mode::drmModeGetCrtc(fd, id) };
    if crtc.is_null() {
        None
    } else {
        Some(Crtc::new(crtc))
    }
}

/// Return Connector.
///
/// Allocated resources are automatically dropped.
///
/// Counterpart for `drmModeGetConnector`.
pub fn get_connector(fd: io::RawFd, id: ConnectorId) -> Option<Connector> {
    let connector = unsafe { ffi::xf86drm_mode::drmModeGetConnector(fd, id) };
    if connector.is_null() {
        None
    } else {
        Some(Connector::new(connector))
    }
}

/// Return Encoder.
///
/// Allocated resources are automatically dropped.
///
/// Counterpart for `drmModeGetEncoder`.
pub fn get_encoder(fd: io::RawFd, id: EncoderId) -> Option<Encoder> {
    let encoder = unsafe { ffi::xf86drm_mode::drmModeGetEncoder(fd, id) };
    if encoder.is_null() {
        None
    } else {
        Some(Encoder::new(encoder))
    }
}

/// Creates a new framebuffer with an buffer object as its scanout buffer.
///
/// Returns newly created buffers ID on success or error code in case if failure.
///
/// Counterpart of `drmModeAddFB`.
pub fn add_fb(fd: io::RawFd,
              width: u32,
              height: u32,
              depth: u8,
              bpp: u8,
              pitch: u32,
              bo_handle: u32)
              -> Result<u32, i32> {
    let mut buffer_id: u32 = 0;
    let result = unsafe {
        ffi::xf86drm_mode::drmModeAddFB(fd,
                                        width,
                                        height,
                                        depth,
                                        bpp,
                                        pitch,
                                        bo_handle,
                                        &mut buffer_id)
    };
    if result == 0 {
        Ok(buffer_id)
    } else {
        Err(result)
    }
}

/// Set the mode on a crtc `crtc_id` with the given mode.
///
/// Counterpart of `drmModeSetCrtc`.
pub fn set_crtc(fd: io::RawFd,
                crtc_id: u32,
                buffer_id: u32,
                x: u32,
                y: u32,
                connectors: &[ConnectorId],
                mode: &ModeInfo)
                -> Result<(), i32> {
    let result = unsafe {
        ffi::xf86drm_mode::drmModeSetCrtc(fd,
                                          crtc_id,
                                          buffer_id,
                                          x,
                                          y,
                                          connectors.as_ptr(),
                                          connectors.len() as i32,
                                          mode.as_ptr())
    };
    if result == 0 { Ok(()) } else { Err(result) }
}

/// Perform page flip.
///
/// Counterpart of `drmModePageFlip`. `drmModePageFlip` takes pointer to arbitrary data which is
/// then passed to callback defined by `drmEventContext`. Passing here any data other than simple
/// integers or static constants would be unsafe. Instead this function takes integer that can be
/// used as key in map to assign it to more complex data.
pub fn page_flip(fd: io::RawFd,
                 crtc_id: u32,
                 fb_id: u32,
                 flags: u32,
                 user_data: i32)
                 -> Result<(), i32> {
    let result = unsafe {
        ffi::xf86drm_mode::drmModePageFlip(fd, crtc_id, fb_id, flags, user_data as *const _)
    };
    if result == 0 { Ok(()) } else { Err(result) }
}
