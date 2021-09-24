use generational_arena::Index;

#[derive(Default)]
pub struct FreeLayout {
    components: Vec<Index>,
}

impl FreeLayout {
    pub fn new(_width: u32, _height: u32) -> Self {
        Self {
            components: Vec::new(),
        }
    }
}
