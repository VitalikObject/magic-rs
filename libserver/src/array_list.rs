#[repr(C)]
pub struct LogicArrayList<T> {
    pub data: *const T,
    pub _capacity: usize,
    pub count: usize,
}

impl<T> LogicArrayList<T>
where
    T: Sized,
{
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.count) }
    }
}
