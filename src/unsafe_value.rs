use std::{fmt::Debug, ops::Deref};

pub struct UnsafeValue<T: Copy + Clone + Debug> {
    value: T,
}

impl<T: Copy + Clone + Debug> Debug for UnsafeValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T: Copy + Clone + Debug> UnsafeValue<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> T {
        self.value
    }

    pub fn set_value(&self, new_value: T) {
        unsafe {
            let value = &self.value as *const T as *mut T;
            value.write(new_value);
        }
    }
}

impl<T: Clone + Copy + Debug> Deref for UnsafeValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<bool> for UnsafeValue<bool> {
    fn from(val: bool) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<u8> for UnsafeValue<u8> {
    fn from(val: u8) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<i8> for UnsafeValue<i8> {
    fn from(val: i8) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<u16> for UnsafeValue<u16> {
    fn from(val: u16) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<i16> for UnsafeValue<i16> {
    fn from(val: i16) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<u32> for UnsafeValue<u32> {
    fn from(val: u32) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<i32> for UnsafeValue<i32> {
    fn from(val: i32) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<u64> for UnsafeValue<u64> {
    fn from(val: u64) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<i64> for UnsafeValue<i64> {
    fn from(val: i64) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<usize> for UnsafeValue<usize> {
    fn from(val: usize) -> Self {
        UnsafeValue::new(val)
    }
}

impl From<isize> for UnsafeValue<isize> {
    fn from(val: isize) -> Self {
        UnsafeValue::new(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::UnsafeValue;

    #[test]
    fn test_change_value_unsafe() {
        let value = UnsafeValue::new(10);

        assert_eq!(10, value.get_value());

        value.set_value(20);

        assert_eq!(20, value.get_value());
    }
}
