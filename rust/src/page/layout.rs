#![allow(unused)]

use generational_arena::Index;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

mod grid;
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
        /// Data related to grid layout implementation
        grid: GridLayout,
    },
}

/// Layout struct that represents layout node
pub struct Layout {
    /// Root html element of a layout
    html_element: HtmlElement,
    /// Height of a layout
    height: usize,
    /// Width of a layout
    width: usize,
    /// Layout kind specyfic data
    kind: LayoutKind,
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

        let kind_class = match &kind {
            LayoutKind::Free { .. } => "free",
            LayoutKind::Flex { .. } => "flex",
            LayoutKind::Grid { .. } => "grid",
        };

        html_element.class_list().add_1(kind_class).unwrap();

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
            height,
            width,
            kind,
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
    pub fn new_grid(width: usize, height: usize) -> Self {
        Self::new(
            width,
            height,
            LayoutKind::Grid {
                grid: GridLayout::new(width, height),
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
    pub fn insert_component(&mut self, component: (Index, &mut Component)) {
        self.html_element.append_child(component.1.element());

        // Disabling the "redundand single branch match" lint
        // because we will want to extend this match in future
        #[allow(clippy::single_match)]
        match &mut self.kind {
            LayoutKind::Grid { grid } => grid.insert_component(component),
            _ => {}
        };
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;

        // Disabling the "redundand single branch match" lint
        // because we will want to extend this match in future
        #[allow(clippy::single_match)]
        match &mut self.kind {
            LayoutKind::Grid { grid } => grid.resize(width, height),
            _ => {}
        };
    }
}

impl PartialEq<HtmlElement> for Layout {
    fn eq(&self, html_element: &HtmlElement) -> bool {
        &self.html_element == html_element
    }
}
