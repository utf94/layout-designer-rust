#![allow(unused)]
/// Include relevent crates and modules
use super::Workspace;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use crate::{
    component::Component,
    page::{layout::Layout, Page},
};

/// Enum to identify the type of html element
enum ElementType {
    PageElement,
    LayoutElement,
    ComponentElement,
    None
}

/// Enum to return the result for on click event on html element inside hirarchy tree
pub enum ClickResult {
    Page(Page),
    Layout(Layout),
    Component(Component),
    None
}

/// Hierarchy Item Data Struct to represent hierarchy tree related data
pub struct HierarchyItemData {
     /// Html element of the item
     item_html_element: Option<HtmlElement>,
     /// Html element of the icon
     icon_html_element: Option<HtmlElement>,
     /// Intialization status of the item
     init_status: bool,
     /// Collapse status of the item
     collapse_status: bool,
     /// Type of element
     element_type: ElementType,
}

/// Methods for Hierarchy Item Data Struct
impl HierarchyItemData {
    /// Create new instance of the Hierarchy Item Data
    pub fn new() -> Self {
        Self {
            item_html_element: None,
            icon_html_element: None,
            init_status: false,
            collapse_status: false,
            element_type: ElementType::None
        }
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
            let page_item_header_element: HtmlElement = page_item_header_element.dyn_into().unwrap();

            let page_item_icon_element = document.create_element("div").unwrap();
            let page_item_icon_element: HtmlElement = page_item_icon_element.dyn_into().unwrap();
            page_item_icon_element.set_class_name("page-item__icon");

            let page_item_icon_img_element = document.create_element("img").unwrap();
            let page_item_icon_img_element: HtmlElement = page_item_icon_img_element.dyn_into().unwrap();
            page_item_icon_img_element.set_attribute(&"src", &"./img/icons/arrow_down.svg");

            let page_item_name_element = document.create_element("div").unwrap();
            let page_item_name_element: HtmlElement = page_item_name_element.dyn_into().unwrap();
            page_item_name_element.set_inner_text(&page.name());

            // Add html elements of page in hierarchy
            page_item_icon_element.append_child(&page_item_icon_img_element);
            page_item_header_element.append_child(&page_item_icon_element);
            page_item_header_element.append_child(&page_item_name_element);
            page_item_element.append_child(&page_item_header_element);

            // Process Hierarchy Item Data for Page
            let mut page_hierarchy_item_data = page.hierarchy_data_mut();
            if !page_hierarchy_item_data.init_status {
                page_hierarchy_item_data.init_status = true;
            }
            page_hierarchy_item_data.item_html_element = Some(page_item_header_element);
            page_hierarchy_item_data.icon_html_element = Some(page_item_icon_element);
            page_hierarchy_item_data.element_type = ElementType::PageElement;

            // Add all layouts in page
            for layout in page.layouts().iter() {
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
                layout_item_name_element.set_inner_text(&layout.name());

                // Add html elements of layout in page
                layout_item_icon_element.append_child(&layout_item_icon_img_element);
                layout_item_header_element.append_child(&layout_item_icon_element);
                layout_item_header_element.append_child(&layout_item_name_element);
                layout_item_element.append_child(&layout_item_header_element);

                // Process Hierarchy Item Data for Layout
                let mut layout_hierarchy_item_data = layout.hierarchy_data_mut();
                if !layout_hierarchy_item_data.init_status {
                    layout_hierarchy_item_data.init_status = true;
                }
                layout_hierarchy_item_data.item_html_element = Some(layout_item_header_element);
                layout_hierarchy_item_data.icon_html_element = Some(layout_item_icon_element);
                layout_hierarchy_item_data.element_type = ElementType::LayoutElement;

                // Add html element of layout children container
                let layout_item_children_element = document.create_element("div").unwrap();
                let layout_item_children_element: HtmlElement = layout_item_children_element.dyn_into().unwrap();
                layout_item_children_element.class_list().add_1("page-item__layout__children");

                // Add all components in layout
                for component in layout.components().iter() {
                    let component_item_element = document.create_element("div").unwrap();
                    let component_item_element: HtmlElement = component_item_element.dyn_into().unwrap();
                    component_item_element.class_list().add_1("page-item__component");

                    let component_item_icon_element = document.create_element("div").unwrap();
                    let component_item_icon_element: HtmlElement = component_item_icon_element.dyn_into().unwrap();
                    component_item_icon_element.set_class_name("page-item__component_icon");

                    let component_item_icon_img_element = document.create_element("img").unwrap();
                    let component_item_icon_img_element: HtmlElement = component_item_icon_img_element.dyn_into().unwrap();
                    component_item_icon_img_element.set_attribute(&"src", &"./img/icons/check.svg");

                    let component_item_name_element = document.create_element("div").unwrap();
                    let component_item_name_element: HtmlElement = component_item_name_element.dyn_into().unwrap();
                    component_item_name_element.set_inner_text(&component.name());

                    // Add html elements of component in layout children container
                    component_item_icon_element.append_child(&component_item_icon_img_element);
                    component_item_element.append_child(&component_item_icon_element);
                    component_item_element.append_child(&component_item_name_element);
                    layout_item_children_element.append_child(&component_item_element);

                    // Process Hierarchy Item Data for Component
                    let mut component_hierarchy_item_data = component.hierarchy_data_mut();
                    if !component_hierarchy_item_data.init_status {
                        component_hierarchy_item_data.init_status = true;
                    }
                    component_hierarchy_item_data.item_html_element = Some(component_item_element);
                    component_hierarchy_item_data.icon_html_element = Some(component_item_icon_element);
                    component_hierarchy_item_data.element_type = ElementType::ComponentElement;
                }

                // Add layout children container in layout
                layout_item_element.append_child(&layout_item_children_element);

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

    /// On click event when any element inside hierarchy tree is clicked
    pub fn on_click(&mut self, workspace: &Workspace, target: &HtmlElement) -> ClickResult {
        ClickResult::None
    }
}

/// Partial equivalence relation for Hierarchy Struct
impl PartialEq<Element> for Hierarchy {
    fn eq(&self, html_element: &Element) -> bool {
        let root: &Element = self.html_element.as_ref();
        root == html_element
    }
}
