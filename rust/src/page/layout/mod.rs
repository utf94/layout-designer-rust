#![allow(unused)]

use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use generational_arena::Index;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

pub mod grid;
use grid::GridLayout;

mod free;
use free::FreeLayout;

mod flex;
use flex::FlexLayout;

use crate::{component::Component, editor::hierarchy::HierarchyItemData};

use self::grid::background::GridBackground;

/// Type of a layout
pub enum LayoutKind {
    /// Layout based on absolute position
    /// It is positioning components in relation to it's left-top root point
    Free {
        /// Data related to free layout implementation
        free: FreeLayout,
    },

    /// CSS Flexbox based layout
    Flex {
        /// Data related to flex layout implementation
        flex: FlexLayout,
    },

    /// CSS Grid based layout
    Grid {
        /// Size of a grid cell in px
        cell_size: u32,
        /// Grid background
        grid_background: GridBackground,
        /// Data related to grid layout implementation
        grid_data: Box<GridLayout>,
    },
}

struct Data {
    /// Name of a layout
    name: RefCell<String>,

    /// Height of a layout
    height: RefCell<u32>,
    /// Width of a layout
    width: RefCell<u32>,
    /// Layout kind specyfic data
    kind: RefCell<LayoutKind>,
    /// Children of a layout
    components: RefCell<Vec<Component>>,

    /// Hierarchy related data
    hierarchy_data: RefCell<HierarchyItemData>,
}

/// Layout struct that represents layout node
#[derive(Clone)]
pub struct Layout {
    /// Root html element of a layout
    pub html_element: HtmlElement,
    /// Inner ref counted data
    data: Rc<Data>,
}

// Init related methods:
impl Layout {
    /// Creates a new layout
    ///
    /// It will initialize the layout data,
    /// and also create a new html element that represents the layout
    pub fn new(width: u32, height: u32, kind: LayoutKind) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.create_element("layout-container").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        let name = match &kind {
            LayoutKind::Free { .. } => {
                html_element
                    .class_list()
                    .add_3("free", "block", "relative")
                    .unwrap();
                "Free"
            }
            LayoutKind::Flex { .. } => {
                html_element
                    .class_list()
                    .add_4("flex", "items-center", "justify-center", "flex-wrap")
                    .unwrap();
                "Flex"
            }
            LayoutKind::Grid { cell_size, .. } => {
                html_element
                    .class_list()
                    .add_3("grid", "justify-items-center", "items-center")
                    .unwrap();

                html_element.style().set_property(
                    "grid-template-columns",
                    &format!("repeat({}, {}px)", 10, cell_size),
                );
                html_element.style().set_property(
                    "grid-template-rows",
                    &format!("repeat({}, {}px)", 3, cell_size),
                );

                "Grid"
            }
        };

        // We don't handle width properly ATM
        //
        // html_element
        //     .style()
        //     .set_property("width", &format!("{}px", width))
        //     .unwrap();
        html_element
            .style()
            .set_property("height", &format!("{}px", height))
            .unwrap();

        Self {
            html_element,
            data: Rc::new(Data {
                name: RefCell::new(name.into()),
                height: RefCell::new(height),
                width: RefCell::new(width),
                kind: RefCell::new(kind),

                components: Default::default(),

                hierarchy_data: RefCell::new(HierarchyItemData::new()),
            }),
        }
    }

    /// Creates a new free layout
    pub fn new_free(width: u32, height: u32) -> Self {
        Self::new(
            width,
            height,
            LayoutKind::Free {
                free: FreeLayout::new(width, height),
            },
        )
    }

    /// Creates a new flex layout
    pub fn new_flex(width: u32, height: u32) -> Self {
        Self::new(
            width,
            height,
            LayoutKind::Flex {
                flex: FlexLayout::new(width, height),
            },
        )
    }

    /// Creates a new grid layout
    ///
    /// # Arguments
    /// * `width` - width of a layout in px
    /// * `height` - height of a layout in px
    /// * `cell_size` - size of a grid cell in px
    pub fn new_grid(width: u32) -> Self {
        let cell_size = width / 10;
        let grid_w = (width as f64 / cell_size as f64).round();

        let height = cell_size * 3;
        let grid_h = 3;

        Self::new(
            width,
            height,
            LayoutKind::Grid {
                cell_size,
                grid_background: GridBackground::new(),
                grid_data: Box::new(GridLayout::new(grid_w as usize, grid_h as usize)),
            },
        )
    }

    /// Append the layout to the element
    ///
    /// Used to append layout to the page
    pub fn append_to(&self, parent: &Element) {
        let kind = self.data.kind.borrow();

        if let LayoutKind::Grid {
            grid_background, ..
        } = &*kind
        {
            grid_background.append_to(&self.html_element);
        }

        parent.append_child(&self.html_element).unwrap();
    }
}

impl Layout {
    pub fn close_icon_element(&self) -> HtmlElement {
        let close_icon_element = self
            .html_element
            .query_selector(".container__close-icon")
            .unwrap()
            .unwrap();
        let close_icon_element: HtmlElement = close_icon_element.dyn_into().unwrap();
        close_icon_element
    }

    pub fn set_is_selected(&mut self, is: bool) {
        if is {
            self.html_element.class_list().add_1("selected").unwrap();
        } else {
            self.html_element.class_list().remove_1("selected").unwrap();
        }
    }

    pub fn hierarchy_data(&self) -> Ref<HierarchyItemData> {
        self.data.hierarchy_data.borrow()
    }

    pub fn hierarchy_data_mut(&self) -> RefMut<HierarchyItemData> {
        self.data.hierarchy_data.borrow_mut()
    }

    pub fn name(&self) -> Ref<str> {
        Ref::map(self.data.name.borrow(), |name| name.as_ref())
    }

    pub fn kind(&self) -> Ref<LayoutKind> {
        self.data.kind.borrow()
    }

    pub fn kind_mut(&self) -> RefMut<LayoutKind> {
        self.data.kind.borrow_mut()
    }

    pub fn bounding_client_rect(&self) -> ((f64, f64), (f64, f64)) {
        let bbox = self.html_element.get_bounding_client_rect();
        ((bbox.left(), bbox.top()), (bbox.width(), bbox.height()))
    }

    /// Determines whether the layout contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }

    /// Get list of components stored in a layout
    pub fn components(&self) -> Ref<[Component]> {
        Ref::map(self.data.components.borrow(), |components| {
            components.as_ref()
        })
    }

    pub fn insert_component(&mut self, mut component: Component) {
        self.html_element.append_child(component.element());

        self.data.components.borrow_mut().push(component.clone());

        component.set_layout(Some(self.html_element.clone()));

        // Disabling the "redundand single branch match" lint
        // because we will want to extend this match in future
        #[allow(clippy::single_match)]
        match &mut *self.data.kind.borrow_mut() {
            LayoutKind::Grid {
                grid_data: grid, ..
            } => grid.insert_component(component),
            _ => {}
        };
    }

    pub fn remove_component(&mut self, component: &mut Component) {
        let mut components = self.data.components.borrow_mut();

        let index = components.iter().position(|c| c == &*component);

        if let Some(index) = index {
            components.remove(index);

            // Disabling the "redundand single branch match" lint
            // because we will want to extend this match in future
            #[allow(clippy::single_match)]
            match &mut *self.data.kind.borrow_mut() {
                LayoutKind::Grid {
                    grid_data: grid, ..
                } => grid.remove_component(component),
                _ => {}
            };
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (*self.data.width.borrow(), *self.data.height.borrow())
    }

    pub fn resize(&mut self, width: Option<u32>, height: Option<u32>) {
        // Disabling the "redundand single branch match" lint
        // because we will want to extend this match in future
        #[allow(clippy::single_match)]
        match &mut *self.data.kind.borrow_mut() {
            LayoutKind::Grid {
                grid_data,
                cell_size,
                ..
            } => {
                let new_cell_size = if let Some(width) = width {
                    width / 10
                } else {
                    *cell_size
                };

                let grid_w = 10;

                let grid_h = if let Some(height) = height {
                    let h = (height as f64) / new_cell_size as f64;
                    h.floor() as usize
                } else {
                    grid_data.height()
                };

                if grid_data.resize(grid_w, grid_h) {
                    if let Some(width) = width {
                        self.data.width.replace(width);
                    }
                    if let Some(height) = height {
                        self.data.height.replace(height);
                    }

                    *cell_size = new_cell_size;

                    self.html_element
                        .style()
                        .set_property("height", &format!("{}px", *cell_size as usize * grid_h))
                        .unwrap();

                    self.html_element.style().set_property(
                        "grid-template-columns",
                        &format!("repeat({}, {}px)", grid_w, cell_size),
                    );
                    self.html_element.style().set_property(
                        "grid-template-rows",
                        &format!("repeat({}, {}px)", grid_h, cell_size),
                    );
                }
            }
            _ => {
                if let Some(width) = width {
                    self.data.width.replace(width);
                }
                if let Some(height) = height {
                    self.data.height.replace(height);

                    self.html_element
                        .style()
                        .set_property("height", &format!("{}px", height))
                        .unwrap();
                }
            }
        }
    }

    pub fn remove(self) {
        self.html_element.remove();

        for component in self.data.components.borrow_mut().iter_mut() {
            component.remove();
        }
    }
}

impl Layout {
    pub fn set_flex_justify(&self, class: &str) {
        match &*self.kind() {
            LayoutKind::Flex { .. } => {
                self.html_element.class_list().remove_3(
                    "justify-start",
                    "justify-center",
                    "justify-end",
                );
                self.html_element.class_list().add_1(class);
            }
            _ => {}
        }
    }

    pub fn set_flex_align(&self, class: &str) {
        match &*self.kind() {
            LayoutKind::Flex { .. } => {
                self.html_element
                    .class_list()
                    .remove_3("items-start", "items-center", "items-end");
                self.html_element.class_list().add_1(class);
            }
            _ => {}
        }
    }
}

impl PartialEq<HtmlElement> for Layout {
    fn eq(&self, html_element: &HtmlElement) -> bool {
        &self.html_element == html_element
    }
}

impl PartialEq<Layout> for Layout {
    fn eq(&self, layout: &Layout) -> bool {
        self.html_element == layout.html_element
    }
}
