use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

pub trait Component:
    Copy
    + Debug
    + Default
    + PartialEq
    + PartialOrd
    + Send
    + Sized
    + Sync
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}
impl Component for i8 {}
impl Component for i16 {}
impl Component for i32 {}
impl Component for i64 {}
impl Component for u8 {}
impl Component for u16 {}
impl Component for u32 {}
impl Component for u64 {}
impl Component for usize {}
impl Component for f32 {}
impl Component for f64 {}
