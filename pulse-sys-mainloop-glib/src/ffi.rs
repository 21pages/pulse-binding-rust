use crate::pa_glib_mainloop;
use glib_sys::GMainContext;
use libpulse_sys::mainloop::api::pa_mainloop_api;
use std::sync::Arc;

pub struct PulseGlibFunctions {
    pub pa_glib_mainloop_new: unsafe extern "C" fn(*mut GMainContext) -> *mut pa_glib_mainloop,
    pub pa_glib_mainloop_free: unsafe extern "C" fn(*mut pa_glib_mainloop),
    pub pa_glib_mainloop_get_api:
        unsafe extern "C" fn(*const pa_glib_mainloop) -> *const pa_mainloop_api,
}

impl PulseGlibFunctions {
    pub(crate) unsafe fn load(lib: &libloading::Library) -> Result<Arc<Self>, libloading::Error> {
        Ok(Arc::new(Self {
            pa_glib_mainloop_new: *lib.get(b"pa_glib_mainloop_new\0")?,
            pa_glib_mainloop_free: *lib.get(b"pa_glib_mainloop_free\0")?,
            pa_glib_mainloop_get_api: *lib.get(b"pa_glib_mainloop_get_api\0")?,
        }))
    }
}

static FUNCTIONS: once_cell::sync::OnceCell<Arc<PulseGlibFunctions>> =
    once_cell::sync::OnceCell::new();

pub fn get_functions() -> Option<Arc<PulseGlibFunctions>> {
    init().ok()?;
    FUNCTIONS.get().cloned()
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    if FUNCTIONS.get().is_some() {
        return Ok(());
    }

    let lib = unsafe { libloading::Library::new("libpulse-mainloop-glib.so.0")? };
    let functions = unsafe { PulseGlibFunctions::load(&lib)? };
    FUNCTIONS
        .set(functions)
        .map_err(|_| "Failed to set functions")?;

    Ok(())
}
