use crate::memory_alignment::Align16;

#[repr(C, align(64))]
pub struct Vertex {
    pub pos: Align16<[f32; 3]>,
    pub nor: Align16<[f32; 3]>,
    pub col: Align16<[f32; 4]>,
    pub tex: Align16<[f32; 2]>,
}

#[repr(C)]
pub struct Indices(pub [u32; 3]);
