use generational_arena::Index;
/// Include relevent crates and modules
use ndarray::Array2;
use std::collections::HashMap;

use crate::component::Component;

/// GridComponentData Struct to store component related data inside the grid
#[derive(Debug)]
struct GridComponentData {
    /// Reference ID which is stored on the 2D grid
    ref_id: usize,
    /// Top left X starting cell position of component on grid
    position_x: usize,
    /// Top left Y starting cell position of component on grid
    position_y: usize,
    /// Total width of component on grid in cells
    width: usize,
    /// Total height of component on grid in cells
    height: usize,
}

/// GridComponentsDataMap Type for grid component hash mapping [unique ID -> GridComponentData]
type GridComponentsDataMap = HashMap<Index, GridComponentData>;

/// GridLayout Struct to store components information inside the grid
pub struct GridLayout {
    /// Total width of the grid in cells
    width: usize,
    /// Total height of the grid in cells
    height: usize,
    /// Grid data in 2D (0 means empty cell & positive number means occupied cell)
    data: Array2<i32>,
    /// Component mapping [unique ID (converted to String) -> GridComponentData]
    mapping: GridComponentsDataMap,
}

/// Methods for GridLayout Struct
impl GridLayout {
    /// Create new instance of GridLayout Struct
    ///
    /// # Arguments
    /// * `width` - width of a grid in cells
    /// * `height` - height of a grid in cells
    pub fn new(width: usize, height: usize) -> Self {
        let data = Array2::<i32>::zeros((width, height));
        let mapping = HashMap::new();
        Self {
            width,
            height,
            data,
            mapping,
        }
    }

    /// Resize the grid, called when html layout element gets resized
    pub fn resize(&mut self, width: usize, height: usize) {
        unimplemented!("Resize impl")
    }

    /// Insert new component into the grid
    pub fn insert_component(&mut self, component: &mut Component) {
        // unimplemented!("Inser component impl")
    }
}
