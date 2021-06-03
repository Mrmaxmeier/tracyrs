#![cfg_attr(feature = "tracy_enable", feature(const_panic))]

pub use libc;
use libc::c_char;

#[repr(C)]
pub struct ZoneCtx {
    _private: [u8; 0],
}
pub type TracyCZoneCtx = *const ZoneCtx;

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
    fn ___tracy_emit_zone_color(ctx: TracyCZoneCtx, color: u32);
    fn ___tracy_emit_zone_value(ctx: TracyCZoneCtx, value: u64);

    fn ___tracy_emit_frame_mark(name: *const c_char);
    fn ___tracy_emit_frame_mark_start(name: *const c_char);
    fn ___tracy_emit_frame_mark_end(name: *const c_char);

    fn ___tracy_emit_plot(name: *const c_char, val: f64);
    fn ___tracy_emit_messageL(txt: *const c_char, callstack: u32);
}

pub fn emit_plot(_name: &'static [u8], _val: f64) {
    #[cfg(feature = "tracy_enable")]
    unsafe {
        ___tracy_emit_plot(_name as *const _ as *const c_char, _val);
    }
}

pub fn emit_message_l(_txt: &'static [u8]) {
    #[cfg(feature = "tracy_enable")]
    unsafe {
        ___tracy_emit_messageL(_txt as *const _ as *const c_char, 0);
    }
}

pub fn frame_mark() {
    #[cfg(feature = "tracy_enable")]
    unsafe {
        ___tracy_emit_frame_mark(std::ptr::null());
    }
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

    pub fn color(&mut self, color: u32) {
        let _ = color;
        #[cfg(feature = "tracy_enable")]
        unsafe {
            crate::___tracy_emit_zone_color(self.0, color)
        }
    }

    pub fn value(&mut self, value: u64) {
        let _ = value;
        #[cfg(feature = "tracy_enable")]
        unsafe {
            crate::___tracy_emit_zone_value(self.0, value)
        }
    }
}

impl Drop for ZoneGuard {
    fn drop(&mut self) {
        #[cfg(feature = "tracy_enable")]
        unsafe {
            crate::___tracy_emit_zone_end(self.0)
        }
    }
}

pub struct FrameGuard(&'static [u8]);
impl FrameGuard {
    pub fn new() -> Self {
        let name = b"frame\0";
        #[cfg(feature = "tracy_enable")]
        unsafe {
            ___tracy_emit_frame_mark_start(name as *const _ as *const c_char);
        }
        FrameGuard(name)
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        #[cfg(feature = "tracy_enable")]
        unsafe {
            ___tracy_emit_frame_mark_end(self.0 as *const _ as *const c_char);
        }
    }
}

#[cfg(feature = "tracy_enable")]
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

#[cfg(feature = "tracy_enable")]
#[macro_export]
macro_rules! zone_guard {
    ($func:literal) => {
        $crate::zone_guard!($func, $func)
    };
    ($func:literal, $name:literal) => {{
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
        $crate::ZoneGuard::from(unsafe { $crate::___tracy_emit_zone_begin(SRC_LOC_PTR, 1) })
    }};
}

#[cfg(not(feature = "tracy_enable"))]
#[macro_export]
macro_rules! zone_guard {
    ($func:literal) => {
        $crate::ZoneGuard::from(std::ptr::null())
    };
    ($func:literal, $name:literal) => {
        $crate::ZoneGuard::from(std::ptr::null())
    };
}

#[cfg(feature = "tracy_enable")]
#[macro_export]
macro_rules! zone {
    ($func:literal) => {
        $crate::zone!($func, $func)
    };
    ($func:literal, $name:literal) => {
        let _guard = $crate::zone_guard!($func, $name);
    };
}

#[cfg(not(feature = "tracy_enable"))]
#[macro_export]
macro_rules! zone {
    ($func:literal) => {};
    ($func:literal, $name:literal) => {};
}

#[cfg(feature = "tracy_enable")]
#[macro_export]
macro_rules! message {
    ($txt:literal) => {{
        const TXT_LITERAL_CSTR_BUFFER: [u8; 64] = $crate::const_cstr($txt);
        $crate::emit_message_l(&TXT_LITERAL_CSTR_BUFFER);
    }};
}

#[cfg(not(feature = "tracy_enable"))]
#[macro_export]
macro_rules! message {
    ($txt:literal) => {};
}
