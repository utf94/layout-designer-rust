#![allow(unused)]
use super::Workspace;
use crate::page::Page;
/// Include relevent crates and modules
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

pub struct HierarchyItemData {}

impl HierarchyItemData {
    pub fn new() -> Self {
        Self {}
    }
}

/// Hierarchy Struct to represent inspector in editor
pub struct Hierarchy {
    /// Root html element of the Hierarchy
    html_element: HtmlElement,
}

/// Methods for Hierarchy Struct
impl Hierarchy {
    /// Create new instance of the hierarchy
    pub fn new() -> Self {
        // Get the parent html element of old hierarchy
        let document = web_sys::window().unwrap().document().unwrap();
        let html_element = document.get_element_by_id("pages-panel").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        // Get the html element for hierarchy
        let inspector_tree = document.get_element_by_id("inspector-tree").unwrap();
        let inspector_tree: HtmlElement = inspector_tree.dyn_into().unwrap();

        // Create the html element for new hierarchy
        let new_inspector_tree = document.create_element("div").unwrap();
        let new_inspector_tree: HtmlElement = new_inspector_tree.dyn_into().unwrap();
        new_inspector_tree.set_id(&"inspector-tree");

        // Replace the html element for hierarchy
        html_element.replace_child(&new_inspector_tree, &inspector_tree);
        Self { html_element }
    }

    /// Update the hierarchy using workspace
    pub fn update(&self, workspace: &Workspace) {
        // Get the html element of old hierarchy
        let document = web_sys::window().unwrap().document().unwrap();
        let inspector_tree = document.get_element_by_id("inspector-tree").unwrap();
        let inspector_tree: HtmlElement = inspector_tree.dyn_into().unwrap();

        // Create the html element for new hierarchy
        let new_inspector_tree = document.create_element("div").unwrap();
        let new_inspector_tree: HtmlElement = new_inspector_tree.dyn_into().unwrap();
        new_inspector_tree.set_id(&"inspector-tree");

        // Add all pages in hierarchy
        for page in workspace.pages() {
            // Create html elements of page
            let page_item_element = document.create_element("div").unwrap();
            let page_item_element: HtmlElement = page_item_element.dyn_into().unwrap();
            page_item_element.set_class_name("page-item");

            let page_item_header_element = document.create_element("header").unwrap();
            let page_item_header_element: HtmlElement =
                page_item_header_element.dyn_into().unwrap();

            let page_item_icon_element = document.create_element("div").unwrap();
            let page_item_icon_element: HtmlElement = page_item_icon_element.dyn_into().unwrap();
            page_item_icon_element.set_class_name("page-item__icon");

            let page_item_icon_img_element = document.create_element("img").unwrap();
            let page_item_icon_img_element: HtmlElement =
                page_item_icon_img_element.dyn_into().unwrap();
            page_item_icon_img_element.set_attribute(&"src", &"./img/icons/arrow_down.svg");

            let page_item_name_element = document.create_element("div").unwrap();
            let page_item_name_element: HtmlElement = page_item_name_element.dyn_into().unwrap();
            page_item_name_element.set_inner_text(&page.name());

            // Add html elements of page in hierarchy
            page_item_icon_element.append_child(&page_item_icon_img_element);
            page_item_header_element.append_child(&page_item_icon_element);
            page_item_header_element.append_child(&page_item_name_element);
            page_item_element.append_child(&page_item_header_element);

            // Add all layouts in page
            for layout in page.layouts().iter() {
                // Create html elements of layout
                let layout_item_element = document.create_element("div").unwrap();
                let layout_item_element: HtmlElement = layout_item_element.dyn_into().unwrap();
                layout_item_element.set_class_name("page-item__children");

                let layout_item_header_element = document.create_element("header").unwrap();
                let layout_item_header_element: HtmlElement =
                    layout_item_header_element.dyn_into().unwrap();

                let layout_item_icon_element = document.create_element("div").unwrap();
                let layout_item_icon_element: HtmlElement =
                    layout_item_icon_element.dyn_into().unwrap();
                layout_item_icon_element.set_class_name("page-item__icon");

                let layout_item_icon_img_element = document.create_element("img").unwrap();
                let layout_item_icon_img_element: HtmlElement =
                    layout_item_icon_img_element.dyn_into().unwrap();
                layout_item_icon_img_element.set_attribute(&"src", &"./img/icons/arrow_down.svg");

                let layout_item_name_element = document.create_element("div").unwrap();
                let layout_item_name_element: HtmlElement =
                    layout_item_name_element.dyn_into().unwrap();
                layout_item_name_element.set_inner_text(&layout.name());

                // Add html elements of layout in page
                layout_item_icon_element.append_child(&layout_item_icon_img_element);
                layout_item_header_element.append_child(&layout_item_icon_element);
                layout_item_header_element.append_child(&layout_item_name_element);
                layout_item_element.append_child(&layout_item_header_element);

                let layout_children = document.create_element("div").unwrap();
                let layout_children: HtmlElement = layout_children.dyn_into().unwrap();
                layout_children
                    .class_list()
                    .add_1("page-item__layout__children");

                // Add all components in layout
                for component in layout.components().iter() {
                    let component_item = document.create_element("div").unwrap();
                    let component_item: HtmlElement = component_item.dyn_into().unwrap();
                    component_item.class_list().add_1("page-item__component");

                    let icon_element = document.create_element("div").unwrap();
                    let icon_element: HtmlElement = icon_element.dyn_into().unwrap();
                    icon_element.set_class_name("page-item__component_icon");
                    component_item.append_child(&icon_element);

                    let icon_img_element = document.create_element("img").unwrap();
                    let icon_img_element: HtmlElement = icon_img_element.dyn_into().unwrap();
                    icon_img_element.set_attribute(&"src", &"./img/icons/check.svg");

                    icon_element.append_child(&icon_img_element);

                    let component_title = document.create_element("div").unwrap();

                    let component_title: HtmlElement = component_title.dyn_into().unwrap();
                    component_title.set_inner_text(&component.name());
                    component_item.append_child(&component_title);

                    layout_children.append_child(&component_item);
                }

                layout_item_element.append_child(&layout_children);

                // Add layout in page
                page_item_element.append_child(&layout_item_element);
            }

            // Add page in hierarchy
            new_inspector_tree.append_child(&page_item_element);
        }

        // Replace the html element for hierarchy
        self.html_element
            .replace_child(&new_inspector_tree, &inspector_tree);
    }

    /// Determines whether the tree contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }
}

/// Partial equivalence relation for Hierarchy Struct
impl PartialEq<Element> for Hierarchy {
    fn eq(&self, html_element: &Element) -> bool {
        let root: &Element = self.html_element.as_ref();
        root == html_element
    }
}
