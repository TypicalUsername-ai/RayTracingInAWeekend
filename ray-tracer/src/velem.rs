use num_traits::Float;

pub trait VElem: Float + Copy {}

impl VElem for f32 {}
