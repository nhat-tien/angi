use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

mod engine;
use engine::TemplateEngine;

static ENGINE: OnceLock<Mutex<TemplateEngine>> = OnceLock::new();

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn angi_set_template_dir(dir: *const c_char) {
    let dir = unsafe {
        CStr::from_ptr(dir).to_string_lossy().to_string()
    };

    let engine = TemplateEngine::new(PathBuf::from(dir));
    ENGINE.set(Mutex::new(engine)).ok();
}

/// # Safety
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn angi_render_template_file(
    name: *const c_char,
    json_ctx: *const c_char,
) -> *mut c_char {
    let name = unsafe {
        CStr::from_ptr(name).to_string_lossy().to_string()
    };

    let json_ctx = unsafe {
        CStr::from_ptr(json_ctx).to_string_lossy().to_string()
    };

    let ctx: serde_json::Value =
        match serde_json::from_str(&json_ctx) {
            Ok(v) => v,
            Err(e) => return error_string(e.to_string()),
        };

    let engine = ENGINE.get().unwrap().lock().unwrap();

    match engine.render(&name, &ctx) {
        Ok(out) => CString::new(out).unwrap().into_raw(),
        Err(e) => error_string(e),
    }
}

fn error_string(msg: String) -> *mut c_char {
    CString::new(format!("__ERROR__:{}", msg))
        .unwrap()
        .into_raw()
}
