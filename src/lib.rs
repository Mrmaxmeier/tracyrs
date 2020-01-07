#![feature(const_fn)]

use libc::c_char;

#[repr(C)]
pub struct ZoneCtx {
    _private: [u8; 0],
}
pub type TracyCZoneCtx = *const ZoneCtx;

#[no_mangle]
extern "C" {
    pub fn ___tracy_emit_zone_begin(
        srcloc: *const ___tracy_source_location_data,
        active: u32,
    ) -> TracyCZoneCtx;
    fn ___tracy_emit_zone_begin_callstack(
        srcloc: *const ___tracy_source_location_data,
        depth: u32,
        active: u32,
    ) -> TracyCZoneCtx;
    fn ___tracy_emit_zone_end(ctx: TracyCZoneCtx);
    fn ___tracy_emit_zone_name(ctx: TracyCZoneCtx, name: *const c_char, size: usize);
    fn ___tracy_emit_zone_text(ctx: TracyCZoneCtx, text: *const c_char, size: usize);

    fn ___tracy_emit_frame_mark(name: *const c_char);
    fn ___tracy_emit_frame_mark_start(name: *const c_char);
    fn ___tracy_emit_frame_mark_end(name: *const c_char);
}

#[repr(C)]
pub struct ___tracy_source_location_data {
    pub name: *const c_char,
    pub function: *const c_char,
    pub file: *const c_char,
    pub line: u32,
    pub color: u32,
}
// TODO: &'static
unsafe impl Send for ___tracy_source_location_data {}
unsafe impl Sync for ___tracy_source_location_data {}

#[repr(C)]
struct ___tracy_c_zone_context {
    id: u32,
    active: u32,
}

pub struct ZoneGuard(TracyCZoneCtx);
impl ZoneGuard {
    pub fn from(ctx: TracyCZoneCtx) -> Self {
        ZoneGuard(ctx)
    }
}

impl Drop for ZoneGuard {
    fn drop(&mut self) {
        unsafe { crate::___tracy_emit_zone_end(self.0) }
    }
}

pub struct FrameGuard();
impl FrameGuard {
    pub fn new() -> Self {
        let name = b"frame\0";
        unsafe {
            ___tracy_emit_frame_mark_start(name as *const _ as *const c_char);
        }
        FrameGuard()
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        let name = b"frame\0";
        unsafe {
            ___tracy_emit_frame_mark_end(name as *const _ as *const c_char);
        }
    }
}

pub const fn cstr(x: &str) -> &[u8] {
    //use std::ffi::CString;
    //let s = unsafe { CString::from_vec_unchecked(x.into()) };
    //s.as_ptr()
    // b"hi\0"
    /*
    let x = x.as_bytes();
    let mut res: [u8; 42] = [0; 42];
    for i in 0..42 {
        unsafe {
            if i < x.len() {
                res[i] = x[i]
            } else {
                res[i] = 0;
            }
        }
    }
    unsafe { &res }
    */
    x.as_bytes()
}

#[macro_export]
macro_rules! zone {
    ($func:literal) => {
        use libc::c_char;
        use $crate::{ZoneGuard, ___tracy_source_location_data};
        static _srcloc: ___tracy_source_location_data = ___tracy_source_location_data {
            color: 0,
            line: line!(),
            file: b"file\0" as *const _ as *const c_char,
            function: b"function\0" as *const _ as *const c_char,
            name: b"name\0" as *const _ as *const c_char,
        };
        let __srcloc: &'static ___tracy_source_location_data = &_srcloc;
        let _guard = ZoneGuard::from(unsafe { $crate::___tracy_emit_zone_begin(__srcloc, 1) });
    };
}

#[cfg(test)]
mod tests {
    fn test() {
        zone!("test");
    }
}
