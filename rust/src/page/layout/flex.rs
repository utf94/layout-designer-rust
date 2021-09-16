use generational_arena::Index;

pub struct FlexLayout {
    components: Vec<Index>,
}

impl FlexLayout {
    pub fn new(_width: usize, _height: usize) -> Self {
        Self {
            components: Vec::new(),
        }
    }
}
