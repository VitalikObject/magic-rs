use std::fmt;

use crate::malloc;

#[repr(C)]
#[derive(PartialEq, Eq, Clone)]
pub struct LogicLong {
    pub higher_int: i32,
    pub lower_int: i32,
}

impl LogicLong {
    pub fn new(higher_int: i32, lower_int: i32) -> Self {
        Self {
            higher_int,
            lower_int,
        }
    }

    pub fn to_heap(&self) -> *const LogicLong {
        let ll = malloc(8);
        unsafe {
            *(ll as *mut i32) = self.higher_int;
            *(ll.wrapping_add(4) as *mut i32) = self.lower_int;
        }

        ll as *const LogicLong
    }

    pub fn is_zero(&self) -> bool {
        self.higher_int == 0 && self.lower_int == 0
    }
}

impl fmt::Display for LogicLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LogicLong({},{})", self.higher_int, self.lower_int)
    }
}
