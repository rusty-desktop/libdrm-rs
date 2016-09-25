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
use ffi::xf86drm_mode::drmModeConnection;
use encoder;
use mode_info;

/// Type of connector id.
pub type ConnectorId = u32;

/// Type of connector type.
pub type ConnectorType = u32;

/// Type of connector type id.
pub type ConnectorTypeId = u32;


static TYPE_NAMES: [&'static str; 15] = ["Unknown", "VGA", "DVII", "DVID", "DVIA", "Composite",
                                         "SVIDEO", "LVDS", "Component", "9PinDIN", "DisplayPort",
                                         "HDMIA", "HDMIB", "TV", "eDP"];

/// Enum representing state of connectors connection.
#[derive(PartialEq)]
pub enum Connection {
    Connected,
    Disconnected,
    Unknown,
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match *self {
                Connection::Connected => "connected",
                Connection::Disconnected => "disconnected",
                Connection::Unknown => "unknown",
            })
    }
}

/// Structure representing connector.
pub struct Connector {
    connector: ffi::xf86drm_mode::drmModeConnectorPtr,
}

/// General methods
impl Connector {
    /// `Connector` constructor.
    /// Does not check if passed arguments are valid.
    pub fn new(connector: ffi::xf86drm_mode::drmModeConnectorPtr) -> Self {
        Connector { connector: connector }
    }

    /// Get string representation of connector type.
    pub fn get_type_name(&self) -> &'static str {
        let connector_type = self.get_connector_type() as usize;
        TYPE_NAMES[if connector_type < TYPE_NAMES.len() { connector_type } else { 0 }]
    }
}

/// Getters for original members
impl Connector {
    #[inline]
    pub fn get_connector_id(&self) -> ConnectorId {
        unsafe { (*self.connector).connector_id }
    }

    /// Get id of encoder currently connected to.
    #[inline]
    pub fn get_encoder_id(&self) -> u32 {
        unsafe { (*self.connector).encoder_id }
    }

    #[inline]
    pub fn get_connector_type(&self) -> ConnectorType {
        unsafe { (*self.connector).connector_type }
    }

    #[inline]
    pub fn get_connector_type_id(&self) -> ConnectorTypeId {
        unsafe { (*self.connector).connector_type_id }
    }

    #[inline]
    pub fn get_connection(&self) -> Connection {
        unsafe {
            match (*self.connector).connection {
                drmModeConnection::DRM_MODE_CONNECTED => Connection::Connected,
                drmModeConnection::DRM_MODE_DISCONNECTED => Connection::Disconnected,
                _ => Connection::Unknown,
            }
        }
    }

    /// Get width in millimeters.
    #[inline]
    pub fn get_mm_width(&self) -> u32 {
        unsafe { (*self.connector).mmWidth }
    }

    /// Get height in millimeters.
    #[inline]
    pub fn get_mm_height(&self) -> u32 {
        unsafe { (*self.connector).mmHeight }
    }

    /// Get count of modes.
    #[inline]
    pub fn get_count_modes(&self) -> i32 {
        unsafe { (*self.connector).count_modes }
    }

    /// Get count of encoders.
    #[inline]
    pub fn get_count_encoders(&self) -> i32 {
        unsafe { (*self.connector).count_encoders }
    }

    /// Return vector of modes.
    pub fn get_modes(&self) -> Vec<mode_info::ModeInfo> {
        let count = self.get_count_modes();
        let mut vec = Vec::with_capacity(count as usize);
        for pos in 0..count as isize {
            vec.push(mode_info::ModeInfo::new(unsafe {
                (*(*self.connector).modes.offset(pos)).clone()
            }));
        }
        vec
    }

    /// Return vector of encoder ids.
    pub fn get_encoders(&self) -> Vec<encoder::EncoderId> {
        let count = self.get_count_encoders();
        let mut vec = Vec::with_capacity(count as usize);
        for pos in 0..count as isize {
            vec.push(unsafe { *(*self.connector).encoders.offset(pos) });
        }
        vec
    }
}

impl Drop for Connector {
    fn drop(&mut self) {
        unsafe { ffi::xf86drm_mode::drmModeFreeConnector(self.connector) };
    }
}

impl std::fmt::Debug for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Connector {{ id: {}, encoder_id: {}, state: {:?}, type: {} }}",
               self.get_connector_id(),
               self.get_encoder_id(),
               self.get_connection(),
               self.get_type_name())
    }
}

