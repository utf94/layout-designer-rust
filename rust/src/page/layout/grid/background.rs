use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, SvgsvgElement};

use crate::{component::Component, editor::workspace::Workspace, page::layout::grid::Block};

use super::GridLayout;

pub struct GridBackground {
    svg: SvgsvgElement,

    placeholder_elm: HtmlElement,

    placeholder_visible: bool,
    placeholder_denied: bool,

    placeholder_pos: (usize, usize),
    placeholder_size: (usize, usize),
}

impl GridBackground {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let svg = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
            .unwrap();
        let svg: SvgsvgElement = svg.dyn_into().unwrap();
        svg.class_list().add_1("grid-svg").unwrap();
        svg.set_attribute("width", "100%");
        svg.set_attribute("height", "100%");

        svg.set_inner_html(
            r#"
              <rect
                width="100%"
                height="100%"
                fill="url(#grid-pattern)"
              ></rect>
          "#,
        );

        let placeholder_elm = document.create_element("div").unwrap();
        let placeholder_elm: HtmlElement = placeholder_elm.dyn_into().unwrap();
        placeholder_elm
            .class_list()
            .add_1("grid-placeholder")
            .unwrap();

        Self {
            svg,

            placeholder_elm,

            placeholder_visible: false,
            placeholder_denied: false,

            placeholder_pos: (1, 1),
            placeholder_size: (1, 1),
        }
    }

    /// Append the background to the element
    pub fn append_to(&self, parent: &Element) {
        parent.append_child(&self.svg).unwrap();
        parent.append_child(&self.placeholder_elm).unwrap();
    }

    pub fn is_placeholder_denied(&self) -> bool {
        self.placeholder_denied
    }

    fn set_placeholder_deny(&mut self, is: bool) {
        if is != self.placeholder_denied {
            if is {
                self.placeholder_elm.class_list().add_1("deny").unwrap();
            } else {
                self.placeholder_elm.class_list().remove_1("deny").unwrap();
            }
        }

        self.placeholder_denied = is;
    }

    pub fn placeholder_pos(&self) -> (usize, usize) {
        self.placeholder_pos
    }

    pub fn placeholder_size(&self) -> (usize, usize) {
        self.placeholder_size
    }

    pub fn set_placeholder_visible(&mut self, is: bool) {
        if is != self.placeholder_visible {
            if is {
                self.placeholder_elm
                    .style()
                    .set_property("visibility", "visible")
                    .unwrap();
                self.placeholder_elm
                    .style()
                    .set_property("opacity", "1")
                    .unwrap();
            } else {
                self.placeholder_elm
                    .style()
                    .set_property("visibility", "hidden")
                    .unwrap();
                self.placeholder_elm
                    .style()
                    .set_property("opacity", "0")
                    .unwrap();
            }
        }

        self.placeholder_visible = is;
    }

    pub fn update_placeholder(
        &mut self,
        workspace: &Workspace,
        grid_data: &GridLayout,
        component: &Component,
        (x, y): (usize, usize),
        (width, height): (usize, usize),
    ) {
        let is = grid_data.get_block_component_indices(Block {
            x,
            y,
            width,
            height,
        });

        let is = is
            .iter()
            .any(|i| workspace.components().get(*i) != Some(component));

        self.set_placeholder_deny(is);
        self.set_placeholder_visible(true);

        self.placeholder_pos = (x, y);
        self.placeholder_size = (width, height);

        self.placeholder_elm
            .style()
            .set_property("grid-column", &format!("{}/span {}", x, width))
            .unwrap();
        self.placeholder_elm
            .style()
            .set_property("grid-row", &format!("{}/span {}", y, height))
            .unwrap();
    }
}
