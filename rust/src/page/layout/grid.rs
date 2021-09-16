/// Include relevent crates and modules
use ndarray::Array2;
use std::collections::HashMap;
use generational_arena::Index;
use crate::component::Component;

/// GridComponentData Struct to store component related data inside the grid
#[derive(Debug)]
struct GridComponentData {
    /// Reference ID which is stored on the 2D grid
    ref_id: usize,
    /// Top left X starting cell position of component on grid
    position_x: u32,
    /// Top left Y starting cell position of component on grid
    position_y: u32,
    /// Total width of component on grid in cells
    width: u32,
    /// Total height of component on grid in cells
    height: u32,
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
    data: Array2<usize>,
    /// Component mapping [unique ID (converted to String) -> GridComponentData]
    mapping: GridComponentsDataMap,
    /// Reference id counter for next value to be used inside the grid cell for component representation
    ref_id_count: usize,
}

/// Methods for GridLayout Struct
impl GridLayout {

    /// Create new instance of GridLayout Struct
    ///
    /// # Arguments
    /// * `width` - width of a grid in cells
    /// * `height` - height of a grid in cells
    pub fn new(width: usize, height: usize) -> Self {
        let data = Array2::<usize>::zeros((width, height));
        let mapping = HashMap::new();
        let ref_id_count = 0;
        Self {
            width,
            height,
            data,
            mapping,
            ref_id_count
        }
    }

    /// Resize the grid, called when html layout element gets resized
    ///
    /// # Arguments
    /// * `width` - width of a grid in cells
    /// * `height` - height of a grid in cells
    pub fn resize(&mut self, width: usize, height: usize) {
        
    }
    
    /// Insert new component into the grid
    /// 
    /// # Arguments
    /// * `component` - component to to add or update
    pub fn insert_component(&mut self, component: &mut Component) {
        // If component already exists then update its data on grid map
        if self.mapping.contains_key(&component.index()) {
            log::debug!("Component already exists {:?}", self.mapping.get(&component.index()));
        }
        // If component is new then add its data on grid map
        else {
            self.ref_id_count += 1;
            let new_grid_component_data = GridComponentData {
                ref_id: self.ref_id_count,
                position_x: component.grid_pos().0,
                position_y: component.grid_pos().1,
                width: component.grid_size().0,
                height: component.grid_size().1
            };

            log::debug!("New entry {:?}", new_grid_component_data);

            self.update_data(&new_grid_component_data);
            self.mapping.insert(component.index(), new_grid_component_data);
        }
    }

    /// Update the data on the grid with new reference id for given component
    ///  
    /// # Arguments
    /// * `grid_component_data` - Grid component data to update on grid map
    pub fn update_data(&mut self, grid_component_data: &GridComponentData) {
        
    }
}
