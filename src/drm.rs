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
	x1: c_ushort,
	y1: c_ushort,
	x2: c_ushort,
	y2: c_ushort
}

/**
 * Drawable information.
 */
 #[repr(C)]
pub struct drm_drawable_info {
	num_rects: c_uint,
	rects: *mut drm_clip_rect
}

/**
 * Texture region,
 */
#[repr(C)]
pub struct drm_tex_region {
	next: c_uchar,
	prev: c_uchar,
	in_use: c_uchar,
	padding: c_uchar,
	age: c_uint
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
	lock: VolatileCell<c_uint>,		/**< lock variable */
	padding: [c_char; 60]
}

/**
 * DRM_IOCTL_VERSION ioctl argument type.
 *
 * \sa drmGetVersion().
 */
#[repr(C)]
pub struct drm_version {
	version_major: c_int,	  /**< Major version */
	version_minor: c_int,	  /**< Minor version */
	version_patchlevel: c_int,	  /**< Patch level */
	name_len: KernelSizeT,	  /**< Length of name buffer */
	name: *mut c_char,	  /**< Name of driver */
	date_len: KernelSizeT,	  /**< Length of date buffer */
	date: *mut c_char,	  /**< User-space buffer to hold date */
	desc_len: KernelSizeT,	  /**< Length of desc buffer */
	desc: *mut c_char
}

/**
 * DRM_IOCTL_GET_UNIQUE ioctl argument type.
 *
 * \sa drmGetBusid() and drmSetBusId().
 */
#[repr(C)]
pub struct drm_unique {
	unique_len: KernelSizeT,	  /**< Length of unique */
	unique: *mut c_char
}

#[repr(C)]
pub struct drm_list {
	count: c_int,		  /**< Length of user-space structures */
	version: *mut drm_version
}

#[repr(C)]
pub struct drm_block {
	unused: c_int
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
	func: drm_control_func,
	irq: c_int
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

/**
 * DRM_IOCTL_GET_CLIENT ioctl argument type.
 */
#[repr(C)]
pub struct drm_client {
	idx: c_int,		/**< Which client desired? */
	auth: c_int,		/**< Is client authenticated? */
	pid: c_ulong,	/**< Process ID */
	uid: c_ulong,	/**< User ID */
	magic: c_ulong,	/**< Magic */
	iocs: c_ulong
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

/**
 * DRM_IOCTL_GET_STATS ioctl argument type.
 */
#[repr(C)]
pub struct drm_stats {
	count: c_ulong,
	data: [drm_stats_data; 15]
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
	context: c_int,
	flags: drm_lock_flags
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
	count: c_int,		 /**< Number of buffers of this size */
	size: c_int,		 /**< Size in bytes */
	low_mark: c_int,		 /**< Low water mark */
	high_mark: c_int,		 /**< High water mark */
	flags: drm_buf_desc_flags,
	agp_start: c_ulong
}

/**
 * DRM_IOCTL_INFO_BUFS ioctl argument type.
 */
#[repr(C)]
pub struct drm_buf_info {
	count: c_int,		/**< Entries in list */
	list: *mut drm_buf_desc
}

/**
 * DRM_IOCTL_FREE_BUFS ioctl argument type.
 */
#[repr(C)]
pub struct drm_buf_free {
	count: c_int,
	list: *mut c_int
}

/**
 * Buffer information
 *
 * \sa drm_buf_map.
 */
#[repr(C)]
pub struct drm_buf_pub {
	idx: c_int,		       /**< Index into the master buffer list */
	total: c_int,		       /**< Buffer size */
	used: c_int,		       /**< Amount of buffer in use (for DMA) */
	address: *mut c_void
}

/**
 * DRM_IOCTL_MAP_BUFS ioctl argument type.
 */
#[repr(C)]
pub struct drm_buf_map {
	count: c_int,		/**< Length of the buffer list */
	virtual_address: *mut c_void,		/**< Mmap'd area in user-virtual */
	list: *mut drm_buf_pub
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
	context: c_int,			  /**< Context handle */
	send_count: c_int,			  /**< Number of buffers to send */
	send_indices: *mut c_int,	  /**< List of handles to buffers */
	send_sizes: *mut c_int,		  /**< Lengths of data to send */
	flags: drm_dma_flags,	  /**< Flags */
	request_count: c_int,		  /**< Number of buffers requested */
	request_size: c_int,		  /**< Desired size for buffers */
	request_indices: *mut c_int,	  /**< Buffer information */
	request_sizes: *mut c_int,
	granted_count: c_int
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
	handle: drm_context_t,
	flags: drm_ctx_flags
}

/**
 * DRM_IOCTL_RES_CTX ioctl argument type.
 */
#[repr(C)]
pub struct drm_ctx_res {
	count: c_int,
	contexts: *mut drm_ctx
}

/**
 * DRM_IOCTL_ADD_DRAW and DRM_IOCTL_RM_DRAW ioctl argument type.
 */
struct drm_draw {
	handle: drm_drawable_t
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
	handle: drm_drawable_t,
	update_type: c_uint,
	num: c_uint,
	data: c_ulonglong
}

/**
 * DRM_IOCTL_GET_MAGIC and DRM_IOCTL_AUTH_MAGIC ioctl argument type.
 */
#[repr(C)]
pub struct drm_auth {
	magic: drm_magic_t
}

/**
 * DRM_IOCTL_IRQ_BUSID ioctl argument type.
 *
 * \sa drmGetInterruptFromBusID().
 */
#[repr(C)]
pub struct drm_irq_busid {
	irq: c_int,	/**< IRQ number */
	busnum: c_int,	/**< bus number */
	devnum: c_int,	/**< device number */
	funcnum: c_int
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
	request_type: drm_vblank_seq_type,
	sequence: c_uint,
	signal: c_ulong,
}

#[repr(C)]
pub struct drm_wait_vblank_reply {
	reply_type: drm_vblank_seq_type,
	sequence: c_uint,
	tval_sec: c_long,
	tval_usec: c_long
}

/**
 * DRM_IOCTL_WAIT_VBLANK ioctl argument type.
 *
 * \sa drmWaitVBlank().
 */
#[repr(C)]
pub struct drm_wait_vblank {
	data: [u8; 24]
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
    crtc: u32,
	cmd: u32
}

/**
 * DRM_IOCTL_AGP_ENABLE ioctl argument type.
 *
 * \sa drmAgpEnable().
 */
#[repr(C)]
pub struct drm_agp_mode {
	mode: c_ulong
}

/**
 * DRM_IOCTL_AGP_ALLOC and DRM_IOCTL_AGP_FREE ioctls argument type.
 *
 * \sa drmAgpAlloc() and drmAgpFree().
 */
#[repr(C)]
pub struct drm_agp_buffer {
	size: c_ulong,	/**< In bytes -- will round to page boundary */
	handle: c_ulong,	/**< Used for binding / unbinding */
	buffer_type: c_ulong,	/**< Type of memory to allocate */
	physical: c_ulong
}

/**
 * DRM_IOCTL_AGP_BIND and DRM_IOCTL_AGP_UNBIND ioctls argument type.
 *
 * \sa drmAgpBind() and drmAgpUnbind().
 */
#[repr(C)]
pub struct drm_agp_binding {
	handle: c_ulong,	/**< From drm_agp_buffer */
	offset: c_ulong
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
	agp_version_major: c_int,
	agp_version_minor: c_int,
	mode: c_ulong,
	aperture_base: c_ulong,	/* physical address */
	aperture_size: c_ulong,	/* bytes */
	memory_allowed: c_ulong,	/* bytes */
	memory_used: c_ulong,

	/* PCI information */
	id_vendor: c_ushort,
	id_device: c_ushort
}

/**
 * DRM_IOCTL_SG_ALLOC ioctl argument type.
 */
#[repr(C)]
pub struct drm_scatter_gather {
	size: c_ulong,	/**< In bytes -- will round to page boundary */
	handle: c_ulong
}

/**
 * DRM_IOCTL_SET_VERSION ioctl argument type.
 */
#[repr(C)]
pub struct drm_set_version {
	drm_di_major: c_int,
	drm_di_minor: c_int,
	drm_dd_major: c_int,
	drm_dd_minor: c_int
}

/** DRM_IOCTL_GEM_CLOSE ioctl argument type */
struct drm_gem_close {
	/** Handle of the object to be closed. */
	handle: u32,
	pad: u32
}

/** DRM_IOCTL_GEM_FLINK ioctl argument type */
#[repr(C)]
pub struct drm_gem_flink {
	/** Handle for the object being named */
	handle: u32,

	/** Returned global name */
	name: u32
}

/** DRM_IOCTL_GEM_OPEN ioctl argument type */
#[repr(C)]
pub struct drm_gem_open {
	/** Name of object being opened */
	name: u32,

	/** Returned handle for the object */
	handle: u32,

	/** Returned size of the object */
	size: u64
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
	capability: u64,
	value: u64
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
	capability: u64,
	value: u64
}

pub const DRM_RDWR: c_int = O_RDWR;
// const DRM_CLOEXEC: c_int = O_CLOEXEC;
#[repr(C)]
pub struct drm_prime_handle {
	handle: u32,

	/** Flags.. only applicable for handle->fd */
	flags: u32,

	/** Returned dmabuf file descriptor */
	fd: i32
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
	event_type: u32,
	length: u32
}

pub const DRM_EVENT_VBLANK: c_int = 0x01;
pub const DRM_EVENT_FLIP_COMPLETE: c_int = 0x02;

#[repr(C)]
pub struct drm_event_vblank {
	base: drm_event,
	user_data: u64,
	tv_sec: u32,
	tv_usec: u32,
	sequence: u32,
	reserved: u32
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
