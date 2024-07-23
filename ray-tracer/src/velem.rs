use num_traits::Num;

pub trait VElem: Num + Copy {}

impl VElem for f32 {}
impl VElem for u8 {}
