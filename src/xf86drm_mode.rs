/*
 * Copyright (c) 2007-2008 Tungsten Graphics, Inc., Cedar Park, Texas.
 * Copyright (c) 2007-2008 Dave Airlie <airlied@linux.ie>
 * Copyright (c) 2007-2008 Jakob Bornecrantz <wallbraker@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

use libc::*;
use drm::*;
use drm_mode::*;

/*
 * This is the interface for modesetting for drm.
 *
 * In order to use this interface you must include either <stdint.h> or another
 * header defining uint32_t, int32_t and uint16_t.
 *
 * It aims to provide a randr1.2 compatible interface for modesettings in the
 * kernel, the interface is also ment to be used by libraries like EGL.
 *
 * More information can be found in randrproto.txt which can be found here:
 * http://gitweb.freedesktop.org/?p=xorg/proto/randrproto.git
 *
 * There are some major diffrences to be noted. Unlike the randr1.2 proto you
 * need to create the memory object of the framebuffer yourself with the ttm
 * buffer object interface. This object needs to be pinned.
 */

/* Scaling mode options */
pub const DRM_MODE_SCALE_NON_GPU: c_int = 0;
pub const DRM_MODE_SCALE_NO_SCALE: c_int = 2;

/* Dithering mode options */
pub const DRM_MODE_ENCODER_NONE: c_int = 0;
pub const DRM_MODE_ENCODER_DAC: c_int = 1;
pub const DRM_MODE_ENCODER_TMDS: c_int = 2;
pub const DRM_MODE_ENCODER_LVDS: c_int = 3;
pub const DRM_MODE_ENCODER_TVDAC: c_int = 4;
pub const DRM_MODE_ENCODER_VIRTUAL: c_int = 5;
pub const DRM_MODE_ENCODER_DSI: c_int = 6;

pub const DRM_MODE_SUBCONNECTOR_AUTOMATIC: c_int = 0;
pub const DRM_MODE_SUBCONNECTOR_UNKNOWN: c_int = 0;
pub const DRM_MODE_SUBCONNECTOR_DVID: c_int = 3;
pub const DRM_MODE_SUBCONNECTOR_DVIA: c_int = 4;
pub const DRM_MODE_SUBCONNECTOR_COMPOSITE: c_int = 5;
pub const DRM_MODE_SUBCONNECTOR_SVIDEO: c_int = 6;
pub const DRM_MODE_SUBCONNECTOR_COMPONENT: c_int = 8;
pub const DRM_MODE_SUBCONNECTOR_SCART: c_int = 9;

pub const DRM_MODE_CONNECTOR_UNKNOWN: c_int = 0;
pub const DRM_MODE_CONNECTOR_VGA: c_int = 1;
pub const DRM_MODE_CONNECTOR_DVII: c_int = 2;
pub const DRM_MODE_CONNECTOR_DVID: c_int = 3;
pub const DRM_MODE_CONNECTOR_DVIA: c_int = 4;
pub const DRM_MODE_CONNECTOR_COMPOSITE: c_int = 5;
pub const DRM_MODE_CONNECTOR_SVIDEO: c_int = 6;
pub const DRM_MODE_CONNECTOR_LVDS: c_int = 7;
pub const DRM_MODE_CONNECTOR_COMPONENT: c_int = 8;
pub const DRM_MODE_CONNECTOR_9PINDIN: c_int = 9;
pub const DRM_MODE_CONNECTOR_DISPLAYPORT: c_int = 10;
pub const DRM_MODE_CONNECTOR_HDMIA: c_int = 11;
pub const DRM_MODE_CONNECTOR_HDMIB: c_int = 12;
pub const DRM_MODE_CONNECTOR_TV: c_int = 13;
pub const DRM_MODE_CONNECTOR_EDP: c_int = 14;
pub const DRM_MODE_CONNECTOR_VIRTUAL: c_int = 15;
pub const DRM_MODE_CONNECTOR_DSI: c_int = 16;

pub const DRM_MODE_PROP_PENDING: c_int = (1<<0);
pub const DRM_MODE_PROP_RANGE: c_int = (1<<1);
pub const DRM_MODE_PROP_IMMUTABLE: c_int = (1<<2);
pub const DRM_MODE_PROP_ENUM: c_int = (1<<3) /* enumerated type with text strings */;
pub const DRM_MODE_PROP_BLOB: c_int = (1<<4);

pub const DRM_MODE_CURSOR_BO: c_int = (1<<0);
pub const DRM_MODE_CURSOR_MOVE: c_int = (1<<1);

/*
 * Feature defines
 *
 * Just because these are defined doesn't mean that the kernel
 * can do that feature, its just for new code vs old libdrm.
 */
pub const DRM_MODE_FEATURE_KMS: c_int = 1;
pub const DRM_MODE_FEATURE_DIRTYFB: c_int = 1;

#[repr(C)]
pub struct drmModeRes {
	count_fbs: c_int,
	fbs: *mut uint32_t,

	count_crtcs: c_int,
	crtcs: *mut uint32_t,

	count_connectors: c_int,
	connectors: *mut uint32_t,

	count_encoders: c_int,
	encoders: *mut uint32_t,

	min_width: uint32_t,
    max_width: uint32_t,
    min_height: uint32_t,
    max_height: uint32_t
}
impl ::std::default::Default for drmModeRes {
    fn default() -> drmModeRes { unsafe { ::std::mem::zeroed() } }
}

pub type drmModeResPtr = *mut drmModeRes;

#[repr(C)]
pub struct drmModeModeInfo {
	clock: uint32_t,
	hdisplay: uint16_t,
    hsync_start: uint16_t,
    hsync_end: uint16_t,
    htotal: uint16_t,
    hskew: uint16_t,
	vdisplay: uint16_t,
    vsync_start: uint16_t,
    vsync_end: uint16_t,
    vtotal: uint16_t,
    vscan: uint16_t,
	vrefresh: uint32_t,
	flags: uint32_t,
	mode_type: uint32_t,
	name: [c_char; DRM_DISPLAY_MODE_LEN as usize]
}
impl ::std::default::Default for drmModeModeInfo {
    fn default() -> drmModeModeInfo { unsafe { ::std::mem::zeroed() } }
}

pub type drmModeModeInfoPtr = *mut drmModeModeInfo;

#[repr(C)]
pub struct drmModeFB {
	fb_id: uint32_t,
	width: uint32_t,
    height: uint32_t,
	pitch: uint32_t,
	bpp: uint32_t,
	depth: uint32_t,
	/* driver specific handle */
	handle: uint32_t
}
impl ::std::default::Default for drmModeFB {
    fn default() -> drmModeFB { unsafe { ::std::mem::zeroed() } }
}

pub type drmModeFBPtr = *mut drmModeFB;

pub type drmModeClip = drm_clip_rect;
pub type drmModeClipPtr = *mut drmModeClip;

#[repr(C)]
pub struct drmModePropertyBlobRes {
	id: uint32_t,
	length: uint32_t,
	data: *mut c_void
}
impl ::std::default::Default for drmModePropertyBlobRes {
    fn default() -> drmModePropertyBlobRes { unsafe { ::std::mem::zeroed() } }
}

pub type drmModePropertyBlobPtr = *mut drmModePropertyBlobRes;

#[repr(C)]
pub struct drmModePropertyRes {
	prop_id: uint32_t,
	flags: uint32_t,
	name: [c_char; DRM_PROP_NAME_LEN as usize],
	count_values: c_int,
	values: uint64_t, /* store the blob lengths */
	count_enums: c_int,
	enums: *mut drm_mode_property_enum,
	count_blobs: c_int,
	blob_ids: *mut uint32_t
}
impl ::std::default::Default for drmModePropertyRes {
    fn default() -> drmModePropertyRes { unsafe { ::std::mem::zeroed() } }
}

pub type drmModePropertyPtr = *mut drmModePropertyRes;

// static __inline int drm_property_type_is(drmModePropertyPtr property,
// 		uint32_t type)
// {
// 	/* instanceof for props.. handles extended type vs original types: */
// 	if (property->flags & DRM_MODE_PROP_EXTENDED_TYPE)
// 		return (property->flags & DRM_MODE_PROP_EXTENDED_TYPE) == type;
// 	return property->flags & type;
// }

#[repr(C)]
pub struct drmModeCrtc {
	crtc_id: uint32_t,
	buffer_id: uint32_t, /**< FB id to connect to 0 = disconnect */

	x: uint32_t,
    y: uint32_t, /**< Position on the framebuffer */
	width: uint32_t,
    height: uint32_t,
	mode_valid: c_int,
	mode: drmModeModeInfo,
	gamma_size: c_int
}
impl ::std::default::Default for drmModeCrtc {
    fn default() -> drmModeCrtc { unsafe { ::std::mem::zeroed() } }
}

pub type drmModeCrtcPtr = *mut drmModeCrtc;

#[repr(C)]
pub struct drmModeEncoder {
	encoder_id: uint32_t,
	encoder_type: uint32_t,
	crtc_id: uint32_t,
	possible_crtcs: uint32_t,
	possible_clones: uint32_t
}
impl ::std::default::Default for drmModeEncoder {
    fn default() -> drmModeEncoder { unsafe { ::std::mem::zeroed() } }
}

pub type drmModeEncoderPtr = *mut drmModeEncoder;

#[repr(C)]
pub enum drmModeConnection {
	DRM_MODE_CONNECTED         = 1,
	DRM_MODE_DISCONNECTED      = 2,
	DRM_MODE_UNKNOWNCONNECTION = 3
}

#[repr(C)]
pub enum drmModeSubPixel {
	DRM_MODE_SUBPIXEL_UNKNOWN        = 1,
	DRM_MODE_SUBPIXEL_HORIZONTAL_RGB = 2,
	DRM_MODE_SUBPIXEL_HORIZONTAL_BGR = 3,
	DRM_MODE_SUBPIXEL_VERTICAL_RGB   = 4,
	DRM_MODE_SUBPIXEL_VERTICAL_BGR   = 5,
	DRM_MODE_SUBPIXEL_NONE           = 6
}

#[repr(C)]
pub struct drmModeConnector {
	connector_id: uint32_t,
	encoder_id: uint32_t, /**< Encoder currently connected to */
	connector_type: uint32_t,
	connector_type_id: uint32_t,
	connection: drmModeConnection,
	mmWidth: uint32_t,
    mmHeight: uint32_t, /**< HxW in millimeters */
	subpixel: drmModeSubPixel,

	count_modes: c_int,
	modes: drmModeModeInfoPtr,

	count_props: c_int,
	props: *mut uint32_t, /**< List of property ids */
	prop_values: *mut uint64_t, /**< List of property values */

	count_encoders: c_int,
	encoders: *mut uint32_t
}
impl ::std::default::Default for drmModeConnector {
    fn default() -> drmModeConnector { unsafe { ::std::mem::zeroed() } }
}

pub type drmModeConnectorPtr = *mut drmModeConnector;

pub const DRM_PLANE_TYPE_OVERLAY: c_int = 0;
pub const DRM_PLANE_TYPE_PRIMARY: c_int = 1;
pub const DRM_PLANE_TYPE_CURSOR: c_int = 2;

#[repr(C)]
pub struct drmModeObjectProperties {
	count_props: uint32_t,
	props: *mut uint32_t,
	prop_values: *mut uint64_t
}
impl ::std::default::Default for drmModeObjectProperties {
    fn default() -> drmModeObjectProperties { unsafe { ::std::mem::zeroed() } }
}

pub type drmModeObjectPropertiesPtr = *mut drmModeObjectProperties;

#[repr(C)]
pub struct drmModePlane {
	count_formats: uint32_t,
	formats: *mut uint32_t,
	plane_id: uint32_t,

	crtc_id: uint32_t,
	fb_id: uint32_t,

	crtc_x: uint32_t,
    crtc_y: uint32_t,
	x: uint32_t,
    y: uint32_t,

	possible_crtcs: uint32_t,
	gamma_size: uint32_t
}
impl ::std::default::Default for drmModePlane {
    fn default() -> drmModePlane { unsafe { ::std::mem::zeroed() } }
}

pub type drmModePlanePtr = *mut drmModePlane;

#[repr(C)]
pub struct drmModePlaneRes {
	count_planes: uint32_t,
	planes: *mut uint32_t
}
impl ::std::default::Default for drmModePlaneRes {
    fn default() -> drmModePlaneRes { unsafe { ::std::mem::zeroed() } }
}

pub type drmModePlaneResPtr = *mut drmModePlaneRes;

#[repr(C)]
pub struct _drmModeAtomicReqItem {
	object_id: uint32_t,
	property_id: uint32_t,
	value: uint64_t
}
impl ::std::default::Default for _drmModeAtomicReqItem {
    fn default() -> _drmModeAtomicReqItem { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct _drmModeAtomicReq {
	cursor: uint32_t,
	size_items: uint32_t,
	items: _drmModeAtomicReqItem
}
impl ::std::default::Default for _drmModeAtomicReq {
    fn default() -> _drmModeAtomicReq { unsafe { ::std::mem::zeroed() } }
}


pub type drmModeAtomicReq = _drmModeAtomicReq;
pub type drmModeAtomicReqPtr = *mut drmModeAtomicReq;

#[link(name = "drm")]
#[allow(dead_code)]
extern {
    pub fn drmModeFreeModeInfo(ptr: drmModeModeInfoPtr) -> c_void;
    pub fn drmModeFreeResources(ptr: drmModeResPtr) -> c_void;
    pub fn drmModeFreeFB(ptr: drmModeFBPtr) -> c_void;
    pub fn drmModeFreeCrtc(ptr: drmModeCrtcPtr) -> c_void;
    pub fn drmModeFreeConnector(ptr: drmModeConnectorPtr) -> c_void;
    pub fn drmModeFreeEncoder(ptr: drmModeEncoderPtr) -> c_void;
    pub fn drmModeFreePlane(ptr: drmModePlanePtr) -> c_void;
    pub fn drmModeFreePlaneResources(ptr: drmModePlaneResPtr) -> c_void;

    /**
    * Retrives all of the resources associated with a card.
    */
    pub fn drmModeGetResources(fd: c_int) -> drmModeResPtr;

    /*
    * FrameBuffer manipulation.
    */

    /**
    * Retrive information about framebuffer bufferId
    */
    pub fn drmModeGetFB(fd: c_int, bufferId: uint32_t) -> drmModeFBPtr;

    /**
    * Creates a new framebuffer with an buffer object as its scanout buffer.
    */
    pub fn drmModeAddFB(fd: c_int, width: uint32_t, height: uint32_t, depth: uint8_t,
        bpp: uint8_t, pitch: uint32_t, bo_handle: uint32_t,
        buf_id: *mut uint32_t) -> c_int;

    /* ...with a specific pixel format */
    pub fn drmModeAddFB2(fd: c_int, width: uint32_t, height: uint32_t,
        pixel_format: uint32_t, bo_handles: [uint32_t; 4],
        pitches: [uint32_t; 4], offsets: [uint32_t; 4],
        buf_id: *mut uint32_t, flags: uint32_t) -> c_int;

    /**
    * Destroies the given framebuffer.
    */
    pub fn drmModeRmFB(fd: c_int, bufferId: uint32_t) -> c_int;

    /**
    * Mark a region of a framebuffer as dirty.
    */
    pub fn drmModeDirtyFB(fd: c_int, bufferId: uint32_t,
        clips: drmModeClipPtr, num_clips: uint32_t) -> c_int;

    /*
    * Crtc functions
    */

    /**
    * Retrive information about the ctrt crtcId
    */
    pub fn drmModeGetCrtc(fd: c_int, crtcId: uint32_t) -> drmModeCrtcPtr;

    /**
    * Set the mode on a crtc crtcId with the given mode modeId.
    */
    pub fn drmModeSetCrtc(fd: c_int, crtcId: uint32_t, bufferId: uint32_t,
        x: uint32_t, y: uint32_t, connectors: *mut uint32_t, count: c_int,
        mode: drmModeModeInfoPtr) -> c_int;

    /*
    * Cursor functions
    */

    /**
    * Set the cursor on crtc
    */
    pub fn drmModeSetCursor(fd: c_int, crtcId: uint32_t, bo_handle: uint32_t, width: uint32_t,
        height: uint32_t) -> c_int;

    pub fn drmModeSetCursor2(fd: c_int, crtcId: uint32_t, bo_handle: uint32_t, width: uint32_t,
        height: uint32_t, hot_x: int32_t, hot_y: int32_t) -> c_int;

    /**
    * Move the cursor on crtc
    */
    pub fn drmModeMoveCursor(fd: c_int, crtcId: uint32_t, x: c_int, y: c_int) -> c_int;

    /**
    * Encoder functions
    */
    pub fn drmModeGetEncoder(fd: c_int, encoder_id: uint32_t) -> drmModeEncoderPtr;

    /*
    * Connector manipulation
    */

    /**
    * Retrieve all information about the connector connectorId. This will do a
    * forced probe on the connector to retrieve remote information such as EDIDs
    * from the display device.
    */
    pub fn drmModeGetConnector(fd: c_int, connectorId: uint32_t) -> drmModeConnectorPtr;

    /**
    * Retrieve current information, i.e the currently active mode and encoder,
    * about the connector connectorId. This will not do any probing on the
    * connector or remote device, and only reports what is currently known.
    * For the complete set of modes and encoders associated with the connector
    * use drmModeGetConnector() which will do a probe to determine any display
    * link changes first.
    */
    pub fn drmModeGetConnectorCurrent(fd: c_int, connector_id: uint32_t) -> drmModeConnectorPtr;

    /**
    * Attaches the given mode to an connector.
    */
    pub fn drmModeAttachMode(fd: c_int, connectorId: uint32_t,
        mode_info: drmModeModeInfoPtr) -> c_int;

    /**
    * Detaches a mode from the connector
    * must be unused, by the given mode.
    */
    pub fn drmModeDetachMode(fd: c_int, connectorId: uint32_t,
        mode_info: drmModeModeInfoPtr) -> c_int;

    pub fn drmModeGetProperty(fd: c_int, propertyId: uint32_t) -> drmModePropertyPtr;
    pub fn drmModeFreeProperty(ptr: drmModePropertyPtr) -> c_void;

    pub fn drmModeGetPropertyBlob(fd: c_int, blob_id: uint32_t) -> drmModePropertyBlobPtr;
    pub fn drmModeFreePropertyBlob(ptr: drmModePropertyBlobPtr) -> c_void;
    pub fn drmModeConnectorSetProperty(fd: c_int, connector_id: uint32_t, property_id: uint32_t,
        value: uint64_t) -> c_int;
    pub fn drmCheckModesettingSupported(busid: *const c_char) -> c_int;

    pub fn drmModeCrtcSetGamma(fd: c_int, crtc_id: uint32_t, size: uint32_t,
        red: *mut uint16_t, green: *mut uint16_t, blue: *mut uint16_t) -> c_int;
    pub fn drmModeCrtcGetGamma(fd: c_int, crtc_id: uint32_t, size: uint32_t,
        red: *mut uint16_t, green: *mut uint16_t, blue: *mut uint16_t) -> c_int;
    pub fn drmModePageFlip(fd: c_int, crtc_id: uint32_t, fb_id: uint32_t,
        flags: uint32_t, user_data: *mut c_void) -> c_int;

    pub fn drmModeGetPlaneResources(fd: c_int) -> drmModePlaneResPtr;
    pub fn drmModeGetPlane(fd: c_int, plane_id: uint32_t) -> drmModePlanePtr;
    pub fn drmModeSetPlane(fd: c_int, plane_id: uint32_t, crtc_id: uint32_t,
        fb_id: uint32_t, flags: uint32_t,
        crtc_x: int32_t, crtc_y: int32_t,
        crtc_w: uint32_t, crtc_h: uint32_t,
        src_x: uint32_t, src_y: uint32_t,
        src_w: uint32_t, src_h: uint32_t) -> c_int;

    pub fn drmModeObjectGetProperties(fd: c_int,
        object_id: uint32_t,
        object_type: uint32_t) -> drmModeObjectPropertiesPtr;
    pub fn drmModeFreeObjectProperties(ptr: drmModeObjectPropertiesPtr) -> c_void;
    pub fn drmModeObjectSetProperty(fd: c_int, object_id: uint32_t,
        object_type: uint32_t, property_id: uint32_t,
        value: uint64_t) -> c_int;

    pub fn drmModeAtomicAlloc() -> drmModeAtomicReqPtr;
    pub fn drmModeAtomicDuplicate(req: drmModeAtomicReqPtr) -> drmModeAtomicReqPtr;
    pub fn drmModeAtomicMerge(base: drmModeAtomicReqPtr,
        augment: drmModeAtomicReqPtr) -> c_int;
    pub fn drmModeAtomicFree(req: drmModeAtomicReqPtr) -> c_void;
    pub fn drmModeAtomicGetCursor(req: drmModeAtomicReqPtr) -> c_int;
    pub fn drmModeAtomicSetCursor(req: drmModeAtomicReqPtr, cursor: c_int) -> c_void;
    pub fn drmModeAtomicAddProperty(req: drmModeAtomicReqPtr,
        object_id: uint32_t,
        property_id: uint32_t,
        value: uint64_t) -> c_int;
    pub fn drmModeAtomicCommit(fd: c_int,
        req: drmModeAtomicReqPtr,
        flags: uint32_t,
        user_data: *mut c_void) -> c_int;

    pub fn drmModeCreatePropertyBlob(fd: c_int, data: *const c_void, size: size_t,
        id: *mut uint32_t) -> c_int;
    pub fn drmModeDestroyPropertyBlob(fd: c_int, id: uint32_t) -> c_int;
}
