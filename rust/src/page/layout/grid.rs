/// Include relevent crates and modules
use crate::component::Component;
use generational_arena::Index;
use ndarray::Array2;
use std::collections::HashMap;

/// Block Struct to represent position and size of block on grid
pub struct Block {
    /// Top left X starting cell position of block
    pub x: usize,
    /// Top left Y starting cell position of block
    pub y: usize,
    /// Total width of block in cells
    pub width: usize,
    /// Total height of block in cells
    pub height: usize,
}

/// GridComponentData Struct to store component related data inside the grid
pub struct GridComponentData {
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
pub type GridComponentsDataMap = HashMap<Index, GridComponentData>;

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
        let mut data = Array2::<i32>::zeros((width, height));
        let mapping = HashMap::new();
        let mapping_ref_id = HashMap::new();
        let ref_id_count = 0;
        Self {
            width,
            height,
            data,
            mapping,
            mapping_ref_id,
            ref_id_count,
        }
    }

    /// Resize the grid, called when html layout element gets resized
    ///
    /// # Arguments
    /// * `width` - Width of a grid in cells
    /// * `height` - Height of a grid in cells
    pub fn resize(&mut self, width: usize, height: usize) {
        let mut new_data = Array2::<i32>::zeros((width, height));
        let old_rows = self.data.nrows();
        let old_cols = self.data.ncols();
        for i in 0..old_rows {
            for j in 0..old_cols {
                new_data[[i, j]] = self.data[[i, j]];
            }
        }
        self.data = new_data;
    }

    /// Insert new or update (position or size) component into the grid
    ///
    /// # Arguments
    /// * `component` - Component to add or update
    pub fn insert_component(&mut self, component: &mut Component) {
        let ref_id: i32;
        // If component already exists then remove it to insert it again with updated data
        if self.mapping.contains_key(&component.index()) {
            let grid_component_data: GridComponentData = self.mapping.remove(&component.index()).unwrap();
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
            component: component.clone(),
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
        log::debug!("{:?}", self.data);
    }

    /// Remove existing component from the grid using component reference
    ///
    /// # Arguments
    /// * `component` - Component to remove
    pub fn remove_component(&mut self, component: &mut Component) {
        self.remove_component_by_index(component.index());
    }

    /// Remove existing component from the grid using component index
    ///
    /// # Arguments
    /// * `index` - Index of component to remove
    pub fn remove_component_by_index(&mut self, index: Index) {
        if self.mapping.contains_key(&index) {
            let grid_component_data: GridComponentData = self.mapping.remove(&index).unwrap();
            self.mapping_ref_id.remove(&grid_component_data.ref_id).unwrap();
            let grid_component_block = Block {
                x: grid_component_data.x,
                y: grid_component_data.y,
                width: grid_component_data.width,
                height: grid_component_data.height,
            };
            self.set_data_block(grid_component_block, 0);
        }
    }

    /// Returns the list of Index of all components at a specific block in vector (empty vector if no component)
    ///  
    /// # Arguments
    /// * `block` - Block representing position and size on grid
    pub fn get_block_component_indices(&self, block: Block) -> Vec<Index> {
        let mut cell_value = 0;
        let mut indices: Vec<Index> = Vec::new();
        for i in block.x..(block.x + block.width) {
            for j in block.y..(block.y + block.height) {
                cell_value = self.get_data_cell(i, j);
                if self.mapping_ref_id.contains_key(&cell_value) {
                    let component_index = self.mapping_ref_id.get(&cell_value).unwrap();
                    if !indices.contains(component_index) {
                        indices.push(*component_index);
                    }
                }
            }
        }
        {
            indices
        }
    }

    /// Returns the Index of component at a specific cell in vector (empty vector if no component)
    ///  
    /// # Arguments
    /// * `x` - X position of cell
    /// * `y` - Y position of cell
    pub fn get_cell_component_index(&self, x: usize, y: usize) -> Option<Index> {
        let cell_value = self.get_data_cell(x, y);
        self.mapping_ref_id.get(&cell_value).copied()
    }

    /// Check if the component is currently overlapping on another component (returns true if overlapping)
    ///  
    /// # Arguments
    /// * `component` - Component to check if its overlapping on another component on grid
    pub fn is_component_overlapping(&mut self, component: &mut Component) -> bool {
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
                    break 'outer;
                }
            }
        }
        {
            overlapping
        }
    }

    /// Check if the block is empty or contain components (returns true if empty)
    ///  
    /// # Arguments
    /// * `block` - Block representing position and size on grid
    pub fn is_data_block_empty(&self, block: Block) -> bool {
        let mut component_sum = 0;
        for i in block.x..(block.x + block.width) {
            for j in block.y..(block.y + block.height) {
                component_sum += self.get_data_cell(i, j);
            }
        }
        if component_sum > 0 {
            false
        } else {
            true
        }
    }

    /// Set the data on the grid with new value for given block
    ///  
    /// # Arguments
    /// * `block` - Block representing position and size on grid
    /// * `value` - Value to write for block
    pub fn set_data_block(&mut self, block: Block, value: i32) {
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
    pub fn set_data_cell(&mut self, value: i32, x: usize, y: usize) {
        self.data[[x - 1, y - 1]] = value;
    }

    /// Get the value of data cell on the grid
    ///  
    /// # Arguments
    /// * `x` - X position of cell
    /// * `y` - Y position of cell
    pub fn get_data_cell(&self, x: usize, y: usize) -> i32 {
        self.data[[x - 1, y - 1]]
    }
}
