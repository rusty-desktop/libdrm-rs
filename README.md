# libdrm-rs

This crate contains bindings for libdrm functionality in linux. This is a WIP.
Currently it exposes bindings for:

- drm.h
- drm_mode.h
- xf86drm.h
- xf86drmMode.h

This is a learning experience for me, so mistakes will be made. If you have any
suggestions, let me know.

The purpose of this crate is to provide low-level bindings that map directly to
the libraries in order to be able to maintain compatibility as new versions of
libdrm appear. I plan to work on safer, more idiomatic, higher level constructs
once I've tested this enough, but those abstractions will appear on another
crate. This will remain as close to 1:1 map as possible.

This was hand-made as bindgen had issues with the files and I didn't test other
generators.

As I said, this is a learning experience as I have very little experience with
Rust. If you see something wrong, please let me know.
