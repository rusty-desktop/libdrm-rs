/*
 * Copyright 1999, 2000 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 */

use libc::*;
use byteorder::{ByteOrder, NativeEndian};
use drm::*;
use util::VolatileCell;

pub type va_list = c_void;

#[repr(C)]
pub enum drmMapType {
    DRM_FRAME_BUFFER    = 0,
    DRM_REGISTERS       = 1,
    DRM_SHM             = 2,
    DRM_AGP             = 3,
    DRM_SCATTER_GATHER  = 4,
    DRM_CONSISTENT      = 5
}

#[repr(C)]
pub enum drmMapFlags {
    DRM_RESTRICTED      = 0x0001,
    DRM_READ_ONLY       = 0x0002,
    DRM_LOCKED          = 0x0004,
    DRM_KERNEL          = 0x0008,
    DRM_WRITE_COMBINING = 0x0010,
    DRM_CONTAINS_LOCK   = 0x0020,
    DRM_REMOVABLE	  = 0x0040
}

/**
 * \warning These values *MUST* match drm.h
 */
 #[repr(C)]
pub enum drmDMAFlags {

    /*@{*/
    DRM_DMA_BLOCK        = 0x01, /**<
				  * Block until buffer dispatched.
				  *
				  * \note the buffer may not yet have been
				  * processed by the hardware -- getting a
				  * hardware lock with the hardware quiescent
				  * will ensure that the buffer has been
				  * processed.
				  */
    DRM_DMA_WHILE_LOCKED = 0x02,
    DRM_DMA_PRIORITY     = 0x04,
    /*@}*/


    /*@{*/
    DRM_DMA_WAIT         = 0x10,
    DRM_DMA_SMALLER_OK   = 0x20,
    DRM_DMA_LARGER_OK    = 0x40
    /*@}*/
}

#[repr(C)]
pub enum drmBufDescFlags {
    DRM_PAGE_ALIGN       = 0x01,
    DRM_AGP_BUFFER       = 0x02,
    DRM_SG_BUFFER        = 0x04,
    DRM_FB_BUFFER        = 0x08,
    DRM_PCI_BUFFER_RO    = 0x10
}

#[repr(C)]
pub enum drmLockFlags {
    DRM_LOCK_READY      = 0x01,
    DRM_LOCK_QUIESCENT  = 0x02,
    DRM_LOCK_FLUSH      = 0x04,
    DRM_LOCK_FLUSH_ALL  = 0x08,
				/* These *HALT* flags aren't supported yet
                                   -- they will be used to support the
                                   full-screen DGA-like mode. */
    DRM_HALT_ALL_QUEUES = 0x10,
    DRM_HALT_CUR_QUEUES = 0x20
}

#[repr(C)]
pub enum drm_context_tFlags {
    DRM_CONTEXT_PRESERVED = 0x01,
    DRM_CONTEXT_2DONLY    = 0x02
}
pub type drm_context_tFlagsPtr = *mut drm_context_tFlags;

#[repr(C)]
pub struct drmBufDesc {
    count: c_int,	  /**< Number of buffers of this size */
    size: c_int,	  /**< Size in bytes */
    low_mark: c_int,  /**< Low water mark */
    high_mark: c_int
}
impl ::std::default::Default for drmBufDesc {
    fn default() -> drmBufDesc { unsafe { ::std::mem::zeroed() } }
}

pub type drmBufDescPtr = *mut drmBufDesc;

#[repr(C)]
pub struct drmBufInfo {
    count: c_int,	        /**< Number of buffers described in list */
    list: *mut drmBufDesc
}
impl ::std::default::Default for drmBufInfo {
    fn default() -> drmBufInfo { unsafe { ::std::mem::zeroed() } }
}

pub type drmBufInfoPtr = *mut drmBufInfo;

#[repr(C)]
pub struct drmBuf {
    idx: c_int,	        /**< Index into the master buffer list */
    total: c_int,	    /**< Buffer size */
    used: c_int,	    /**< Amount of buffer in use (for DMA) */
    address: drmAddress
}
impl ::std::default::Default for drmBuf {
    fn default() -> drmBuf { unsafe { ::std::mem::zeroed() } }
}

pub type drmBufPtr = *mut drmBuf;

/**
 * Buffer mapping information.
 *
 * Used by drmMapBufs() and drmUnmapBufs() to store information about the
 * mapped buffers.
 */
#[repr(C)]
pub struct drmBufMap {
    count: c_int,	  /**< Number of buffers mapped */
    list: drmBufPtr
}
impl ::std::default::Default for drmBufMap {
    fn default() -> drmBufMap { unsafe { ::std::mem::zeroed() } }
}

pub type drmBufMapPtr = *mut drmBufMap;

#[repr(C)]
pub struct drmLock {
    lock: VolatileCell<c_uint>,
    padding: [c_char; 60]
    /* This is big enough for most current (and future?) architectures:
       DEC Alpha:              32 bytes
       Intel Merced:           ?
       Intel P5/PPro/PII/PIII: 32 bytes
       Intel StrongARM:        32 bytes
       Intel i386/i486:        16 bytes
       MIPS:                   32 bytes (?)
       Motorola 68k:           16 bytes
       Motorola PowerPC:       32 bytes
       Sun SPARC:              32 bytes
    */
}
impl ::std::default::Default for drmLock {
    fn default() -> drmLock { unsafe { ::std::mem::zeroed() } }
}

pub type drmLockPtr = *mut drmLock;

/**
 * Indices here refer to the offset into
 * list in drmBufInfo
 */
 #[repr(C)]
pub struct drmDMAReq {
    context: drm_context_t,  	  /**< Context handle */
    send_count: c_int,            /**< Number of buffers to send */
    send_list: *mut c_int,        /**< List of handles to buffers */
    send_sizes: *mut c_int,       /**< Lengths of data to send, in bytes */
    flags: drmDMAFlags,           /**< Flags */
    request_count: c_int,         /**< Number of buffers requested */
    request_size: c_int,          /**< Desired size of buffers requested */
    request_list: *mut c_int,     /**< Buffer information */
    request_sizes: *mut c_int,    /**< Minimum acceptable sizes */
    granted_count: c_int
}
impl ::std::default::Default for drmDMAReq {
    fn default() -> drmDMAReq { unsafe { ::std::mem::zeroed() } }
}

pub type drmDMAReqPtr = *mut drmDMAReq;

#[repr(C)]
pub struct drmRegion {
    handle: drm_handle_t,
    offset: c_uint,
    size: drmSize,
    map: drmAddress
}
impl ::std::default::Default for drmRegion {
    fn default() -> drmRegion { unsafe { ::std::mem::zeroed() } }
}

pub type drmRegionPtr = *mut drmRegion;

#[repr(C)]
pub struct drmTextureRegion {
    next: c_uchar,
    prev: c_uchar,
    in_use: c_uchar,
    padding: c_uchar,	/**< Explicitly pad this out */
    age: c_uint
}
impl ::std::default::Default for drmTextureRegion {
    fn default() -> drmTextureRegion { unsafe { ::std::mem::zeroed() } }
}

pub type drmTextureRegionPtr = *mut drmTextureRegion;

#[repr(C)]
pub struct drmPciBusInfo {
    domain: uint16_t,
    bus: uint8_t,
    dev: uint8_t,
    func: uint8_t,
}
impl ::std::default::Default for drmPciBusInfo {
    fn default() -> drmPciBusInfo { unsafe { ::std::mem::zeroed() } }
}

pub type drmPciBusInfoPtr = *mut drmPciBusInfo;

#[repr(C)]
pub struct drmPciDeviceInfo {
    vendor_id: uint16_t,
    device_id: uint16_t,
    subvendor_id: uint16_t,
    subdevice_id: uint16_t,
    revision_id: uint8_t,
}
impl ::std::default::Default for drmPciDeviceInfo {
    fn default() -> drmPciDeviceInfo { unsafe { ::std::mem::zeroed() } }
}

pub type drmPciDeviceInfoPtr = *mut drmPciDeviceInfo;

#[repr(C)]
pub struct businfo {
    pci: drmPciBusInfoPtr
}
impl ::std::default::Default for businfo {
    fn default() -> businfo { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct deviceinfo {
    pci: drmPciDeviceInfoPtr
}
impl ::std::default::Default for deviceinfo {
    fn default() -> deviceinfo { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct drmDevice {
    nodes: *mut *mut c_char, /* DRM_NODE_MAX sized array */
    available_nodes: c_int, /* DRM_NODE_* bitmask */
    bustype: c_int,
    businfo: businfo,
    deviceinfo: deviceinfo
}
impl ::std::default::Default for drmDevice {
    fn default() -> drmDevice { unsafe { ::std::mem::zeroed() } }
}

pub type drmDevicePtr = *mut drmDevice;

#[repr(C)]
pub struct drmEventContext {
	/* This struct is versioned so we can add more pointers if we
	 * add more events. */
	version: c_int,

	vblank_handler: extern fn(fd: c_int,
			       sequence: c_uint,
			       tv_sec: c_uint,
			       tv_usec: c_uint,
			       user_data: *mut c_void) -> c_void,

	page_flip_handler: extern fn(fd: c_int,
				  sequence: c_uint,
				  tv_sec: c_uint,
				  tv_usec: c_uint,
				  user_data: *mut c_void) -> c_void
}
impl ::std::default::Default for drmEventContext {
    fn default() -> drmEventContext { unsafe { ::std::mem::zeroed() } }
}

pub type drmEventContextPtr = *mut drmEventContext;

#[repr(C)]
pub enum drmVBlankSeqType {
    DRM_VBLANK_ABSOLUTE = 0x0,
    DRM_VBLANK_RELATIVE = 0x1,
    /* bits 1-6 are reserved for high crtcs */
    DRM_VBLANK_HIGH_CRTC_MASK = 0x0000003e,
    DRM_VBLANK_EVENT = 0x4000000,
    DRM_VBLANK_FLIP = 0x8000000,
    DRM_VBLANK_NEXTONMISS = 0x10000000,
    DRM_VBLANK_SECONDARY = 0x20000000,
    DRM_VBLANK_SIGNAL   = 0x40000000	/* Send signal instead of blocking */
}
const DRM_VBLANK_HIGH_CRTC_SHIFT : u8 = 1;
impl drmVBlankSeqType {
    fn from_u32(n: u32) -> drmVBlankSeqType {
        match n {
            0x0         => drmVBlankSeqType::DRM_VBLANK_ABSOLUTE,
            0x1         => drmVBlankSeqType::DRM_VBLANK_RELATIVE,
            0x0000003e  => drmVBlankSeqType::DRM_VBLANK_HIGH_CRTC_MASK,
            0x04000000  => drmVBlankSeqType::DRM_VBLANK_EVENT,
            0x08000000  => drmVBlankSeqType::DRM_VBLANK_FLIP,
            0x10000000  => drmVBlankSeqType::DRM_VBLANK_NEXTONMISS,
            0x20000000  => drmVBlankSeqType::DRM_VBLANK_SECONDARY,
            0x40000000  => drmVBlankSeqType::DRM_VBLANK_SIGNAL,
            _           => drmVBlankSeqType::DRM_VBLANK_ABSOLUTE
        }
    }
}

#[repr(C)]
pub struct drmVBlankReq {
	request_type: drmVBlankSeqType,
	sequence: c_uint,
	signal: c_ulong,
}
impl ::std::default::Default for drmVBlankReq {
    fn default() -> drmVBlankReq { unsafe { ::std::mem::zeroed() } }
}

pub type drmVBlankReqPtr = *mut drmVBlankReq;

#[repr(C)]
pub struct drmVBlankReply {
	reply_type: drmVBlankSeqType,
	sequence: c_uint,
	tval_sec: c_long,
	tval_usec: c_long
}
impl ::std::default::Default for drmVBlankReply {
    fn default() -> drmVBlankReply { unsafe { ::std::mem::zeroed() } }
}

pub type drmVBlankReplyPtr = *mut drmVBlankReply;

#[repr(C)]
pub struct drmVBlank {
    data: [u8; 24]
}
impl ::std::default::Default for drmVBlank {
    fn default() -> drmVBlank { unsafe { ::std::mem::zeroed() } }
}

pub type drmVBlankPtr = *mut drmVBlank;

impl drmVBlank {
    #[cfg(target_pointer_width = "32")]
    fn request (&self) -> drmVBlankReq {
        let req = drmVBlankReq {
            request_type: drmVBlankSeqType::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            signal: NativeEndian::read_u32(&self.data[9..16])
        };

        req
    }

    #[cfg(target_pointer_width = "32")]
    fn reply (&self) -> drmVBlankReply {
        let reply = drmVBlankReply {
            reply_type: drmVBlankSeqType::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            tval_sec: NativeEndian::read_i32(&self.data[9..16]),
            tval_usec: NativeEndian::read_i32(&self.data[17..24])
        };

        reply
    }

    #[cfg(target_pointer_width = "64")]
    fn request (&self) -> drmVBlankReq {
        let req = drmVBlankReq {
            request_type: drmVBlankSeqType::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            signal: NativeEndian::read_u64(&self.data[9..16])
        };

        req
    }

    #[cfg(target_pointer_width = "64")]
    fn reply (&self) -> drmVBlankReply {
        let reply = drmVBlankReply {
            reply_type: drmVBlankSeqType::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            tval_sec: NativeEndian::read_i64(&self.data[9..16]),
            tval_usec: NativeEndian::read_i64(&self.data[17..24])
        };

        reply
    }
}

#[repr(C)]
pub struct drmServerInfo {
  debug_print: extern fn(format: *const c_char, ap: va_list) -> c_int,
  load_module: extern fn(name: *const c_char) -> c_int,
  get_perms: extern fn(gid: *mut gid_t, mode: *mut mode_t) -> c_void
}
impl ::std::default::Default for drmServerInfo {
    fn default() -> drmServerInfo { unsafe { ::std::mem::zeroed() } }
}

pub type drmServerInfoPtr = *mut drmServerInfo;

#[repr(C)]
pub struct drmHashEntry {
    fd: c_int,
    f: extern fn(num: c_int, ptr1: *mut c_void, ptr2: *mut c_void) -> c_void,
    tag_table: *mut c_void
}
impl ::std::default::Default for drmHashEntry {
    fn default() -> drmHashEntry { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct drmVersion {
    version_major:      c_int,          /**< Major version */
    version_minor:      c_int,          /**< Minor version */
    version_patchlevel: c_int,          /**< Patch level */
    name_len:           c_int,          /**< Length of name buffer */
    name:               *mut c_char,    /**< Name of driver */
    date_len:           c_int,          /**< Length of date buffer */
    date:               *mut c_char,    /**< User-space buffer to hold date */
    desc_len:           c_int,          /**< Length of desc buffer */
    desc:               *mut c_char
}
impl ::std::default::Default for drmVersion {
    fn default() -> drmVersion { unsafe { ::std::mem::zeroed() } }
}

pub type drmVersionPtr = *mut drmVersion;

#[repr(C)]
pub struct drmSetVersion {
	drm_di_major: c_int,
	drm_di_minor: c_int,
	drm_dd_major: c_int,
	drm_dd_minor: c_int
}
impl ::std::default::Default for drmSetVersion {
    fn default() -> drmSetVersion { unsafe { ::std::mem::zeroed() } }
}

pub type drmSetVersionPtr = *mut drmSetVersion;

#[repr(C)]
pub struct drmStatsTData {
    value: c_ulong,
    long_format: *const c_char,
    long_name: *const c_char,
    rate_format: *const c_char,
    rate_name: *const c_char,
    isvalue: c_int,
    mult_names: *const c_char,
    mult: c_int,
    verbose: c_int
}
impl ::std::default::Default for drmStatsTData {
    fn default() -> drmStatsTData { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct drmStatsT {
    count: c_ulong,
    data: *mut [drmStatsTData; 15]
}
impl ::std::default::Default for drmStatsT {
    fn default() -> drmStatsT { unsafe { ::std::mem::zeroed() } }
}


pub type drm_handle_t = c_uint;

pub type drmSize = c_uint;
pub type drmSizePtr = *mut drmSize;
pub type drmAddress = *mut c_void;
pub type drmAddressPtr = *mut drmAddress;

pub const DRM_NODE_PRIMARY : u8 = 0;
pub const DRM_NODE_CONTROL : u8 = 1;
pub const DRM_NODE_RENDER : u8 = 2;
pub const DRM_NODE_MAX : u8 = 3;
pub const DRM_EVENT_CONTEXT_VERSION : u8 = 2;
pub const DRM_BUS_PCI : u8 = 0;

#[link(name = "drm")]
#[allow(dead_code)]
extern {
    /* General user-level programmer's API: unprivileged */
    pub fn drmAvailable() -> c_int;
    pub fn drmOpen(name: *const c_char, busid: *const c_char) -> c_int;
    pub fn drmOpenWithType(name: *const c_char, busid: *const c_char, open_type: c_int) -> c_int;
    pub fn drmOpenControl(minor: c_int) -> c_int;
    pub fn drmOpenRender(minor: c_int) -> c_int;
    pub fn drmClose(fd: c_int) -> c_int;
    pub fn drmGetVersion(fd: c_int) -> drmVersionPtr;
    pub fn drmGetLibVersion(fd: c_int)-> drmVersionPtr;
    pub fn drmGetCap(fd: c_int, capability: uint64_t, value: *mut uint64_t) -> c_int;
    pub fn drmFreeVersion(ptr: drmVersionPtr) -> c_void;
    pub fn drmGetMagic(fd: c_int, magic: *mut drm_magic_t) -> c_int;
    pub fn drmGetBusid(fd: c_int) -> c_char;
    pub fn drmGetInterruptFromBusID(fd: c_int, busnum: c_int, devnum: c_int, funcnum: c_int) -> c_int;
    pub fn drmGetMap(fd: c_int, idx: c_int, offset: *mut drm_handle_t,
        size: drmSizePtr, map_type: *mut drmMapType,
        flags: *mut drmMapFlags, handle: *mut drm_handle_t,
        mtrr: *mut c_int) -> c_int;
    pub fn drmGetClient(fd: c_int, idx: c_int, auth: *mut c_int, pid: *mut c_int,
	    uid: *mut c_int, magic: *mut c_ulong,
	    iocs: *mut c_ulong) -> c_int;
    pub fn drmGetStats(fd: c_int, stats: *mut drmStatsT) -> c_int;
    pub fn drmSetInterfaceVersion(fd: c_int, version: drmSetVersionPtr) -> c_int;
    pub fn drmCommandNone(fd: c_int, drmCommandIndex: c_long) -> c_int;
    pub fn drmCommandRead(fd: c_int, drmCommandIndex: c_ulong,
        data: *mut c_void, size: c_ulong) -> c_int;
    pub fn drmCommandWrite(fd: c_int, drmCommandIndex: c_ulong,
        data: *mut c_void, size: c_ulong) -> c_int;
    pub fn drmCommandWriteRead(fd: c_int, drmCommandIndex: c_ulong,
        data: *mut c_void, size: c_ulong) -> c_int;

/* General user-level programmer's API: X server (root) only  */
    pub fn drmFreeBusid(busid: *const c_char) -> c_void;
    pub fn drmSetBusid(fd: c_int, busid: *const c_char) -> c_int;
    pub fn drmAuthMagic(fd: c_int, magic: drm_magic_t) -> c_int;
    pub fn drmAddMap(fd: c_int,
        offset: drm_handle_t,
        size: drmSize,
        map_type: drmMapType,
        flags: drmMapFlags,
        handle: *mut drm_handle_t) -> c_int;
    pub fn drmRmMap(fd: c_int, handle: drm_handle_t) -> c_int;
    pub fn drmAddContextPrivateMapping(fd: c_int, ctx_id: drm_context_t,
        handle: drm_handle_t) -> c_int;
    pub fn drmAddBufs(fd: c_int, count: c_int, size: c_int, flags: drmBufDescFlags,
		agp_offset: c_int) -> c_int;
    pub fn drmMarkBufs(fd: c_int, low: c_double, high: c_double) -> c_int;
    pub fn drmCreateContext(fd: c_int, handle: *mut drm_context_t) -> c_int;
    pub fn drmSetContextFlags(fd: c_int, context: drm_context_t,
		flags: drm_context_tFlags) -> c_int;
    pub fn drmGetContextFlags(fd: c_int, context: drm_context_t,
		flags: *mut drm_context_tFlags) -> c_int;
    pub fn drmAddContextTag(fd: c_int, context: drm_context_t, tag: *mut c_void) -> c_int;
    pub fn drmDelContextTag(fd: c_int, context: drm_context_t) -> c_int;
    pub fn drmGetContextTag(fd: c_int, context: drm_context_t) -> *mut c_void;
    pub fn drmGetReservedContextList(fd: c_int, count: *mut c_int) -> *mut drm_context_t;
    pub fn drmFreeReservedContextList(context: *mut drm_context_t) -> c_void;
    pub fn drmSwitchToContext(fd: c_int, context: drm_context_t) -> c_int;
    pub fn drmDestroyContext(fd: c_int, handle: drm_context_t) -> c_int;
    pub fn drmCreateDrawable(fd: c_int, handle: *mut drm_drawable_t) -> c_int;
    pub fn drmDestroyDrawable(fd: c_int, handle: drm_drawable_t) -> c_int;
    pub fn drmUpdateDrawableInfo(fd: c_int, handle: drm_drawable_t,
		drawable_info_type: drm_drawable_info_type_t,
		num: c_uint, data: *mut c_void) -> c_int;
    pub fn drmCtlInstHandler(fd: c_int, irq: c_int) -> c_int;
    pub fn drmCtlUninstHandler(fd: c_int) -> c_int;
    pub fn drmSetClientCap(fd: c_int, capability: uint64_t, value: uint64_t) -> c_int;

/* General user-level programmer's API: authenticated client and/or X */
    pub fn drmMap(fd: c_int, handle: drm_handle_t, size: drmSize,
		address: drmAddressPtr) -> c_int;
    pub fn drmUnmap(address: drmAddress, size: drmSize) -> c_int;
    pub fn drmGetBufInfo(fd: c_int) -> drmBufInfoPtr;
    pub fn drmMapBufs(fd: c_int) -> drmBufMapPtr;
    pub fn drmUnmapBufs(bufs: drmBufMapPtr) -> c_int;
    pub fn drmDMA(fd: c_int, request: drmDMAReqPtr) -> c_int;
    pub fn drmFreeBufs(fd: c_int, count: c_int, list: *mut c_int) -> c_int;
    pub fn drmGetLock(fd: c_int, context: drm_context_t, flags: drmLockFlags) -> c_int;
    pub fn drmUnlock(fd: c_int, context: drm_context_t) -> c_int;
    pub fn drmFinish(fd: c_int, context: c_int, flags: drmLockFlags) -> c_int;
    pub fn drmGetContextPrivateMapping(fd: c_int, ctx_id: drm_context_t,
		handle: *mut drm_handle_t) -> c_int;

/* AGP/GART support: X server (root) only */
    pub fn drmAgpAcquire(fd: c_int) -> c_int;
    pub fn drmAgpRelease(fd: c_int) -> c_int;
    pub fn drmAgpEnable(fd: c_int, mode: c_ulong) -> c_int;
    pub fn drmAgpAlloc(fd: c_int, size: c_ulong,
        agp_alloc_type: c_ulong, address: *mut c_ulong,
		handle: *mut drm_handle_t) -> c_int;
    pub fn drmAgpFree(fd: c_int, handle: drm_handle_t) -> c_int;
    pub fn drmAgpBind(fd: c_int, handle: drm_handle_t,
        offset: c_ulong) -> c_int;
    pub fn drmAgpUnbind(fd: c_int, handle: drm_handle_t) -> c_int;

/* AGP/GART info: authenticated client and/or X */
    pub fn drmAgpVersionMajor(fd: c_int) -> c_int;
    pub fn drmAgpVersionMinor(fd: c_int) -> c_int;
    pub fn drmAgpGetMode(fd: c_int) -> c_ulong;
    pub fn drmAgpBase(fd: c_int) -> c_ulong; /* Physical location */
    pub fn drmAgpSize(fd: c_int) -> c_ulong; /* Bytes */
    pub fn drmAgpMemoryUsed(fd: c_int) -> c_ulong;
    pub fn drmAgpMemoryAvail(fd: c_int) -> c_ulong;
    pub fn drmAgpVendorId(fd: c_int) -> c_uint;
    pub fn drmAgpDeviceId(fd: c_int) -> c_uint;

/* PCI scatter/gather support: X server (root) only */
    pub fn drmScatterGatherAlloc(fd: c_int, size: c_ulong,
        handle: *mut drm_handle_t) -> c_int;
    pub fn drmScatterGatherFree(fd: c_int, handle: drm_handle_t) -> c_int;
    pub fn drmWaitVBlank(fd: c_int, vbl: *mut drmVBlank) -> c_int;

/* Support routines */
    pub fn drmSetServerInfo(info: drmServerInfoPtr) -> c_void;
    pub fn drmError(err: c_int, label: *const c_char) -> c_int;
    pub fn drmMalloc(size: c_int) -> *mut c_void;
    pub fn drmFree(pt: *mut c_void) -> c_void;

/* Hash table routines */
    pub fn drmHashCreate() -> *mut c_void;
    pub fn drmHashDestroy(t: *mut c_void) -> c_int;
    pub fn drmHashLookup(t: *mut c_void, key: c_ulong, value: *mut *mut c_void) -> c_int;
    pub fn drmHashInsert(t: *mut c_void, key: c_ulong, value: *mut c_void) -> c_int;
    pub fn drmHashDelete(t: *mut c_void, key: c_ulong) -> c_int;
    pub fn drmHashFirst(t: *mut c_void, key: c_ulong, value: *mut *mut c_void) -> c_int;
    pub fn drmHashNext(t: *mut c_void, key: *mut c_ulong,
        value: *mut *mut c_void) -> c_int;

/* PRNG routines */
    pub fn drmRandomCreate(seed: c_ulong) -> *mut c_void;
    pub fn drmRandomDestroy(state: *mut c_void) -> c_int;
    pub fn drmRandom(state: *mut c_void) -> c_ulong;
    pub fn drmRandomDouble(state: *mut c_void) -> c_double;

/* Skip list routines */

    pub fn drmSLCreate() -> *mut c_void;
    pub fn drmSLDestroy(l: *mut c_void) -> c_int;
    pub fn drmSLLookup(l: *mut c_void, key: c_ulong, value: *mut *mut c_void) -> c_int;
    pub fn drmSLInsert(l: *mut c_void, key: c_ulong, value: *mut c_void) -> c_int;
    pub fn drmSLDelete(l: *mut c_void, key: c_ulong) -> c_int;
    pub fn drmSLNext(l: *mut c_void, key: *mut c_ulong, value: *mut *mut c_void) -> c_int;
    pub fn drmSLFirst(l: *mut c_void, key: *mut c_ulong, value: *mut *mut c_void) -> c_int;
    pub fn drmSLDump(l: *mut c_void) -> c_void;
    pub fn drmSLLookupNeighbors(l: *mut c_void, key: c_ulong,
        prev_key: *mut c_ulong, prev_value: *mut *mut c_void,
		next_key: *mut c_ulong, next_value: *mut *mut c_void) -> c_int;

    pub fn drmOpenOnce(unused: *mut c_void, BusID: *const c_char,
        newlyopened: *mut c_int) -> c_int;
    pub fn drmOpenOnceWithType(BusID: *const c_char, newlyopened: *mut c_int,
        open_type: c_int) -> c_int;
    pub fn drmCloseOnce(fd: c_int) -> c_void;
    pub fn drmMsg(format: *const c_char, ...) -> c_void;

    pub fn drmSetMaster(fd: c_int) -> c_int;
    pub fn drmDropMaster(fd: c_int) -> c_int;

    pub fn drmHandleEvent(fd: c_int, evctx: drmEventContextPtr) -> c_int;

    pub fn drmGetDeviceNameFromFd(fd: c_int) -> *mut c_char;
    pub fn drmGetNodeTypeFromFd(fd: c_int) -> c_int;

    pub fn drmPrimeHandleToFD(fd: c_int, handle: uint32_t, flags: uint32_t,
        prime_fd: *mut c_int) -> c_int;
    pub fn drmPrimeFDToHandle(fd: c_int, prime_fd: c_int, handle: *mut uint32_t) -> c_int;

    pub fn drmGetPrimaryDeviceNameFromFd(fd: c_int) -> *mut c_char;
    pub fn drmGetRenderDeviceNameFromFd(fd: c_int) -> *mut c_char;

    pub fn drmGetDevice(fd: c_int, device: *mut drmDevicePtr) -> c_int;
    pub fn drmFreeDevice(device: *mut drmDevicePtr) -> c_void;

    pub fn drmGetDevices(devices: *const drmDevicePtr, max_devices: c_int) -> c_int;
    pub fn drmFreeDevices(devices: *const drmDevicePtr, count: c_int) -> c_void;
}
