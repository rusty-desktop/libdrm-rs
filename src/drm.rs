/*
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * All rights reserved.
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
 * VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */
use libc::*;
use byteorder::{ByteOrder, NativeEndian};
use util::*;

pub const DRM_NAME: &'static str = "drm";	        /**< Name in kernel, /dev, and /proc */
pub const DRM_MIN_ORDER: c_int = 5;        /**< At least 2^5 bytes = 32 bytes */
pub const DRM_MAX_ORDER: u8	= 22;	    /**< Up to 2^22 bytes = 4MB */
pub const DRM_RAM_PERCENT: c_int = 10;	    /**< How much system ram can we lock? */

#[macro_export]
macro_rules! drm_io { ($nr:expr) => (io!(100, $nr)); }
#[macro_export]
macro_rules! drm_iow { ($nr:expr, $ty:expr) => (iow!(100, $nr, $ty)); }
#[macro_export]
macro_rules! drm_ior { ($nr:expr, $ty:expr) => (ior!(100, $nr, $ty)); }
#[macro_export]
macro_rules! drm_iowr { ($nr:expr, $ty:expr) => (iorw!(100, $nr, $ty)); }

#[macro_export]
macro_rules! drm_ioctl_version { () => (drm_iowr!(0x00, mem::size_of::<drm_version>())); }
#[macro_export]
macro_rules! drm_ioctl_get_unique { () => (drm_iowr!(0x01, mem::size_of::<drm_unique>())); }
#[macro_export]
macro_rules! drm_ioctl_get_magic { () => (drm_ior!(0x02, mem::size_of::<drm_auth>())); }
#[macro_export]
macro_rules! drm_ioctl_irq_busid { () => (drm_iowr!(0x03, mem::size_of::<drm_irq_busid>())); }
#[macro_export]
macro_rules! drm_ioctl_get_map { () => (drm_iowr!(0x04, mem::size_of::<drm_map>())); }
#[macro_export]
macro_rules! drm_ioctl_get_client { () => (drm_iowr!(0x05, mem::size_of::<drm_client>())); }
#[macro_export]
macro_rules! drm_ioctl_get_stats { () => (drm_ior!(0x06, mem::size_of::<drm_stats>())); }
#[macro_export]
macro_rules! drm_ioctl_set_version { () => (drm_iowr!(0x07, mem::size_of::<drm_set_version>())); }
#[macro_export]
macro_rules! drm_ioctl_modeset_ctl { () => (drm_iow!(0x08, mem::size_of::<drm_modeset_ctl>())); }
#[macro_export]
macro_rules! drm_ioctl_gem_close { () => (drm_iow!(0x09, mem::size_of::<drm_gem_close>())); }
#[macro_export]
macro_rules! drm_ioctl_gem_flink { () => (drm_iowr!(0x0a, mem::size_of::<drm_gem_flink>())); }
#[macro_export]
macro_rules! drm_ioctl_gem_open { () => (drm_iowr!(0x0b, mem::size_of::<drm_gem_open>())); }
#[macro_export]
macro_rules! drm_ioctl_get_cap { () => (drm_iowr!(0x0c, mem::size_of::<drm_get_cap>())); }
#[macro_export]
macro_rules! drm_ioctl_set_client_cap { () => (drm_iow!(0x0d, mem::size_of::<drm_set_client_cap>())); }

#[macro_export]
macro_rules! drm_ioctl_set_unique { () => (drm_iow!(0x10, mem::size_of::<drm_unique>())); }
#[macro_export]
macro_rules! drm_ioctl_auth_magic { () => (drm_iow!(0x11, mem::size_of::<drm_auth>())); }
#[macro_export]
macro_rules! drm_ioctl_block { () => (drm_iowr!(0x12, mem::size_of::<drm_block>())); }
#[macro_export]
macro_rules! drm_ioctl_unblock { () => (drm_iowr!(0x13, mem::size_of::<drm_block>())); }
#[macro_export]
macro_rules! drm_ioctl_control { () => (drm_iow!(0x14, mem::size_of::<drm_control>())); }
#[macro_export]
macro_rules! drm_ioctl_add_map { () => (drm_iowr!(0x15, mem::size_of::<drm_map>())); }
#[macro_export]
macro_rules! drm_ioctl_add_bufs { () => (drm_iowr!(0x16, mem::size_of::<drm_buf_desc>())); }
#[macro_export]
macro_rules! drm_ioctl_mark_bufs { () => (drm_iow!(0x17, mem::size_of::<drm_buf_desc>())); }
#[macro_export]
macro_rules! drm_ioctl_info_bufs { () => (drm_iowr!(0x18, mem::size_of::<drm_buf_info>())); }
#[macro_export]
macro_rules! drm_ioctl_map_bufs { () => (drm_iowr!(0x19, mem::size_of::<drm_buf_map>())); }
#[macro_export]
macro_rules! drm_ioctl_free_bufs { () => (drm_iow!(0x1a, mem::size_of::<drm_buf_free>())); }

#[macro_export]
macro_rules! drm_ioctl_rm_map { () => (drm_iow!(0x1b, mem::size_of::<drm_map>())); }

#[macro_export]
macro_rules! drm_ioctl_set_sarea_ctx { () => (drm_iow!(0x1c, mem::size_of::<drm_ctx_priv_map>())); }
#[macro_export]
macro_rules! drm_ioctl_get_sarea_ctx { () => (drm_iowr!(0x1d, mem::size_of::<drm_ctx_priv_map>())); }

#[macro_export]
macro_rules! drm_ioctl_set_master { () => (drm_io!(0x1e)); }
#[macro_export]
macro_rules! drm_ioctl_drop_master { () => (drm_io!(0x1f)); }

#[macro_export]
macro_rules! drm_ioctl_add_ctx { () => (drm_iowr!(0x20, mem::size_of::<drm_ctx>())); }
#[macro_export]
macro_rules! drm_ioctl_rm_ctx { () => (drm_iowr!(0x21, mem::size_of::<drm_ctx>())); }
#[macro_export]
macro_rules! drm_ioctl_mod_ctx { () => (drm_iow!(0x22, mem::size_of::<drm_ctx>())); }
#[macro_export]
macro_rules! drm_ioctl_get_ctx { () => (drm_iowr!(0x23, mem::size_of::<drm_ctx>())); }
#[macro_export]
macro_rules! drm_ioctl_switch_ctx { () => (drm_iow!(0x24, mem::size_of::<drm_ctx>())); }
#[macro_export]
macro_rules! drm_ioctl_new_ctx { () => (drm_iow!(0x25, mem::size_of::<drm_ctx>())); }
#[macro_export]
macro_rules! drm_ioctl_res_ctx { () => (drm_iowr!(0x26, mem::size_of::<drm_ctx_res>())); }
#[macro_export]
macro_rules! drm_ioctl_add_draw { () => (drm_iowr!(0x27, mem::size_of::<drm_draw>())); }
#[macro_export]
macro_rules! drm_ioctl_rm_draw { () => (drm_iowr!(0x28, mem::size_of::<drm_draw>())); }
#[macro_export]
macro_rules! drm_ioctl_dma { () => (drm_iowr!(0x29, mem::size_of::<drm_dma>())); }
#[macro_export]
macro_rules! drm_ioctl_lock { () => (drm_iow!(0x2a, mem::size_of::<drm_lock>())); }
#[macro_export]
macro_rules! drm_ioctl_unlock { () => (drm_iow!(0x2b, mem::size_of::<drm_lock>())); }
#[macro_export]
macro_rules! drm_ioctl_finish { () => (drm_iow!(0x2c, mem::size_of::<drm_lock>())); }

#[macro_export]
macro_rules! drm_ioctl_prime_handle_to_fd { () => (drm_iowr!(0x2d, mem::size_of::<drm_prime_handle>())); }
#[macro_export]
macro_rules! drm_ioctl_prime_fd_to_handle { () => (drm_iowr!(0x2e, mem::size_of::<drm_prime_handle>())); }

#[macro_export]
macro_rules! drm_ioctl_agp_acquire { () => (drm_io!(0x30)); }
#[macro_export]
macro_rules! drm_ioctl_agp_release { () => (drm_io!(0x31)); }
#[macro_export]
macro_rules! drm_ioctl_agp_enable { () => (drm_iow!(0x32, mem::size_of::<drm_agp_mode>())); }
#[macro_export]
macro_rules! drm_ioctl_agp_info { () => (drm_ior!(0x33, mem::size_of::<drm_agp_info>())); }
#[macro_export]
macro_rules! drm_ioctl_agp_alloc { () => (drm_iowr!(0x34, mem::size_of::<drm_agp_buffer>())); }
#[macro_export]
macro_rules! drm_ioctl_agp_free { () => (drm_iow!(0x35, mem::size_of::<drm_agp_buffer>())); }
#[macro_export]
macro_rules! drm_ioctl_agp_bind { () => (drm_iow!(0x36, mem::size_of::<drm_agp_binding>())); }
#[macro_export]
macro_rules! drm_ioctl_agp_unbind { () => (drm_iow!(0x37, mem::size_of::<drm_agp_binding>())); }

#[macro_export]
macro_rules! drm_ioctl_sg_alloc { () => (drm_iowr!(0x38, mem::size_of::<drm_scatter_gather>())); }
#[macro_export]
macro_rules! drm_ioctl_sg_free { () => (drm_iow!(0x39, mem::size_of::<drm_scatter_gather>())); }

#[macro_export]
macro_rules! drm_ioctl_wait_vblank { () => (drm_iowr!(0x3a, mem::size_of::<drm_wait_vblank>())); }

#[macro_export]
macro_rules! drm_ioctl_update_draw { () => (drm_iow!(0x3f, mem::size_of::<drm_update_draw>())); }

#[macro_export]
macro_rules! drm_ioctl_mode_getresources { () => (drm_iowr!(0xA0, mem::size_of::<drm_mode_card_res>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getcrtc { () => (drm_iowr!(0xA1, mem::size_of::<drm_mode_crtc>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_setcrtc { () => (drm_iowr!(0xA2, mem::size_of::<drm_mode_crtc>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_cursor { () => (drm_iowr!(0xA3, mem::size_of::<drm_mode_cursor>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getgamma { () => (drm_iowr!(0xA4, mem::size_of::<drm_mode_crtc_lut>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_setgamma { () => (drm_iowr!(0xA5, mem::size_of::<drm_mode_crtc_lut>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getencoder { () => (drm_iowr!(0xA6, mem::size_of::<drm_mode_get_encoder>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getconnector { () => (drm_iowr!(0xA7, mem::size_of::<drm_mode_get_connector>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_attachmode { () => (drm_iowr!(0xA8, mem::size_of::<drm_mode_mode_cmd>())); } /* deprecated (never worked) */
#[macro_export]
macro_rules! drm_ioctl_mode_detachmode { () => (drm_iowr!(0xA9, mem::size_of::<drm_mode_mode_cmd>())); } /* deprecated (never worked) */

#[macro_export]
macro_rules! drm_ioctl_mode_getproperty { () => (drm_iowr!(0xAA, mem::size_of::<drm_mode_get_property>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_setproperty { () => (drm_iowr!(0xAB, mem::size_of::<drm_mode_connector_set_property>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getpropblob { () => (drm_iowr!(0xAC, mem::size_of::<drm_mode_get_blob>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getfb { () => (drm_iowr!(0xAD, mem::size_of::<drm_mode_fb_cmd>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_addfb { () => (drm_iowr!(0xAE, mem::size_of::<drm_mode_fb_cmd>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_addfb { () => (drm_iowr!(0xAE, mem::size_of::<c_uint>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_page_flip { () => (drm_iowr!(0xB0, mem::size_of::<drm_mode_crtc_page_flip>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_dirtyfb { () => (drm_iowr!(0xB1, mem::size_of::<drm_mode_fb_dirty_cmd>())); }

#[macro_export]
macro_rules! drm_ioctl_mode_create_dumb { () => (drm_iowr!(0xB2, mem::size_of::<drm_mode_create_dumb>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_map_dumb { () => (drm_iowr!(0xB3, mem::size_of::<drm_mode_map_dumb>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_destroy_dumb { () => (drm_iowr!(0xB4, mem::size_of::<drm_mode_destroy_dumb>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getplaneresources { () => (drm_iowr!(0xB5, mem::size_of::<drm_mode_get_plane_res>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_getplane { () => (drm_iowr!(0xB6, mem::size_of::<drm_mode_get_plane>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_setplane { () => (drm_iowr!(0xB7, mem::size_of::<drm_mode_set_plane>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_addfb2 { () => (drm_iowr!(0xB8, mem::size_of::<drm_mode_fb_cmd2>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_obj_getproperties { () => (drm_iowr!(0xB9, mem::size_of::<drm_mode_obj_get_properties>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_obj_setproperty { () => (drm_iowr!(0xBA, mem::size_of::<drm_mode_obj_set_property>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_cursor2 { () => (drm_iowr!(0xBB, mem::size_of::<drm_mode_cursor2>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_atomic { () => (drm_iowr!(0xBC, mem::size_of::<drm_mode_atomic>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_createpropblob { () => (drm_iowr!(0xBD, mem::size_of::<drm_mode_create_blob>())); }
#[macro_export]
macro_rules! drm_ioctl_mode_destroypropblob { () => (drm_iowr!(0xBE, mem::size_of::<drm_mode_destroy_blob>())); }

pub type drm_context_t = c_uint;
pub type drm_drawable_t = c_uint;
pub type drm_magic_t = c_uint;

/**
 * Cliprect.
 *
 * \warning: If you change this structure, make sure you change
 * XF86DRIClipRectRec in the server as well
 *
 * \note KW: Actually it's illegal to change either for
 * backwards-compatibility reasons.
 */
 #[repr(C)]
pub struct drm_clip_rect {
	pub x1: c_ushort,
	pub y1: c_ushort,
	pub x2: c_ushort,
	pub y2: c_ushort
}
impl ::std::default::Default for drm_clip_rect {
    fn default() -> drm_clip_rect { unsafe { ::std::mem::zeroed() } }
}


/**
 * Drawable information.
 */
 #[repr(C)]
pub struct drm_drawable_info {
	pub num_rects: c_uint,
	pub rects: *mut drm_clip_rect
}
impl ::std::default::Default for drm_drawable_info {
    fn default() -> drm_drawable_info { unsafe { ::std::mem::zeroed() } }
}


/**
 * Texture region,
 */
#[repr(C)]
pub struct drm_tex_region {
	pub next: c_uchar,
	pub prev: c_uchar,
	pub in_use: c_uchar,
	pub padding: c_uchar,
	pub age: c_uint
}
impl ::std::default::Default for drm_tex_region {
    fn default() -> drm_tex_region { unsafe { ::std::mem::zeroed() } }
}


/**
 * Hardware lock.
 *
 * The lock structure is a simple cache-line aligned integer.  To avoid
 * processor bus contention on a multiprocessor system, there should not be any
 * other data stored in the same cache line.
 */
#[repr(C)]
pub struct drm_hw_lock {
	pub lock: VolatileCell<c_uint>,		/**< lock variable */
	pub padding: [c_char; 60]
}
impl ::std::default::Default for drm_hw_lock {
    fn default() -> drm_hw_lock { unsafe { ::std::mem::zeroed() } }
}

/**
 * DRM_IOCTL_VERSION ioctl argument type.
 *
 * \sa drmGetVersion().
 */
#[repr(C)]
pub struct drm_version {
	pub version_major: c_int,	  /**< Major version */
	pub version_minor: c_int,	  /**< Minor version */
	pub version_patchlevel: c_int,	  /**< Patch level */
	pub name_len: KernelSizeT,	  /**< Length of name buffer */
	pub name: *mut c_char,	  /**< Name of driver */
	pub date_len: KernelSizeT,	  /**< Length of date buffer */
	pub date: *mut c_char,	  /**< User-space buffer to hold date */
	pub desc_len: KernelSizeT,	  /**< Length of desc buffer */
	pub desc: *mut c_char
}
impl ::std::default::Default for drm_version {
    fn default() -> drm_version { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_GET_UNIQUE ioctl argument type.
 *
 * \sa drmGetBusid() and drmSetBusId().
 */
#[repr(C)]
pub struct drm_unique {
	pub unique_len: KernelSizeT,	  /**< Length of unique */
	pub unique: *mut c_char
}
impl ::std::default::Default for drm_unique {
    fn default() -> drm_unique { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct drm_list {
	pub count: c_int,		  /**< Length of user-space structures */
	pub version: *mut drm_version
}
impl ::std::default::Default for drm_list {
    fn default() -> drm_list { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct drm_block {
	pub unused: c_int
}
impl ::std::default::Default for drm_block {
    fn default() -> drm_block { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub enum drm_control_func {
    DRM_ADD_COMMAND,
    DRM_RM_COMMAND,
    DRM_INST_HANDLER,
    DRM_UNINST_HANDLER
}

/**
 * DRM_IOCTL_CONTROL ioctl argument type.
 *
 * \sa drmCtlInstHandler() and drmCtlUninstHandler().
 */
#[repr(C)]
pub struct drm_control {
	pub func: drm_control_func,
	pub irq: c_int
}
impl ::std::default::Default for drm_control {
    fn default() -> drm_control { unsafe { ::std::mem::zeroed() } }
}


/**
 * Type of memory to map.
 */
 #[repr(C)]
pub enum drm_map_type {
	_DRM_FRAME_BUFFER = 0,	  /**< WC (no caching), no core dump */
	_DRM_REGISTERS = 1,	  /**< no caching, no core dump */
	_DRM_SHM = 2,		  /**< shared, cached */
	_DRM_AGP = 3,		  /**< AGP/GART */
	_DRM_SCATTER_GATHER = 4,  /**< Scatter/gather memory for PCI DMA */
	_DRM_CONSISTENT = 5
}

/**
 * Memory mapping flags.
 */
 #[repr(C)]
pub enum drm_map_flags {
	_DRM_RESTRICTED = 0x01,	     /**< Cannot be mapped to user-virtual */
	_DRM_READ_ONLY = 0x02,
	_DRM_LOCKED = 0x04,	     /**< shared, cached, locked */
	_DRM_KERNEL = 0x08,	     /**< kernel requires access */
	_DRM_WRITE_COMBINING = 0x10, /**< use write-combining if available */
	_DRM_CONTAINS_LOCK = 0x20,   /**< SHM page that contains lock */
	_DRM_REMOVABLE = 0x40,	     /**< Removable mapping */
	_DRM_DRIVER = 0x80
}

#[repr(C)]
pub struct drm_ctx_priv_map {
	ctx_id: c_uint,	 /**< Context requesting private mapping */
	handle: *mut c_void
}
impl ::std::default::Default for drm_ctx_priv_map {
    fn default() -> drm_ctx_priv_map { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_GET_MAP, DRM_IOCTL_ADD_MAP and DRM_IOCTL_RM_MAP ioctls
 * argument type.
 *
 * \sa drmAddMap().
 */
#[repr(C)]
pub struct drm_map {
	offset: c_ulong,	 /**< Requested physical address (0 for SAREA)*/
	size: c_ulong,	 /**< Requested physical size (bytes) */
	map_type: drm_map_type,	 /**< Type of memory to map */
	flags: drm_map_flags,	 /**< Flags */
	handle: *mut c_void,		 /**< User-space: "Handle" to pass to mmap() */
				 /**< Kernel-space: kernel-virtual address */
	mtrr: c_int
}
impl ::std::default::Default for drm_map {
    fn default() -> drm_map { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_GET_CLIENT ioctl argument type.
 */
#[repr(C)]
pub struct drm_client {
	pub idx: c_int,		/**< Which client desired? */
	pub auth: c_int,		/**< Is client authenticated? */
	pub pid: c_ulong,	/**< Process ID */
	pub uid: c_ulong,	/**< User ID */
	pub magic: c_ulong,	/**< Magic */
	pub iocs: c_ulong
}
impl ::std::default::Default for drm_client {
    fn default() -> drm_client { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub enum drm_stat_type {
	_DRM_STAT_LOCK,
	_DRM_STAT_OPENS,
	_DRM_STAT_CLOSES,
	_DRM_STAT_IOCTLS,
	_DRM_STAT_LOCKS,
	_DRM_STAT_UNLOCKS,
	_DRM_STAT_VALUE,	/**< Generic value */
	_DRM_STAT_BYTE,		/**< Generic byte counter (1024bytes/K) */
	_DRM_STAT_COUNT,	/**< Generic non-byte counter (1000/k) */

	_DRM_STAT_IRQ,		/**< IRQ */
	_DRM_STAT_PRIMARY,	/**< Primary DMA bytes */
	_DRM_STAT_SECONDARY,	/**< Secondary DMA bytes */
	_DRM_STAT_DMA,		/**< DMA */
	_DRM_STAT_SPECIAL,	/**< Special DMA (e.g., priority or polled) */
	_DRM_STAT_MISSED
}

#[repr(C)]
pub struct drm_stats_data {
    value: c_ulong,
    stat_type: drm_stat_type
}
impl ::std::default::Default for drm_stats_data {
    fn default() -> drm_stats_data { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_GET_STATS ioctl argument type.
 */
#[repr(C)]
pub struct drm_stats {
	pub count: c_ulong,
	pub data: [drm_stats_data; 15]
}
impl ::std::default::Default for drm_stats {
    fn default() -> drm_stats { unsafe { ::std::mem::zeroed() } }
}


/**
 * Hardware locking flags.
 */
#[repr(C)]
pub enum drm_lock_flags {
	_DRM_LOCK_READY = 0x01,	     /**< Wait until hardware is ready for DMA */
	_DRM_LOCK_QUIESCENT = 0x02,  /**< Wait until hardware quiescent */
	_DRM_LOCK_FLUSH = 0x04,	     /**< Flush this context's DMA queue first */
	_DRM_LOCK_FLUSH_ALL = 0x08,  /**< Flush all DMA queues first */
	/* These *HALT* flags aren't supported yet
	   -- they will be used to support the
	   full-screen DGA-like mode. */
	_DRM_HALT_ALL_QUEUES = 0x10, /**< Halt all current and future queues */
	_DRM_HALT_CUR_QUEUES = 0x20
}

/**
 * DRM_IOCTL_LOCK, DRM_IOCTL_UNLOCK and DRM_IOCTL_FINISH ioctl argument type.
 *
 * \sa drmGetLock() and drmUnlock().
 */
#[repr(C)]
pub struct drm_lock {
	pub context: c_int,
	pub flags: drm_lock_flags
}
impl ::std::default::Default for drm_lock {
    fn default() -> drm_lock { unsafe { ::std::mem::zeroed() } }
}


/**
 * DMA flags
 *
 * \warning
 * These values \e must match xf86drm.h.
 *
 * \sa drm_dma.
 */
#[repr(C)]
pub enum drm_dma_flags {
	/* Flags for DMA buffer dispatch */
	_DRM_DMA_BLOCK = 0x01,	      /**<
				       * Block until buffer dispatched.
				       *
				       * \note The buffer may not yet have
				       * been processed by the hardware --
				       * getting a hardware lock with the
				       * hardware quiescent will ensure
				       * that the buffer has been
				       * processed.
				       */
	_DRM_DMA_WHILE_LOCKED = 0x02, /**< Dispatch while lock held */
	_DRM_DMA_PRIORITY = 0x04,     /**< High priority dispatch */

	/* Flags for DMA buffer request */
	_DRM_DMA_WAIT = 0x10,	      /**< Wait for free buffers */
	_DRM_DMA_SMALLER_OK = 0x20,   /**< Smaller-than-requested buffers OK */
	_DRM_DMA_LARGER_OK = 0x40
}

#[repr(C)]
pub enum drm_buf_desc_flags {
    _DRM_PAGE_ALIGN = 0x01,	/**< Align on page boundaries for DMA */
    _DRM_AGP_BUFFER = 0x02,	/**< Buffer is in AGP space */
    _DRM_SG_BUFFER = 0x04,	/**< Scatter/gather memory buffer */
    _DRM_FB_BUFFER = 0x08,	/**< Buffer is in frame buffer */
    _DRM_PCI_BUFFER_RO = 0x10
}

/**
 * DRM_IOCTL_ADD_BUFS and DRM_IOCTL_MARK_BUFS ioctl argument type.
 *
 * \sa drmAddBufs().
 */
#[repr(C)]
pub struct drm_buf_desc {
	pub count: c_int,		 /**< Number of buffers of this size */
	pub size: c_int,		 /**< Size in bytes */
	pub low_mark: c_int,		 /**< Low water mark */
	pub high_mark: c_int,		 /**< High water mark */
	pub flags: drm_buf_desc_flags,
	pub agp_start: c_ulong
}
impl ::std::default::Default for drm_buf_desc {
    fn default() -> drm_buf_desc { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_INFO_BUFS ioctl argument type.
 */
#[repr(C)]
pub struct drm_buf_info {
	pub count: c_int,		/**< Entries in list */
	pub list: *mut drm_buf_desc
}
impl ::std::default::Default for drm_buf_info {
    fn default() -> drm_buf_info { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_FREE_BUFS ioctl argument type.
 */
#[repr(C)]
pub struct drm_buf_free {
	pub count: c_int,
	pub list: *mut c_int
}
impl ::std::default::Default for drm_buf_free {
    fn default() -> drm_buf_free { unsafe { ::std::mem::zeroed() } }
}


/**
 * Buffer information
 *
 * \sa drm_buf_map.
 */
#[repr(C)]
pub struct drm_buf_pub {
	pub idx: c_int,		       /**< Index into the master buffer list */
	pub total: c_int,		       /**< Buffer size */
	pub used: c_int,		       /**< Amount of buffer in use (for DMA) */
	pub address: *mut c_void
}
impl ::std::default::Default for drm_buf_pub {
    fn default() -> drm_buf_pub { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_MAP_BUFS ioctl argument type.
 */
#[repr(C)]
pub struct drm_buf_map {
	pub count: c_int,		/**< Length of the buffer list */
	pub virtual_address: *mut c_void,		/**< Mmap'd area in user-virtual */
	pub list: *mut drm_buf_pub
}
impl ::std::default::Default for drm_buf_map {
    fn default() -> drm_buf_map { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_DMA ioctl argument type.
 *
 * Indices here refer to the offset into the buffer list in drm_buf_get.
 *
 * \sa drmDMA().
 */
#[repr(C)]
pub struct drm_dma {
	pub context: c_int,			  /**< Context handle */
	pub send_count: c_int,			  /**< Number of buffers to send */
	pub send_indices: *mut c_int,	  /**< List of handles to buffers */
	pub send_sizes: *mut c_int,		  /**< Lengths of data to send */
	pub flags: drm_dma_flags,	  /**< Flags */
	pub request_count: c_int,		  /**< Number of buffers requested */
	pub request_size: c_int,		  /**< Desired size for buffers */
	pub request_indices: *mut c_int,	  /**< Buffer information */
	pub request_sizes: *mut c_int,
	pub granted_count: c_int
}
impl ::std::default::Default for drm_dma {
    fn default() -> drm_dma { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub enum drm_ctx_flags {
	_DRM_CONTEXT_PRESERVED = 0x01,
	_DRM_CONTEXT_2DONLY = 0x02
}

/**
 * DRM_IOCTL_ADD_CTX ioctl argument type.
 *
 * \sa drmCreateContext() and drmDestroyContext().
 */
#[repr(C)]
pub struct drm_ctx {
	pub handle: drm_context_t,
	pub flags: drm_ctx_flags
}
impl ::std::default::Default for drm_ctx {
    fn default() -> drm_ctx { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_RES_CTX ioctl argument type.
 */
#[repr(C)]
pub struct drm_ctx_res {
	pub count: c_int,
	pub contexts: *mut drm_ctx
}
impl ::std::default::Default for drm_ctx_res {
    fn default() -> drm_ctx_res { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_ADD_DRAW and DRM_IOCTL_RM_DRAW ioctl argument type.
 */
struct drm_draw {
	pub handle: drm_drawable_t
}

/**
 * DRM_IOCTL_UPDATE_DRAW ioctl argument type.
 */
#[repr(C)]
pub enum drm_drawable_info_type_t {
	DRM_DRAWABLE_CLIPRECTS
}

#[repr(C)]
pub struct drm_update_draw {
	pub handle: drm_drawable_t,
	pub update_type: c_uint,
	pub num: c_uint,
	pub data: c_ulonglong
}
impl ::std::default::Default for drm_update_draw {
    fn default() -> drm_update_draw { unsafe { ::std::mem::zeroed() } }
}

/**
 * DRM_IOCTL_GET_MAGIC and DRM_IOCTL_AUTH_MAGIC ioctl argument type.
 */
#[repr(C)]
pub struct drm_auth {
	pub magic: drm_magic_t
}
impl ::std::default::Default for drm_auth {
    fn default() -> drm_auth { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_IRQ_BUSID ioctl argument type.
 *
 * \sa drmGetInterruptFromBusID().
 */
#[repr(C)]
pub struct drm_irq_busid {
	pub irq: c_int,	/**< IRQ number */
	pub busnum: c_int,	/**< bus number */
	pub devnum: c_int,	/**< device number */
	pub funcnum: c_int
}
impl ::std::default::Default for drm_irq_busid {
    fn default() -> drm_irq_busid { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub enum drm_vblank_seq_type {
	_DRM_VBLANK_ABSOLUTE = 0x0,	/**< Wait for specific vblank sequence number */
	_DRM_VBLANK_RELATIVE = 0x1,	/**< Wait for given number of vblanks */
	/* bits 1-6 are reserved for high crtcs */
	_DRM_VBLANK_HIGH_CRTC_MASK = 0x0000003e,
	_DRM_VBLANK_EVENT = 0x4000000,   /**< Send event instead of blocking */
	_DRM_VBLANK_FLIP = 0x8000000,   /**< Scheduled buffer swap should flip */
	_DRM_VBLANK_NEXTONMISS = 0x10000000,	/**< If missed, wait for next vblank */
	_DRM_VBLANK_SECONDARY = 0x20000000,	/**< Secondary display controller */
	_DRM_VBLANK_SIGNAL = 0x40000000
}
impl drm_vblank_seq_type {
    fn from_u32(n: u32) -> drm_vblank_seq_type {
        match n {
            0x0         => drm_vblank_seq_type::_DRM_VBLANK_ABSOLUTE,
            0x1         => drm_vblank_seq_type::_DRM_VBLANK_RELATIVE,
            0x0000003e  => drm_vblank_seq_type::_DRM_VBLANK_HIGH_CRTC_MASK,
            0x04000000  => drm_vblank_seq_type::_DRM_VBLANK_EVENT,
            0x08000000  => drm_vblank_seq_type::_DRM_VBLANK_FLIP,
            0x10000000  => drm_vblank_seq_type::_DRM_VBLANK_NEXTONMISS,
            0x20000000  => drm_vblank_seq_type::_DRM_VBLANK_SECONDARY,
            0x40000000  => drm_vblank_seq_type::_DRM_VBLANK_SIGNAL,
            _           => drm_vblank_seq_type::_DRM_VBLANK_ABSOLUTE
        }
    }
}
pub const _DRM_VBLANK_HIGH_CRTC_SHIFT: c_int = 1;

#[repr(C)]
pub struct drm_wait_vblank_request {
	pub request_type: drm_vblank_seq_type,
	pub sequence: c_uint,
	pub signal: c_ulong,
}
impl ::std::default::Default for drm_wait_vblank_request {
    fn default() -> drm_wait_vblank_request { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
pub struct drm_wait_vblank_reply {
	pub reply_type: drm_vblank_seq_type,
	pub sequence: c_uint,
	pub tval_sec: c_long,
	pub tval_usec: c_long
}
impl ::std::default::Default for drm_wait_vblank_reply {
    fn default() -> drm_wait_vblank_reply { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_WAIT_VBLANK ioctl argument type.
 *
 * \sa drmWaitVBlank().
 */
#[repr(C)]
pub struct drm_wait_vblank {
	pub data: [u8; 24]
}
impl ::std::default::Default for drm_wait_vblank {
    fn default() -> drm_wait_vblank { unsafe { ::std::mem::zeroed() } }
}


impl drm_wait_vblank {
    #[cfg(target_pointer_width = "32")]
    fn request (&self) -> drm_wait_vblank_request {
        let req = drm_wait_vblank_request {
            request_type: drm_vblank_seq_type::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            signal: NativeEndian::read_u32(&self.data[9..16])
        };

        req
    }

    #[cfg(target_pointer_width = "32")]
    fn reply (&self) -> drm_wait_vblank_reply {
        let reply = drm_wait_vblank_reply {
            reply_type: drm_vblank_seq_type::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            tval_sec: NativeEndian::read_i32(&self.data[9..16]),
            tval_usec: NativeEndian::read_i32(&self.data[17..24])
        };

        reply
    }

    #[cfg(target_pointer_width = "64")]
    fn request (&self) -> drm_wait_vblank_request {
        let req = drm_wait_vblank_request {
            request_type: drm_vblank_seq_type::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            signal: NativeEndian::read_u64(&self.data[9..16])
        };

        req
    }

    #[cfg(target_pointer_width = "64")]
    fn reply (&self) -> drm_wait_vblank_reply {
        let reply = drm_wait_vblank_reply {
            reply_type: drm_vblank_seq_type::from_u32(NativeEndian::read_u32(&self.data[0..4])),
            sequence: NativeEndian::read_u32(&self.data[5..8]),
            tval_sec: NativeEndian::read_i64(&self.data[9..16]),
            tval_usec: NativeEndian::read_i64(&self.data[17..24])
        };

        reply
    }
}

pub const _DRM_PRE_MODESET: c_int = 1;
pub const _DRM_POST_MODESET: c_int = 2;

/**
 * DRM_IOCTL_MODESET_CTL ioctl argument type
 *
 * \sa drmModesetCtl().
 */
#[repr(C)]
pub struct drm_modeset_ctl {
    pub crtc: u32,
	pub cmd: u32
}
impl ::std::default::Default for drm_modeset_ctl {
    fn default() -> drm_modeset_ctl { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_AGP_ENABLE ioctl argument type.
 *
 * \sa drmAgpEnable().
 */
#[repr(C)]
pub struct drm_agp_mode {
	pub mode: c_ulong
}
impl ::std::default::Default for drm_agp_mode {
    fn default() -> drm_agp_mode { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_AGP_ALLOC and DRM_IOCTL_AGP_FREE ioctls argument type.
 *
 * \sa drmAgpAlloc() and drmAgpFree().
 */
#[repr(C)]
pub struct drm_agp_buffer {
	pub size: c_ulong,	/**< In bytes -- will round to page boundary */
	pub handle: c_ulong,	/**< Used for binding / unbinding */
	pub buffer_type: c_ulong,	/**< Type of memory to allocate */
	pub physical: c_ulong
}
impl ::std::default::Default for drm_agp_buffer {
    fn default() -> drm_agp_buffer { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_AGP_BIND and DRM_IOCTL_AGP_UNBIND ioctls argument type.
 *
 * \sa drmAgpBind() and drmAgpUnbind().
 */
#[repr(C)]
pub struct drm_agp_binding {
	pub handle: c_ulong,	/**< From drm_agp_buffer */
	pub offset: c_ulong
}
impl ::std::default::Default for drm_agp_binding {
    fn default() -> drm_agp_binding { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_AGP_INFO ioctl argument type.
 *
 * \sa drmAgpVersionMajor(), drmAgpVersionMinor(), drmAgpGetMode(),
 * drmAgpBase(), drmAgpSize(), drmAgpMemoryUsed(), drmAgpMemoryAvail(),
 * drmAgpVendorId() and drmAgpDeviceId().
 */
#[repr(C)]
pub struct drm_agp_info {
	pub agp_version_major: c_int,
	pub agp_version_minor: c_int,
	pub mode: c_ulong,
	pub aperture_base: c_ulong,	/* physical address */
	pub aperture_size: c_ulong,	/* bytes */
	pub memory_allowed: c_ulong,	/* bytes */
	pub memory_used: c_ulong,

	/* PCI information */
	pub id_vendor: c_ushort,
	pub id_device: c_ushort
}
impl ::std::default::Default for drm_agp_info {
    fn default() -> drm_agp_info { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_SG_ALLOC ioctl argument type.
 */
#[repr(C)]
pub struct drm_scatter_gather {
	pub size: c_ulong,	/**< In bytes -- will round to page boundary */
	pub handle: c_ulong
}
impl ::std::default::Default for drm_scatter_gather {
    fn default() -> drm_scatter_gather { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_IOCTL_SET_VERSION ioctl argument type.
 */
#[repr(C)]
pub struct drm_set_version {
	pub drm_di_major: c_int,
	pub drm_di_minor: c_int,
	pub drm_dd_major: c_int,
	pub drm_dd_minor: c_int
}
impl ::std::default::Default for drm_set_version {
    fn default() -> drm_set_version { unsafe { ::std::mem::zeroed() } }
}


/** DRM_IOCTL_GEM_CLOSE ioctl argument type */
struct drm_gem_close {
	/** Handle of the object to be closed. */
	pub handle: u32,
	pub pad: u32
}

/** DRM_IOCTL_GEM_FLINK ioctl argument type */
#[repr(C)]
pub struct drm_gem_flink {
	/** Handle for the object being named */
	pub handle: u32,

	/** Returned global name */
	pub name: u32
}
impl ::std::default::Default for drm_gem_flink {
    fn default() -> drm_gem_flink { unsafe { ::std::mem::zeroed() } }
}


/** DRM_IOCTL_GEM_OPEN ioctl argument type */
#[repr(C)]
pub struct drm_gem_open {
	/** Name of object being opened */
	pub name: u32,

	/** Returned handle for the object */
	pub handle: u32,

	/** Returned size of the object */
	pub size: u64
}
impl ::std::default::Default for drm_gem_open {
    fn default() -> drm_gem_open { unsafe { ::std::mem::zeroed() } }
}


pub const DRM_CAP_DUMB_BUFFER: c_int = 0x1;
pub const DRM_CAP_VBLANK_HIGH_CRTC: c_int = 0x2;
pub const DRM_CAP_DUMB_PREFERRED_DEPTH: c_int = 0x3;
pub const DRM_CAP_DUMB_PREFER_SHADOW: c_int = 0x4;
pub const DRM_CAP_PRIME: c_int = 0x5;
pub const  DRM_PRIME_CAP_IMPORT: c_int = 0x1;
pub const  DRM_PRIME_CAP_EXPORT: c_int = 0x2;
pub const DRM_CAP_TIMESTAMP_MONOTONIC: c_int = 0x6;
pub const DRM_CAP_ASYNC_PAGE_FLIP: c_int = 0x7;
/*
 * The CURSOR_WIDTH and CURSOR_HEIGHT capabilities return a valid widthxheight
 * combination for the hardware cursor. The intention is that a hardware
 * agnostic userspace can query a cursor plane size to use.
 *
 * Note that the cross-driver contract is to merely return a valid size;
 * drivers are free to attach another meaning on top, eg. i915 returns the
 * maximum plane size.
 */
pub const DRM_CAP_CURSOR_WIDTH: c_int = 0x8;
pub const DRM_CAP_CURSOR_HEIGHT: c_int = 0x9;
pub const DRM_CAP_ADDFB2_MODIFIERS: c_int = 0x10;

/** DRM_IOCTL_GET_CAP ioctl argument type */
#[repr(C)]
pub struct drm_get_cap {
	pub capability: u64,
	pub value: u64
}
impl ::std::default::Default for drm_get_cap {
    fn default() -> drm_get_cap { unsafe { ::std::mem::zeroed() } }
}


/**
 * DRM_CLIENT_CAP_STEREO_3D
 *
 * if set to 1, the DRM core will expose the stereo 3D capabilities of the
 * monitor by advertising the supported 3D layouts in the flags of struct
 * drm_mode_modeinfo.
 */
pub const DRM_CLIENT_CAP_STEREO_3D: c_int = 1;

/**
 * DRM_CLIENT_CAP_UNIVERSAL_PLANES
 *
 * If set to 1, the DRM core will expose all planes (overlay, primary, and
 * cursor) to userspace.
 */
pub const DRM_CLIENT_CAP_UNIVERSAL_PLANES: c_int = 2;

/**
 * DRM_CLIENT_CAP_ATOMIC
 *
 * If set to 1, the DRM core will expose atomic properties to userspace
 */
pub const DRM_CLIENT_CAP_ATOMIC: c_int = 3;

/** DRM_IOCTL_SET_CLIENT_CAP ioctl argument type */
#[repr(C)]
pub struct drm_set_client_cap {
	pub capability: u64,
	pub value: u64
}
impl ::std::default::Default for drm_set_client_cap {
    fn default() -> drm_set_client_cap { unsafe { ::std::mem::zeroed() } }
}


pub const DRM_RDWR: c_int = O_RDWR;
// const DRM_CLOEXEC: c_int = O_CLOEXEC;
#[repr(C)]
pub struct drm_prime_handle {
	pub handle: u32,

	/** Flags.. only applicable for handle->fd */
	pub flags: u32,

	/** Returned dmabuf file descriptor */
	pub fd: i32
}
impl ::std::default::Default for drm_prime_handle {
    fn default() -> drm_prime_handle { unsafe { ::std::mem::zeroed() } }
}


// #include "drm_mode.h"

/**
 * Device specific ioctls should only be in their respective headers
 * The device specific ioctl range is from 0x40 to 0x9f.
 * Generic IOCTLS restart at 0xA0.
 *
 * \sa drmCommandNone(), drmCommandRead(), drmCommandWrite(), and
 * drmCommandReadWrite().
 */
pub const DRM_COMMAND_BASE: c_int = 0x40;
pub const DRM_COMMAND_END: c_int = 0xA0;

/**
 * Header for events written back to userspace on the drm fd.  The
 * type defines the type of event, the length specifies the total
 * length of the event (including the header), and user_data is
 * typically a 64 bit value passed with the ioctl that triggered the
 * event.  A read on the drm fd will always only return complete
 * events, that is, if for example the read buffer is 100 bytes, and
 * there are two 64 byte events pending, only one will be returned.
 *
 * Event types 0 - 0x7fffffff are generic drm events, 0x80000000 and
 * up are chipset specific.
 */
#[repr(C)]
pub struct drm_event {
	pub event_type: u32,
	pub length: u32
}
impl ::std::default::Default for drm_event {
    fn default() -> drm_event { unsafe { ::std::mem::zeroed() } }
}


pub const DRM_EVENT_VBLANK: c_int = 0x01;
pub const DRM_EVENT_FLIP_COMPLETE: c_int = 0x02;

#[repr(C)]
pub struct drm_event_vblank {
	pub base: drm_event,
	pub user_data: u64,
	pub tv_sec: u32,
	pub tv_usec: u32,
	pub sequence: u32,
	pub reserved: u32
}
impl ::std::default::Default for drm_event_vblank {
    fn default() -> drm_event_vblank { unsafe { ::std::mem::zeroed() } }
}


/* typedef area */
type drm_clip_rect_t = drm_clip_rect;
type drm_drawable_info_t = drm_drawable_info;
type drm_tex_region_t = drm_tex_region;
type drm_hw_lock_t = drm_hw_lock;
type drm_version_t = drm_version;
type drm_unique_t = drm_unique;
type drm_list_t = drm_list;
type drm_block_t = drm_block;
type drm_control_t = drm_control;
type drm_map_type_t = drm_map_type;
type drm_map_flags_t = drm_map_flags;
type drm_ctx_priv_map_t = drm_ctx_priv_map;
type drm_map_t = drm_map;
type drm_client_t = drm_client;
type drm_stat_type_t = drm_stat_type;
type drm_stats_t = drm_stats;
type drm_lock_flags_t = drm_lock_flags;
type drm_lock_t = drm_lock;
type drm_dma_flags_t = drm_dma_flags;
type drm_buf_desc_t = drm_buf_desc;
type drm_buf_info_t = drm_buf_info;
type drm_buf_free_t = drm_buf_free;
type drm_buf_pub_t = drm_buf_pub;
type drm_buf_map_t = drm_buf_map;
type drm_dma_t = drm_dma;
type drm_wait_vblank_t = drm_wait_vblank;
type drm_agp_mode_t = drm_agp_mode;
type drm_ctx_flags_t = drm_ctx_flags;
type drm_ctx_t = drm_ctx;
type drm_ctx_res_t = drm_ctx_res;
type drm_draw_t = drm_draw;
type drm_update_draw_t = drm_update_draw;
type drm_auth_t = drm_auth;
type drm_irq_busid_t = drm_irq_busid;
type drm_vblank_seq_type_t = drm_vblank_seq_type;

type drm_agp_buffer_t = drm_agp_buffer;
type drm_agp_binding_t = drm_agp_binding;
type drm_agp_info_t = drm_agp_info;
type drm_scatter_gather_t = drm_scatter_gather;
type drm_set_version_t = drm_set_version;
