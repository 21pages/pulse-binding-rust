use libloading::Library;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

static PULSE_LIB: OnceCell<Mutex<Option<Library>>> = OnceCell::new();

const POSSIBLE_NAMES: &[&str] = &[
    "libpulse.so.0",
    "libpulse.so",
    "pipewire-pulse/libpulse.so.0",
    "pipewire-pulse/libpulse.so",
];

pub(crate) fn get_pulse_library() -> Option<&'static Mutex<Option<Library>>> {
    if PULSE_LIB.get().is_none() {
        let lib = POSSIBLE_NAMES
            .iter()
            .find_map(|name| unsafe { Library::new(*name).ok() });

        PULSE_LIB.set(Mutex::new(lib)).ok()?;
    }
    Some(PULSE_LIB.get()?)
}
