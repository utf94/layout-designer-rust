/// Include relevent crates and modules
use ndarray::Array2;
use std::collections::HashMap;
use generational_arena::Index;
use crate::component::Component;

/// Block Struct to represent position and size of block on grid
struct Block {
    /// Top left X starting cell position of block
    x: usize,
    /// Top left Y starting cell position of block
    y: usize,
    /// Total width of block in cells
    width: usize,
    /// Total height of block in cells
    height: usize,
}

/// GridComponentData Struct to store component related data inside the grid
struct GridComponentData {
    /// Reference ID which is stored on the 2D grid
    ref_id: i32,
    /// Top left X starting cell position of component on grid
    x: usize,
    /// Top left Y starting cell position of component on grid
    y: usize,
    /// Total width of component on grid in cells
    width: usize,
    /// Total height of component on grid in cells
    height: usize,
    /// Reference to the component attached to it
    component: Component,
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
    /// Component mapping [Reference ID -> unique ID (converted to String)]
    mapping_ref_id: HashMap<i32, Index>,
    /// Reference id counter for next value to be used inside the grid cell for component representation
    ref_id_count: i32,
}

/// Methods for GridLayout Struct
impl GridLayout {

    /// Create new instance of GridLayout Struct
    ///
    /// # Arguments
    /// * `width` - Width of a grid in cells
    /// * `height` - Height of a grid in cells
    pub fn new(width: usize, height: usize) -> Self {
        let data = Array2::<i32>::zeros((width, height));
        let mapping = HashMap::new();
        let mapping_ref_id = HashMap::new();
        let ref_id_count = 0;
        Self {
            width,
            height,
            data,
            mapping,
            mapping_ref_id,
            ref_id_count
        }
    }

    /// Resize the grid, called when html layout element gets resized
    ///
    /// # Arguments
    /// * `width` - Width of a grid in cells
    /// * `height` - Height of a grid in cells
    pub fn resize(&mut self, width: usize, height: usize) {
        let mut move_cells = 0;
        let movement_stack : Vec<(i32, GridComponentData)> = Vec::new();
        // If new width is smaller than move components to left (as far as they can go)
        if width < self.width {
            move_cells = self.width - width;
        }
        // If new height is smaller than move components to top (as far as they can go)
        if height < self.height {
            move_cells = self.height - height;
        }
    }
    
    /// Insert new or update (position or size) component into the grid
    /// 
    /// # Arguments
    /// * `component` - Component to to add or update
    pub fn insert_component(&mut self, component: &mut Component) {
        let ref_id : i32;
        // If component already exists then remove it to insert it again with updated data
        if self.mapping.contains_key(&component.index()) {
            let grid_component_data : GridComponentData = self.mapping.remove(&component.index()).unwrap();
            self.mapping_ref_id.remove(&grid_component_data.ref_id).unwrap();
            let grid_component_block = Block {
                x: grid_component_data.x,
                y: grid_component_data.y,
                width: grid_component_data.width,
                height: grid_component_data.height,
            };
            self.set_data_block(grid_component_block, 0);
            ref_id = grid_component_data.ref_id;
        }
        // If component is new then calculate its new reference id
        else {
            self.ref_id_count += 1;
            ref_id = self.ref_id_count;
        }
        // Insert new or updated component back to mapping and update it on grid
        let new_grid_component_data = GridComponentData {
            ref_id: ref_id,
            x: component.grid_pos().0 as usize,
            y: component.grid_pos().1 as usize,
            width: component.grid_size().0 as usize,
            height: component.grid_size().1 as usize,
            component: component.clone()
        };
        let grid_component_block = Block {
            x: new_grid_component_data.x,
            y: new_grid_component_data.y,
            width: new_grid_component_data.width,
            height: new_grid_component_data.height,
        };
        self.set_data_block(grid_component_block, new_grid_component_data.ref_id);
        self.mapping_ref_id.insert(new_grid_component_data.ref_id, component.index());
        self.mapping.insert(component.index(), new_grid_component_data);
    }

    /// Check if the component is currently overlapping on another component (returns true if overlapping)
    ///  
    /// # Arguments
    /// * `component` - Component to check if its overlapping on another component on grid
    fn is_component_overlapping(&mut self, component: &mut Component) -> bool {
        let block = Block {
            x: component.grid_pos().0 as usize,
            y: component.grid_pos().1 as usize,
            width: component.grid_size().0 as usize,
            height: component.grid_size().1 as usize,
        };
        let mut ref_id = -1;
        if self.mapping.contains_key(&component.index()) {
            ref_id = self.mapping.get(&component.index()).unwrap().ref_id;
        }
        let mut overlapping = false;
        'outer: for i in block.x..(block.x + block.width) {
            for j in block.y..(block.y + block.height) {
                if self.get_data_cell(i, j) != 0 && self.get_data_cell(i, j) != ref_id {
                    overlapping = true;
                    break 'outer
                }
            }
        }
        {overlapping}
    }

    /// Check if the block is empty or contain components (returns true if empty)
    ///  
    /// # Arguments
    /// * `block` - Block representing position and size on grid
    fn is_data_block_empty(&mut self, block: Block) -> bool {
        let mut component_sum = 0;
        for i in block.x..(block.x + block.width) {
            for j in block.y..(block.y + block.height) {
                component_sum += self.get_data_cell(i, j);
            }
        }
        if component_sum > 0 {
            false
        }
        else {
            true
        }
    }

    /// Set the data on the grid with new value for given block
    ///  
    /// # Arguments
    /// * `block` - Block representing position and size on grid
    /// * `value` - Value to write for block
    fn set_data_block(&mut self, block: Block, value: i32) {
        for i in block.x..(block.x + block.width) {
            for j in block.y..(block.y + block.height) {
                self.set_data_cell(value, i, j);
            }
        }
    }

    /// Set the data cell on the grid with new value
    ///  
    /// # Arguments
    /// * `value` - Value for cell
    /// * `x` - X position of cell
    /// * `y` - Y position of cell
    fn set_data_cell(&mut self, value: i32, x: usize, y: usize) {
        self.data[[x-1, y-1]] = value;
    }

    /// Get the value of data cell on the grid
    ///  
    /// # Arguments
    /// * `x` - X position of cell
    /// * `y` - Y position of cell
    fn get_data_cell(&mut self, x: usize, y: usize) -> i32 {
        self.data[[x-1, y-1]]
    }
}
