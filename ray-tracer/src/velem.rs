use num_traits::Float;

pub trait VElem:
    Float
    + From<f32>
    + Default
    + std::fmt::Display
    + std::fmt::Debug
    + rand::distributions::uniform::SampleBorrow<Self>
    + rand::distributions::uniform::SampleUniform
{
}

impl VElem for f32 {}
impl VElem for f64 {}
