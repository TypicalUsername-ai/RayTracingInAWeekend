use num_traits::Float;

pub trait VElem: Float + Copy + std::fmt::Display {}

impl VElem for f32 {}
