use generational_arena::Index;

#[derive(Default)]
pub struct FreeLayout {
    components: Vec<Index>,
}

impl FreeLayout {
    pub fn new(_width: usize, _height: usize) -> Self {
        Self {
            components: Vec::new(),
        }
    }
}
