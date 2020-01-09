#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_loop)]
#![feature(const_panic)]

pub use libc;
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

unsafe impl Send for ZoneGuard {}
unsafe impl Sync for ZoneGuard {}

pub struct FrameGuard();
impl FrameGuard {
    pub fn new() -> Self {
        #[cfg(tracy_enable)]
        unsafe {
            let name = b"frame\0";
            ___tracy_emit_frame_mark_start(name as *const _ as *const c_char);
        }
        FrameGuard()
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        #[cfg(tracy_enable)]
        unsafe {
            let name = b"frame\0";
            ___tracy_emit_frame_mark_end(name as *const _ as *const c_char);
        }
    }
}

pub const fn const_cstr(x: &'static str) -> [u8; 64] {
    assert!(x.len() < 64);
    let x = x.as_bytes();
    let mut res: [u8; 64] = [0; 64];
    let mut i = 0;
    while i < 63 {
        if i < x.len() {
            res[i] = x[i]
        } else {
            res[i] = 0;
        }
        i += 1;
    }
    res
}

#[macro_export]
macro_rules! zone {
    ($func:literal) => {
        $crate::zone!($func, $func)
    };
    ($func:literal, $name:literal) => {
        const FUNC_LITERAL_CSTR_BUFFER: [u8; 64] = $crate::const_cstr($func);
        const NAME_LITERAL_CSTR_BUFFER: [u8; 64] = $crate::const_cstr($name);
        const FILE_LITERAL_CSTR_BUFFER: [u8; 64] = $crate::const_cstr(file!());
        const SRC_LOC: $crate::___tracy_source_location_data =
            $crate::___tracy_source_location_data {
                color: 0,
                line: line!(),
                file: &FILE_LITERAL_CSTR_BUFFER as *const _ as *const $crate::libc::c_char,
                function: &FUNC_LITERAL_CSTR_BUFFER as *const _ as *const $crate::libc::c_char,
                name: &NAME_LITERAL_CSTR_BUFFER as *const _ as *const $crate::libc::c_char,
            };
        const SRC_LOC_PTR: &'static $crate::___tracy_source_location_data = &SRC_LOC;
        #[cfg(tracy_enable)]
        let _guard =
            $crate::ZoneGuard::from(unsafe { $crate::___tracy_emit_zone_begin(SRC_LOC_PTR, 1) });
    };
}

#[cfg(test)]
mod tests {
    fn test() {
        zone!("test");
    }
}
