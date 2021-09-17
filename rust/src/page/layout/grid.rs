/// Include relevent crates and modules
use ndarray::Array2;
use std::collections::HashMap;
use generational_arena::Index;
use crate::component::Component;

/// GridComponentData Struct to store component related data inside the grid
#[derive(Debug)]
struct GridComponentData {
    /// Reference ID which is stored on the 2D grid
    ref_id: i32,
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
    /// Reference id counter for next value to be used inside the grid cell for component representation
    ref_id_count: i32,
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

        }
        // If component is new then add its data on grid map
        else {
            self.ref_id_count += 1;
            let new_grid_component_data = GridComponentData {
                ref_id: self.ref_id_count,
                position_x: component.grid_pos().0 as usize,
                position_y: component.grid_pos().1 as usize,
                width: component.grid_size().0 as usize,
                height: component.grid_size().1 as usize
            };
            self.update_data(&new_grid_component_data);
            // log::debug!("Map {:?}", self.data);
            self.mapping.insert(component.index(), new_grid_component_data);
        }
    }

    /// Update the data on the grid with new reference id for given component
    ///  
    /// # Arguments
    /// * `grid_component_data` - Grid component data to update on grid map
    fn update_data(&mut self, grid_component_data: &GridComponentData) {
        for i in grid_component_data.position_x..(grid_component_data.position_x + grid_component_data.width) {
            for j in grid_component_data.position_y..(grid_component_data.position_y + grid_component_data.height) {
                self.set_data_cell(grid_component_data.ref_id, i, j);
            }
        }
    }

    /// Set the data cell on the grid with new value
    ///  
    /// # Arguments
    /// * `value` - value for cell
    /// * `x` - X position of cell
    /// * `y` - Y position of cell
    fn set_data_cell(&mut self, value: i32, x: usize, y: usize) {
        self.data[[x-1, y-1]] = value;
    }
}
