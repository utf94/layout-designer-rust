use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::{component::Component, editor::hierarchy::HierarchyItemData};

pub mod layout;
use layout::Layout;

struct Data {
    /// The name of a page
    ///
    /// For example `Home`, `Contact`, `News`
    name: RefCell<String>,

    /// Width of a page
    width: RefCell<u32>,

    /// List of layouts inside of a page,
    /// laid out one under the other
    layouts: RefCell<Vec<Layout>>,

    /// Hierarchy related data
    hierarchy_data: RefCell<HierarchyItemData>,

    grid_cell_size: RefCell<u32>,
}

/// The representation of a Paga
///
/// Page is an arrea where layouts are added
/// We can have multiple pages in the same workspace
#[derive(Clone)]
pub struct Page {
    /// Root html element of a page
    ///
    /// Layouts are placed inside of it
    pub html_element: HtmlElement,

    /// Inner ref counted data
    data: Rc<Data>,
}

impl Page {
    /// Create a new page, with given name
    pub fn new(name: &str, width: u32) -> Self {
        let name = name.to_owned();

        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.create_element("div").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        html_element
            .style()
            .set_property("width", &format!("{}px", width))
            .unwrap();

        html_element.class_list().add_1("page").unwrap();

        Self {
            html_element,

            data: Rc::new(Data {
                name: RefCell::new(name),
                width: RefCell::new(width),

                layouts: Default::default(),

                hierarchy_data: RefCell::new(HierarchyItemData::new()),
                grid_cell_size: RefCell::new(76),
            }),
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

    pub fn width(&self) -> u32 {
        *self.data.width.borrow()
    }

    #[allow(unused)]
    pub fn set_is_selected(&mut self, is: bool) {
        if is {
            self.html_element.class_list().add_1("selected").unwrap();
        } else {
            self.html_element.class_list().remove_1("selected").unwrap();
        }
    }

    /// Determines whether the workspace contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }

    ///  Fina layout on a page
    pub fn find_layout_by_element(&self, elm: &Element) -> Option<Layout> {
        self.data
            .layouts
            .borrow()
            .iter()
            .find(|layout| layout.contains(elm))
            .cloned()
    }

    ///  Fina component on a page
    pub fn find_component_by_element(&self, elm: &Element) -> Option<Component> {
        let layout = self.find_layout_by_element(elm);
        layout.and_then(|layout| {
            layout
                .components()
                .iter()
                .find(|compoent| compoent.contains(elm))
                .cloned()
        })
    }

    /// Append the page to the element
    ///
    /// Used to append page to the workspace
    pub fn append_to(&self, parent: &Element) {
        parent.append_child(&self.html_element).unwrap();
    }

    /// Get unmutable list of all layouts in a page
    pub fn layouts(&self) -> Ref<[Layout]> {
        Ref::map(self.data.layouts.borrow(), |l| l.as_ref())
    }

    /// Get mutable list of all layouts in a page
    pub fn layouts_mut(&mut self) -> RefMut<[Layout]> {
        RefMut::map(self.data.layouts.borrow_mut(), |l| l.as_mut())
    }

    pub fn remove_layout(&mut self, id: usize) -> Option<Layout> {
        let mut layouts = self.data.layouts.borrow_mut();

        if layouts.get(id).is_some() {
            Some(layouts.remove(id))
        } else {
            None
        }
    }

    /// Insert a layout into a page
    pub fn insert_layout(&mut self, layout: Layout, index: Option<usize>) {
        if let Some(index) = index {
            let mut layouts = self.data.layouts.borrow_mut();
            if let Some(l) = layouts.get(index) {
                l.html_element
                    .before_with_node_1(&layout.html_element)
                    .unwrap();
                layouts.insert(index, layout);
            } else {
                layout.append_to(&self.html_element);
                layouts.push(layout);
            }
        } else {
            layout.append_to(&self.html_element);
            self.data.layouts.borrow_mut().push(layout);
        }
    }

    /// Resize the page
    ///
    /// # Arguments
    /// * `width` - width of a page in px
    pub fn resize(&mut self, width: u32) {
        self.data.grid_cell_size.replace(width / 10);
        self.data.width.replace(width);

        self.html_element
            .style()
            .set_property("width", &format!("{}px", width))
            .unwrap();

        for layout in self.data.layouts.borrow_mut().iter_mut() {
            layout.resize(Some(width), None);
        }
    }
}

impl PartialEq<HtmlElement> for Page {
    fn eq(&self, html_element: &HtmlElement) -> bool {
        &self.html_element == html_element
    }
}
