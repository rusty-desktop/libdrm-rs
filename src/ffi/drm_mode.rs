/*
 * Copyright (c) 2007 Dave Airlie <airlied@linux.ie>
 * Copyright (c) 2007 Jakob Bornecrantz <wallbraker@gmail.com>
 * Copyright (c) 2008 Red Hat Inc.
 * Copyright (c) 2007-2008 Tungsten Graphics, Inc., Cedar Park, TX., USA
 * Copyright (c) 2007-2008 Intel Corporation
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
 */

use libc::*;

pub const DRM_DISPLAY_INFO_LEN: c_int = 32;
pub const DRM_CONNECTOR_NAME_LEN: c_int = 32;
pub const DRM_DISPLAY_MODE_LEN: c_int = 32;
pub const DRM_PROP_NAME_LEN: c_int = 32;

pub const DRM_MODE_TYPE_BUILTIN: c_int = (1<<0);
pub const DRM_MODE_TYPE_CLOCK_C: c_int = ((1<<1) | DRM_MODE_TYPE_BUILTIN);
pub const DRM_MODE_TYPE_CRTC_C: c_int = ((1<<2) | DRM_MODE_TYPE_BUILTIN);
pub const DRM_MODE_TYPE_PREFERRED: c_int = (1<<3);
pub const DRM_MODE_TYPE_DEFAULT: c_int = (1<<4);
pub const DRM_MODE_TYPE_USERDEF: c_int = (1<<5);
pub const DRM_MODE_TYPE_DRIVER: c_int = (1<<6);

/* Video mode flags */
/* bit compatible with the xorg definitions. */
pub const DRM_MODE_FLAG_PHSYNC: c_int = (1<<0);
pub const DRM_MODE_FLAG_NHSYNC: c_int = (1<<1);
pub const DRM_MODE_FLAG_PVSYNC: c_int = (1<<2);
pub const DRM_MODE_FLAG_NVSYNC: c_int = (1<<3);
pub const DRM_MODE_FLAG_INTERLACE: c_int = (1<<4);
pub const DRM_MODE_FLAG_DBLSCAN: c_int = (1<<5);
pub const DRM_MODE_FLAG_CSYNC: c_int = (1<<6);
pub const DRM_MODE_FLAG_PCSYNC: c_int = (1<<7);
pub const DRM_MODE_FLAG_NCSYNC: c_int = (1<<8);
pub const DRM_MODE_FLAG_HSKEW: c_int = (1<<9); /* hskew provided */
pub const DRM_MODE_FLAG_BCAST: c_int = (1<<10);
pub const DRM_MODE_FLAG_PIXMUX: c_int = (1<<11);
pub const DRM_MODE_FLAG_DBLCLK: c_int = (1<<12);
pub const DRM_MODE_FLAG_CLKDIV2: c_int = (1<<13);
 /*
  * When adding a new stereo mode don't forget to adjust DRM_MODE_FLAGS_3D_MAX
  * (define not exposed to user space).
  */
pub const DRM_MODE_FLAG_3D_MASK: c_int = (0x1f<<14);
pub const DRM_MODE_FLAG_3D_NONE: c_int = (0<<14);
pub const DRM_MODE_FLAG_3D_FRAME_PACKING: c_int = (1<<14);
pub const DRM_MODE_FLAG_3D_FIELD_ALTERNATIVE: c_int = (2<<14);
pub const DRM_MODE_FLAG_3D_LINE_ALTERNATIVE: c_int = (3<<14);
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_FULL: c_int = (4<<14);
pub const DRM_MODE_FLAG_3D_L_DEPTH: c_int = (5<<14);
pub const DRM_MODE_FLAG_3D_L_DEPTH_GFX_GFX_DEPTH: c_int = (6<<14);
pub const DRM_MODE_FLAG_3D_TOP_AND_BOTTOM: c_int = (7<<14);
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_HALF: c_int = (8<<14);


/* DPMS flags */
/* bit compatible with the xorg definitions. */
pub const DRM_MODE_DPMS_ON: c_int = 0;
pub const DRM_MODE_DPMS_STANDBY: c_int = 1;
pub const DRM_MODE_DPMS_SUSPEND: c_int = 2;
pub const DRM_MODE_DPMS_OFF: c_int = 3;

/* Scaling mode options */
pub const DRM_MODE_SCALE_NONE: c_int = 0; /* Unmodified timing (display or
					     software can still scale) */
pub const DRM_MODE_SCALE_FULLSCREEN: c_int = 1; /* Full screen, ignore aspect */
pub const DRM_MODE_SCALE_CENTER: c_int = 2; /* Centered, no scaling */
pub const DRM_MODE_SCALE_ASPECT: c_int = 3; /* Full screen, preserve aspect */

/* Picture aspect ratio options */
pub const DRM_MODE_PICTURE_ASPECT_NONE: c_int = 0;
pub const DRM_MODE_PICTURE_ASPECT_4_3: c_int = 1;
pub const DRM_MODE_PICTURE_ASPECT_16_9: c_int = 2;

/* Dithering mode options */
pub const DRM_MODE_DITHERING_OFF: c_int = 0;
pub const DRM_MODE_DITHERING_ON: c_int = 1;
pub const DRM_MODE_DITHERING_AUTO: c_int = 2;

/* Dirty info options */
pub const DRM_MODE_DIRTY_OFF: c_int = 0;
pub const DRM_MODE_DIRTY_ON: c_int = 1;
pub const DRM_MODE_DIRTY_ANNOTATE: c_int = 2;

#[repr(C)]
pub struct drm_mode_property_enum {
	value: u64,
	name: [c_char; DRM_PROP_NAME_LEN as usize]
}
impl ::std::default::Default for drm_mode_property_enum {
    fn default() -> drm_mode_property_enum { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
pub struct drm_mode_create_dumb {
    pub height: u32,
    pub width: u32,
    pub bpp: u32,
    pub flags: u32,
    /* handle, pitch, size will be returned */
    pub handle: u32,
    pub pitch: u32,
    pub size: u64
}
impl ::std::default::Default for drm_mode_create_dumb {
    fn default() -> drm_mode_create_dumb { unsafe { ::std::mem::zeroed() } }
}

/* set up for mmap of a dumb scanout buffer */
#[repr(C)]
pub struct drm_mode_map_dumb {
    /** Handle for the object being mapped. */
    pub handle: u32,
    pub pad: u32,
    /**
    * Fake offset to use for subsequent mmap call
    *
    * This is a fixed-size type for 32/64 compatibility.
    */
    pub offset: u64
}
impl ::std::default::Default for drm_mode_map_dumb {
    fn default() -> drm_mode_map_dumb { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
pub struct drm_mode_destroy_dumb {
    pub handle: u32
}
impl ::std::default::Default for drm_mode_destroy_dumb {
    fn default() -> drm_mode_destroy_dumb { unsafe { ::std::mem::zeroed() } }
}

