use std::{
    ffi::{CStr, CString},
    fmt,
};

use crate::{import, malloc};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ScString(pub *const u8);

impl<S> From<S> for ScString
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        import!(string_ctor(ptr: *const u8, data: *const u8) -> () = 0x176748 + 1);
        let sc_string = malloc(32);

        string_ctor(sc_string, CString::new(value.as_ref()).unwrap().as_ptr());
        Self(sc_string)
    }
}

impl fmt::Display for ScString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length = unsafe { *(self.0.wrapping_add(4) as *const i32) as usize };
        if length + 1 > 8 {
            let c_string = unsafe { *(self.0.wrapping_add(8) as *const *const u8) };
            unsafe {
                write!(
                    f,
                    "{}",
                    CStr::from_ptr(c_string).to_string_lossy().to_string()
                )
            }
        } else if length > 0 {
            unsafe {
                write!(
                    f,
                    "{}",
                    CStr::from_ptr(self.0.wrapping_add(8) as *const u8)
                        .to_string_lossy()
                        .to_string()
                )
            }
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for ScString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[repr(transparent)]
pub struct StringBuilder(pub *const u8);

impl StringBuilder {
    pub fn new() -> Self {
        import!(string_builder_ctor(ptr: *const u8) -> () = 0x1772BA + 1);

        let instance = malloc(12);
        string_builder_ctor(instance);
        Self(instance)
    }
}

impl fmt::Display for StringBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        import!(string_builder_to_string(sc_str: *const u8, ptr: *const u8) -> () = 0x1772D8 + 1);
        let sc_str = ScString::from("");
        string_builder_to_string(sc_str.0, self.0);
        write!(f, "{sc_str}")
    }
}
