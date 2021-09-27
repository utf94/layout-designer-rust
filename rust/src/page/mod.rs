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
    name: String,

    /// Width of a page
    width: u32,

    /// List of layouts inside of a page,
    /// laid out one under the other
    layouts: Vec<Layout>,

    /// Hierarchy related data
    hierarchy_data: HierarchyItemData,

    grid_cell_size: u32,
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
    data: Rc<RefCell<Data>>,
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

            data: Rc::new(RefCell::new(Data {
                name,
                width,

                layouts: Vec::new(),

                hierarchy_data: HierarchyItemData::new(),
                grid_cell_size: 76,
            })),
        }
    }

    pub fn hierarchy_data(&self) -> Ref<HierarchyItemData> {
        Ref::map(self.data.borrow(), |data| &data.hierarchy_data)
    }

    pub fn hierarchy_data_mut(&self) -> RefMut<HierarchyItemData> {
        RefMut::map(self.data.borrow_mut(), |data| &mut data.hierarchy_data)
    }

    pub fn name(&self) -> Ref<str> {
        Ref::map(self.data.borrow(), |data| data.name.as_ref())
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
            .borrow()
            .layouts
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
        Ref::map(self.data.borrow(), |d| d.layouts.as_ref())
    }

    /// Get mutable list of all layouts in a page
    pub fn layouts_mut(&mut self) -> RefMut<[Layout]> {
        RefMut::map(self.data.borrow_mut(), |d| d.layouts.as_mut())
    }

    pub fn remove_layout(&mut self, id: usize) -> Option<Layout> {
        let mut data = self.data.borrow_mut();

        if data.layouts.get(id).is_some() {
            Some(data.layouts.remove(id))
        } else {
            None
        }
    }

    /// Insert a layout into a page
    pub fn insert_layout(&mut self, layout: Layout, index: Option<usize>) {
        if let Some(index) = index {
            let mut data = self.data.borrow_mut();
            if let Some(l) = data.layouts.get(index) {
                l.html_element
                    .before_with_node_1(&layout.html_element)
                    .unwrap();
                data.layouts.insert(index, layout);
            } else {
                layout.append_to(&self.html_element);
                data.layouts.push(layout);
            }
        } else {
            layout.append_to(&self.html_element);
            self.data.borrow_mut().layouts.push(layout);
        }
    }

    /// Insert a component into a layout inside of this page
    pub fn insert_component_into_layout(
        &mut self,
        layou_elm: &HtmlElement,
        component: &mut Component,
    ) {
        let mut data = self.data.borrow_mut();
        let layout = data.layouts.iter_mut().find(|l| l == &layou_elm);

        if let Some(layout) = layout {
            layout.insert_component(component);
        }
    }

    /// Resize the page
    ///
    /// # Arguments
    /// * `width` - width of a page in px
    pub fn resize(&mut self, width: u32) {
        let mut data = self.data.borrow_mut();

        data.grid_cell_size = width / 10;

        data.width = width;

        self.html_element
            .style()
            .set_property("width", &format!("{}px", width))
            .unwrap();

        for layout in data.layouts.iter_mut() {
            layout.resize(Some(width), None);
        }
    }
}

impl PartialEq<HtmlElement> for Page {
    fn eq(&self, html_element: &HtmlElement) -> bool {
        &self.html_element == html_element
    }
}
