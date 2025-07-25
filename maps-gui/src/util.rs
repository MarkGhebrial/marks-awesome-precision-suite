use std::ops::{Add, Div, Mul, Range, Sub};

/// Map a value from one range to another.
pub fn map<T>(value: T, in_range: Range<T>, out_range: Range<T>) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Div<Output = T>,
    T: Copy,
{
    (value - in_range.start) * (out_range.end - out_range.start) / (in_range.end - in_range.start)
        + out_range.start
}
