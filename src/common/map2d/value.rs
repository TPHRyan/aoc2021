use std::fmt::{Debug, Display};

pub trait Value: Copy + Debug + Default + Display + PartialEq + PartialOrd + Sized {}

impl Value for bool {}

impl Value for i8 {}
impl Value for i16 {}
impl Value for i32 {}

impl Value for u8 {}
impl Value for u16 {}
impl Value for u32 {}
impl Value for u64 {}
impl Value for usize {}

impl Value for f32 {}
impl Value for f64 {}
