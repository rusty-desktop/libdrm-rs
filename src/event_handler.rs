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

//! In C `drmHandleEvent` takes as an argument pointer to context containing two optional pointers
//! to callback functions handling v-blanks or page flips. It reads events from DRM device and call
//! appropriate callback with no other state than passed to `drmModePageFlip`. Any other state must
//! be global.
//!
//! In Rust we try avoid forcing users to implement global states or singletons, however passing
//! state by `drmModePageFlip` would make interface unsafe. We want to have safe interface but also
//! provide users context for their callbacks without forcing them to implement global state.
//!
//! This problem is solved by taking context in `handle_event` function wrapping `drmHandleEvent`
//! and storing it in thread local storage for time of processing events. User callbacks are then
//! called within this context.

use libc;
use std::cell::RefCell;
use std::os::unix::io;

use ffi;

pub const DRM_CONTEXT_VERSION: libc::c_int = 2; /**< Desired DRM event context version */

/// Trait for contexts passed to `handle_event`.
pub trait EventContext {
    fn vblank_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32);
    fn page_flip_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32);
}

/// Thread local storage for event contexts.
thread_local!(static CONTEXT: RefCell<Option<Box<EventContext>>> = RefCell::new(None));

/// Helper function for handling v-blanks.
extern "C" fn vblank_handler(fd: libc::c_int,
                             sequence: libc::c_uint,
                             tv_sec: libc::c_uint,
                             tv_usec: libc::c_uint,
                             user_data: *mut libc::c_void) {
    CONTEXT.with(|s| if let Some(ref mut context) = *s.borrow_mut() {
        context.vblank_handler(fd, sequence, tv_sec, tv_usec, user_data as i32);
    });
}

/// Helper function for handling page flips.
extern "C" fn page_flip_handler(fd: libc::c_int,
                                sequence: libc::c_uint,
                                tv_sec: libc::c_uint,
                                tv_usec: libc::c_uint,
                                user_data: *mut libc::c_void) {
    CONTEXT.with(|s| if let Some(ref mut context) = *s.borrow_mut() {
        context.page_flip_handler(fd, sequence, tv_sec, tv_usec, user_data as i32);
    });
}

/// Handle event from DRM device.
///
/// Counterpart for `drmHandleEvent`.
pub fn handle_event(fd: io::RawFd, context: Box<EventContext>) {
    CONTEXT.with(|s| *s.borrow_mut() = Some(context));

    let mut drm_context = ffi::xf86drm::drmEventContext::default();
    drm_context.version = DRM_CONTEXT_VERSION;
    drm_context.vblank_handler = vblank_handler;
    drm_context.page_flip_handler = page_flip_handler;
    unsafe {
        ffi::xf86drm::drmHandleEvent(fd, &drm_context);
    }

    CONTEXT.with(|s| *s.borrow_mut() = None);
}
