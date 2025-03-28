use proc_maps::Pid;
use std::{ffi::c_void, sync::LazyLock};

pub static LIBG_BASE: LazyLock<usize> = LazyLock::new(|| get_module_base("libg.so").unwrap());

macro_rules! import {
    ($name:ident($($arg_name:ident: $arg_type:ty),*) -> $ret_type:ty = $rva:expr) => {
        pub fn $name($($arg_name: $arg_type,)*) -> $ret_type {
            unsafe {
                type FuncType = unsafe extern "C" fn($($arg_type,)*) -> $ret_type;
                 ::std::mem::transmute::<usize, FuncType>(*crate::ffi_util::LIBG_BASE + $rva)($($arg_name,)*)
            }
        }
    };
}

pub fn disable_event_tracker() {
    // Causes crashes in logic functions due to being not initialized
    // useless SC analytics.

    const EVENT_TRACKER_FUNCTIONS: &[i32] = &[0x14BCC0, 0x14BA1C, 0x14BB4C, 0x14BA88, 0x1A39A0];
    const TRACK_FUNCTIONS: &[i32] = &[0x1A3E58, 0x14BE64, 0x14BC58];

    unsafe {
        for &addr in EVENT_TRACKER_FUNCTIONS.iter().chain(TRACK_FUNCTIONS) {
            let page_size = libc::sysconf(libc::_SC_PAGE_SIZE);
            let addr = LIBG_BASE.wrapping_add(addr as usize);
            libc::mprotect(
                ((addr as i32) & !(page_size - 1)) as *mut c_void,
                page_size as usize,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            );

            std::slice::from_raw_parts_mut(addr as *mut u8, 2).copy_from_slice(&[0x70, 0x47]);
        }
    }
}

pub(crate) use import;

pub fn get_module_base(shared_object_name: &str) -> Option<usize> {
    const ELF_MAGIC: u32 = 0x464C457F;

    proc_maps::get_process_maps(std::process::id() as Pid)
        .ok()?
        .into_iter()
        .filter(|range| {
            range
                .filename()
                .map(|p| p.to_string_lossy().ends_with(shared_object_name))
                .unwrap_or(false)
        })
        .find(|range| unsafe { *(range.start() as *const u32) } == ELF_MAGIC)
        .map(|range| range.start())
}
