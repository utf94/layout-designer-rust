#![allow(unused)]

use std::{
    cell::{Ref, RefCell},
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

use crate::component::Component;

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
        cell_size: usize,
        /// Data related to grid layout implementation
        grid_data: GridLayout,
    },
}

struct Data {
    /// Height of a layout
    height: usize,
    /// Width of a layout
    width: usize,
    /// Layout kind specyfic data
    kind: LayoutKind,
    /// Children of a layout
    components: Vec<Component>,
}

/// Layout struct that represents layout node
#[derive(Clone)]
pub struct Layout {
    /// Root html element of a layout
    html_element: HtmlElement,
    /// Inner ref counted data
    data: Rc<RefCell<Data>>,
}

// Init related methods:
impl Layout {
    /// Creates a new layout
    ///
    /// It will initialize the layout data,
    /// and also create a new html element that represents the layout
    pub fn new(width: usize, height: usize, kind: LayoutKind) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.create_element("layout-container").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        match &kind {
            LayoutKind::Free { .. } => {
                html_element.class_list().add_2("free", "block").unwrap();
            }
            LayoutKind::Flex { .. } => {
                html_element
                    .class_list()
                    .add_4("flex", "items-center", "justify-evenly", "flex-wrap")
                    .unwrap();
            }
            LayoutKind::Grid { .. } => {
                html_element
                    .class_list()
                    .add_4(
                        "grid",
                        "grid-cols-10",
                        "justify-items-center",
                        "items-center",
                    )
                    .unwrap();
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
            data: Rc::new(RefCell::new(Data {
                height,
                width,
                kind,

                components: Vec::new(),
            })),
        }
    }

    /// Creates a new free layout
    pub fn new_free(width: usize, height: usize) -> Self {
        Self::new(
            width,
            height,
            LayoutKind::Free {
                free: FreeLayout::new(width, height),
            },
        )
    }

    /// Creates a new flex layout
    pub fn new_flex(width: usize, height: usize) -> Self {
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
    pub fn new_grid(width: usize, height: usize, cell_size: usize) -> Self {
        let grid_w = (width as f64 / cell_size as f64).round();
        let grid_h = (height as f64 / cell_size as f64).round();

        Self::new(
            width,
            height,
            LayoutKind::Grid {
                cell_size,
                grid_data: GridLayout::new(grid_w as usize, grid_h as usize),
            },
        )
    }

    /// Append the layout to the element
    ///
    /// Used to append layout to the page
    pub fn append_to(&self, parent: &Element) {
        parent.append_child(&self.html_element).unwrap();
    }
}

impl Layout {
    pub fn kind(&self) -> Ref<LayoutKind> {
        Ref::map(self.data.borrow(), |r| &r.kind)
    }

    /// Determines whether the layout contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }

    /// Get list of components stored in a layout
    pub fn components(&self) -> Ref<[Component]> {
        Ref::map(self.data.borrow(), |r| r.components.as_ref())
    }

    pub fn insert_component(&mut self, component: &mut Component) {
        self.html_element.append_child(component.element());

        let mut data = self.data.borrow_mut();
        data.components.push(component.clone());

        component.set_layout(Some(self.html_element.clone()));

        // Disabling the "redundand single branch match" lint
        // because we will want to extend this match in future
        #[allow(clippy::single_match)]
        match &mut data.kind {
            LayoutKind::Grid {
                grid_data: grid, ..
            } => grid.insert_component(component),
            _ => {}
        };
    }

    pub fn remove_component(&mut self, component: &mut Component) {
        let mut data = self.data.borrow_mut();

        let index = data.components.iter().position(|c| c == &*component);

        if let Some(index) = index {
            data.components.remove(index);

            // Disabling the "redundand single branch match" lint
            // because we will want to extend this match in future
            #[allow(clippy::single_match)]
            match &mut data.kind {
                LayoutKind::Grid {
                    grid_data: grid, ..
                } => grid.remove_component(component),
                _ => {}
            };
        }
    }

    pub fn size(&self) -> (usize, usize) {
        let data = self.data.borrow();
        (data.width, data.height)
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let mut data = self.data.borrow_mut();

        data.width = width;
        data.height = height;

        // Disabling the "redundand single branch match" lint
        // because we will want to extend this match in future
        #[allow(clippy::single_match)]
        match &mut data.kind {
            LayoutKind::Grid {
                grid_data: grid,
                cell_size,
            } => {
                let grid_w = (width as f64 / *cell_size as f64).round();
                let grid_h = (height as f64 / *cell_size as f64).round();

                grid.resize(width as usize, height as usize);
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
