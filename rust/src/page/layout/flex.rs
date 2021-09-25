use generational_arena::Index;

pub struct FlexLayout {
    components: Vec<Index>,
}

impl FlexLayout {
    pub fn new(_width: u32, _height: u32) -> Self {
        Self {
            components: Vec::new(),
        }
    }
}
