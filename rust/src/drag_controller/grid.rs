use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Temporary Representation Of Grid layout element
/// Should be replaced by [crate::layout::grid::GridLayout]
pub struct Grid {
    grid: HtmlElement,
    placeholder: HtmlElement,

    placeholder_pos: (u32, u32),
    placeholder_size: (u32, u32),
}

impl Grid {
    fn new(grid: HtmlElement) -> Self {
        let placeholder = grid
            .query_selector(".grid-placeholder")
            .unwrap()
            .expect("grid placeholder  not found");
        let placeholder: HtmlElement = placeholder.dyn_into().unwrap();

        Self {
            grid,
            placeholder,

            placeholder_pos: (0, 0),
            placeholder_size: (1, 1),
        }
    }

    pub fn red_overlay(&self) -> bool {
        self.placeholder.class_list().contains("deny")
    }

    pub fn set_red_overlay(&self, is: bool) {
        if is {
            self.placeholder.class_list().add_1("deny").unwrap();
        } else {
            self.placeholder.class_list().remove_1("deny").unwrap();
        }
    }

    pub fn placeholder_pos(&self) -> (u32, u32) {
        self.placeholder_pos
    }

    pub fn placeholder_size(&self) -> (u32, u32) {
        self.placeholder_size
    }

    pub fn set_placeholder_visible(&self, is: bool) {
        if is {
            self.placeholder
                .style()
                .set_property("visibility", "visible")
                .unwrap();
            self.placeholder
                .style()
                .set_property("opacity", "1")
                .unwrap();
        } else {
            self.placeholder
                .style()
                .set_property("visibility", "hidden")
                .unwrap();
            self.placeholder
                .style()
                .set_property("opacity", "0")
                .unwrap();
        }
    }

    pub fn resize_placeholder(&mut self, w: f64, h: f64) {
        let div_x = w / 76.0;
        let div_y = h / 76.0;

        self.placeholder_size = (div_x.round().max(1.0) as u32, div_y.round().max(1.0) as u32);
    }

    pub fn move_placeholder(&mut self, x: f64, y: f64) {
        let grid_bbox = self.grid.get_bounding_client_rect();

        let grid_w = (grid_bbox.width() / 76.0).floor() as u32;
        let grid_h = (grid_bbox.height() / 76.0).floor() as u32;

        let sub_x = x - grid_bbox.left();
        let sub_y = y - grid_bbox.top();

        let div_x = sub_x / 76.0;
        let div_y = sub_y / 76.0;

        let grid_x = div_x.floor() as u32 + 1;
        let grid_y = div_y.floor() as u32 + 1;

        let grid_x = grid_x.min(grid_w - self.placeholder_size.0 + 1).max(0);
        let grid_y = grid_y.min(grid_h + 1).max(0);

        self.placeholder_pos = (grid_x, grid_y);

        self.placeholder
            .style()
            .set_property(
                "grid-column",
                &format!("{}/span {}", grid_x, self.placeholder_size.0),
            )
            .unwrap();
        self.placeholder
            .style()
            .set_property(
                "grid-row",
                &format!("{}/span {}", grid_y, self.placeholder_size.1),
            )
            .unwrap();
    }
}

/// Temporary Representation Of Grid elements list
/// Should be replaced
pub struct Grids {
    grids: Vec<Grid>,
}

impl Grids {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let grids = document
            .query_selector_all("layout-container.grid")
            .unwrap();

        let grids: Vec<_> = (0..grids.length())
            .map(|id| {
                let grid = grids.get(id).unwrap();
                let grid: HtmlElement = grid.dyn_into().unwrap();
                Grid::new(grid)
            })
            .collect();

        Self { grids }
    }

    pub fn get_grid(&self, container: &HtmlElement) -> &Grid {
        let grid = self.grids.iter().find(|g| &g.grid == container);
        grid.unwrap()
    }

    pub fn get_grid_mut(&mut self, container: &HtmlElement) -> &mut Grid {
        let grid = self.grids.iter_mut().find(|g| &g.grid == container);
        grid.unwrap()
    }

    pub fn hide_placeholders(&self) {
        for grid in self.grids.iter() {
            grid.set_red_overlay(false);
            grid.set_placeholder_visible(false);
        }
    }
}
