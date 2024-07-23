use num_traits::Float;

pub trait VElem: Float + Copy + std::fmt::Display + Default + From<f32> {}

impl VElem for f32 {}
