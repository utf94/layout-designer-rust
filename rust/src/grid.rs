use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, SvgsvgElement};

pub struct Grid {
    grid: HtmlElement,
    placeholder: HtmlElement,
    svg: SvgsvgElement,

    pub pos: (u32, u32),
    pub size: (u32, u32),
}

impl Grid {
    fn new(grid: HtmlElement) -> Self {
        let children = grid.children();

        let mut svg = None;
        let mut placeholder = None;

        for id in 0..children.length() {
            let ch = children.item(id).unwrap();

            if ch.class_list().contains("grid-svg") {
                svg = Some(ch);
            } else if ch.class_list().contains("grid-placeholder") {
                placeholder = Some(ch);
            }
        }

        let svg = svg.expect("grid svg not found");
        let svg: SvgsvgElement = svg.dyn_into().unwrap();

        let placeholder = placeholder.expect("grid placeholder  not found");
        let placeholder: HtmlElement = placeholder.dyn_into().unwrap();

        Self {
            grid,
            placeholder,
            svg,

            pos: (0, 0),
            size: (1, 1),
        }
    }

    fn resize_placeholder(&mut self, w: f64, h: f64) {
        let div_x = w / 76.0;
        let div_y = h / 76.0;

        self.size = (div_x.round().max(1.0) as u32, div_y.round().max(1.0) as u32);
    }

    fn move_placeholder(&mut self, x: f64, y: f64) {
        let grid_bbox = self.grid.get_bounding_client_rect();

        let grid_w = (grid_bbox.width() / 76.0).floor() as u32;
        let grid_h = (grid_bbox.height() / 76.0).floor() as u32;

        let sub_x = x - grid_bbox.left();
        let sub_y = y - grid_bbox.top();

        let div_x = sub_x / 76.0;
        let div_y = sub_y / 76.0;

        let grid_x = div_x.floor() as u32 + 1;
        let grid_y = div_y.floor() as u32 + 1;

        let grid_x = grid_x.min(grid_w - self.size.0 + 1).max(0);
        let grid_y = grid_y.min(grid_h + 1).max(0);

        self.pos = (grid_x, grid_y);

        self.placeholder
            .style()
            .set_property("grid-column", &format!("{}/span {}", grid_x, self.size.0))
            .unwrap();
        self.placeholder
            .style()
            .set_property("grid-row", &format!("{}/span {}", grid_y, self.size.1))
            .unwrap();
    }
}

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

    pub fn resize_placeholder(&mut self, container: &HtmlElement, w: f64, h: f64) {
        let grid = self.grids.iter_mut().find(|g| &g.grid == container);
        let grid = grid.unwrap();
        grid.resize_placeholder(w, h);
    }

    pub fn move_placeholder_to(&mut self, container: &HtmlElement, x: f64, y: f64) {
        let grid = self.grids.iter_mut().find(|g| &g.grid == container);
        let grid = grid.unwrap();
        grid.move_placeholder(x, y);
    }

    pub fn get_grid(&self, container: &HtmlElement) -> &Grid {
        let grid = self.grids.iter().find(|g| &g.grid == container);
        let grid = grid.unwrap();
        grid
    }

    pub fn show(&mut self, container: &HtmlElement) {
        self.hide();

        let grid = self
            .grids
            .iter_mut()
            .find(|g| g.grid.is_equal_node(Some(container)));
        let grid = grid.unwrap();

        grid.svg
            .style()
            .set_property("visibility", "visible")
            .unwrap();
        grid.svg.style().set_property("opacity", "1").unwrap();

        grid.placeholder
            .style()
            .set_property("visibility", "visible")
            .unwrap();
        grid.placeholder
            .style()
            .set_property("opacity", "1")
            .unwrap();
    }

    pub fn hide(&self) {
        for grid in self.grids.iter() {
            grid.placeholder
                .style()
                .set_property("visibility", "hidden")
                .unwrap();
            grid.placeholder
                .style()
                .set_property("opacity", "0")
                .unwrap();

            grid.svg
                .style()
                .set_property("visibility", "hidden")
                .unwrap();
            grid.svg.style().set_property("opacity", "0").unwrap();
        }
    }
}
