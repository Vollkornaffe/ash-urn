#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct Align8<T: Copy>(pub T);
impl<T: Copy> From<T> for Align8<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct Align16<T: Copy>(pub T);
impl<T: Copy> From<T> for Align16<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
