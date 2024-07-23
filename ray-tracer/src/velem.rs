use num_traits::Float;

pub trait VElem: Float + Copy + std::fmt::Display + Default {}

impl VElem for f32 {}
