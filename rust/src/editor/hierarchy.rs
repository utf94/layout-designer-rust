#![allow(unused)]
/// Include relevent crates and modules
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use super::Workspace;
use crate::page::Page;

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
        Self {
            html_element,
        }
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
        for i in 0..workspace.pages().len() {
            // Get the name of page
            let mut page_name = i.to_string();
            page_name.push_str(" Page");

            // Create html elements of page
            let page_item_element = document.create_element("div").unwrap();
            let page_item_element: HtmlElement = page_item_element.dyn_into().unwrap();
            page_item_element.set_class_name("page-item");

            let page_item_header_element = document.create_element("header").unwrap();
            let page_item_header_element: HtmlElement = page_item_header_element.dyn_into().unwrap();

            let page_item_icon_element = document.create_element("div").unwrap();
            let page_item_icon_element: HtmlElement = page_item_icon_element.dyn_into().unwrap();
            page_item_icon_element.set_class_name("page-item__icon");

            let page_item_icon_img_element = document.create_element("img").unwrap();
            let page_item_icon_img_element: HtmlElement = page_item_icon_img_element.dyn_into().unwrap();
            page_item_icon_img_element.set_attribute(&"src", &"./img/icons/arrow_down.svg");

            let page_item_name_element = document.create_element("div").unwrap();
            let page_item_name_element: HtmlElement = page_item_name_element.dyn_into().unwrap();
            page_item_name_element.set_inner_text(&page_name);

            // Add html elements of page in hierarchy
            page_item_icon_element.append_child(&page_item_icon_img_element);
            page_item_header_element.append_child(&page_item_icon_element);
            page_item_header_element.append_child(&page_item_name_element);
            page_item_element.append_child(&page_item_header_element);

            // Add all layouts in page
            for j in 0..3 {
                // Get the name of layout
                let mut layout_name = i.to_string();
                layout_name.push_str(&".");
                layout_name.push_str(&j.to_string());
                layout_name.push_str(" Layout");

                // Create html elements of layout
                let layout_item_element = document.create_element("div").unwrap();
                let layout_item_element: HtmlElement = layout_item_element.dyn_into().unwrap();
                layout_item_element.set_class_name("page-item__children");

                let layout_item_header_element = document.create_element("header").unwrap();
                let layout_item_header_element: HtmlElement = layout_item_header_element.dyn_into().unwrap();

                let layout_item_icon_element = document.create_element("div").unwrap();
                let layout_item_icon_element: HtmlElement = layout_item_icon_element.dyn_into().unwrap();
                layout_item_icon_element.set_class_name("page-item__icon");

                let layout_item_icon_img_element = document.create_element("img").unwrap();
                let layout_item_icon_img_element: HtmlElement = layout_item_icon_img_element.dyn_into().unwrap();
                layout_item_icon_img_element.set_attribute(&"src", &"./img/icons/arrow_down.svg");

                let layout_item_name_element = document.create_element("div").unwrap();
                let layout_item_name_element: HtmlElement = layout_item_name_element.dyn_into().unwrap();
                layout_item_name_element.set_inner_text(&layout_name);

                // Add html elements of layout in page
                layout_item_icon_element.append_child(&layout_item_icon_img_element);
                layout_item_header_element.append_child(&layout_item_icon_element);
                layout_item_header_element.append_child(&layout_item_name_element);
                layout_item_element.append_child(&layout_item_header_element);

                // Add layout in page
                page_item_element.append_child(&layout_item_element);
            }

            // Add page in hierarchy
            new_inspector_tree.append_child(&page_item_element);
        }

        // Replace the html element for hierarchy
        self.html_element.replace_child(&new_inspector_tree, &inspector_tree);
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
