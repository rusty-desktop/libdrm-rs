use ffi;

/// Resources structure.
/// Can be obtained by call to `drm_mode::get_mode_resources`
pub struct Resources {
    pub resources: ffi::xf86drm_mode::drmModeResPtr,
}

impl Resources {
    pub fn get_count_fbs(&self) -> i32 {
        unsafe { (*self.resources).count_fbs }
    }

    pub fn get_count_crtcs(&self) -> i32 {
        unsafe { (*self.resources).count_crtcs }
    }

    pub fn get_count_connectors(&self) -> i32 {
        unsafe { (*self.resources).count_connectors }
    }

    pub fn get_count_encoders(&self) -> i32 {
        unsafe { (*self.resources).count_encoders }
    }
}
