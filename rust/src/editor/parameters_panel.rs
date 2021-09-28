use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};

use crate::{
    editor::{Selection, Workspace},
    html_elements::component::ComponentPropertie,
    utils,
};

use self::layout_settings::LayoutSettings;

mod layout_settings;

/// The panel on the right side of the editor
pub struct ParametersPanel {
    root: HtmlElement,
    component_list: HtmlElement,

    selected_settings: Option<LayoutSettings>,
}

impl ParametersPanel {
    /// Initialize the parameters panel (on the right side of the editor)
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let root = document.get_element_by_id("parameters").unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();

        let component_list = document.get_element_by_id("component-list").unwrap();
        let component_list: HtmlElement = component_list.dyn_into().unwrap();
        Self {
            root,
            component_list,
            selected_settings: None,
        }
    }

    pub fn set_selected(&mut self, selection: &Selection) {
        match selection {
            Selection::Layout(layout) => {
                if Some(layout) != self.selected_settings.as_ref().map(|s| &s.layout) {
                    let settings = LayoutSettings::new(layout.clone());

                    if let Some(old) = self.selected_settings.take() {
                        self.root.replace_child(&settings.root, &old.root).unwrap();
                    } else {
                        self.root.append_child(&settings.root).unwrap();
                    }

                    self.selected_settings = Some(settings);
                }
            }
            _ => {
                if let Some(old) = self.selected_settings.take() {
                    old.root.remove();
                }
            }
        }
    }

    /// Update the list of components
    pub fn update_debug_components_tree(&self, workspace: &Workspace) {
        self.component_list.set_inner_html("");

        let document = web_sys::window().unwrap().document().unwrap();
        for (_, comp) in workspace.components().iter() {
            let li = document.create_element("li").unwrap();
            let li: HtmlElement = li.dyn_into().unwrap();

            li.set_inner_text(&format!(
                "<{}/>",
                comp.element().descriptor().tag_name().to_lowercase()
            ));

            let properties_div = document.create_element("div").unwrap();

            for param in comp.element().descriptor().parameters().iter() {
                let key = param.name();

                // TODO(poly): Replace this huge messy match with something cleaner
                match param.data_type() {
                    ComponentPropertie::Color => {
                        let div = document.create_element("div").unwrap();
                        let div: HtmlElement = div.dyn_into().unwrap();
                        div.style().set_property("display", "flex").unwrap();
                        div.style().set_property("align-items", "center").unwrap();

                        {
                            let label = document.create_element("div").unwrap();
                            let label: HtmlElement = label.dyn_into().unwrap();

                            label.set_inner_text("Color: ");

                            div.append_child(&label).unwrap();
                        }

                        {
                            let input = document.create_element("input").unwrap();
                            let input: HtmlInputElement = input.dyn_into().unwrap();

                            input.set_class_name("color-picker");
                            input.set_type("color");

                            let value = comp.element().get_attribute(&key);

                            input.set_value(&value.unwrap_or_else(|| "#2ecc71".into()));

                            let cb = utils::new_listener(
                                (comp.element().clone(), key.clone()),
                                |(element, key), e: web_sys::InputEvent| {
                                    let input: HtmlInputElement =
                                        e.target().unwrap().dyn_into().unwrap();
                                    element.set_attribute(key, &input.value()).unwrap();
                                },
                            );

                            input.set_oninput(Some(&cb));

                            div.append_child(&input).unwrap();
                        }

                        properties_div.append_child(&div).unwrap();
                    }
                    ComponentPropertie::Text => {
                        let div = document.create_element("div").unwrap();
                        let div: HtmlElement = div.dyn_into().unwrap();
                        div.style().set_property("display", "flex").unwrap();
                        div.style().set_property("align-items", "center").unwrap();

                        {
                            let label = document.create_element("div").unwrap();
                            let label: HtmlElement = label.dyn_into().unwrap();

                            label.set_inner_text(&key);

                            div.append_child(&label).unwrap();
                        }

                        {
                            let input = document.create_element("input").unwrap();
                            let input: HtmlInputElement = input.dyn_into().unwrap();

                            input.set_class_name("text-picker");
                            input.set_type("text");

                            let value = comp.element().get_attribute(&key);

                            input.set_value(&value.unwrap_or_else(|| "".into()));

                            let cb = utils::new_listener(
                                (comp.element().clone(), key.clone()),
                                |(element, key), e: web_sys::InputEvent| {
                                    let input: HtmlInputElement =
                                        e.target().unwrap().dyn_into().unwrap();
                                    element.set_attribute(key, &input.value()).unwrap();
                                },
                            );

                            input.set_oninput(Some(&cb));

                            div.append_child(&input).unwrap();
                        }

                        properties_div.append_child(&div).unwrap();
                    }
                    ComponentPropertie::Number => {
                        let div = document.create_element("div").unwrap();
                        let div: HtmlElement = div.dyn_into().unwrap();
                        div.style().set_property("display", "flex").unwrap();
                        div.style().set_property("align-items", "center").unwrap();

                        {
                            let label = document.create_element("div").unwrap();
                            let label: HtmlElement = label.dyn_into().unwrap();
                            label.style().set_property("margin-right", "2px").unwrap();

                            label.set_inner_text(&key);

                            div.append_child(&label).unwrap();
                        }

                        {
                            let input = document.create_element("input").unwrap();
                            let input: HtmlInputElement = input.dyn_into().unwrap();

                            input.set_class_name("text-picker");
                            input.set_type("number");

                            let value = comp.element().get_attribute(&key);

                            input.set_value(&value.unwrap_or_else(|| "0".into()));

                            let cb = utils::new_listener(
                                (comp.element().clone(), key.clone()),
                                |(element, key), e: web_sys::InputEvent| {
                                    let input: HtmlInputElement =
                                        e.target().unwrap().dyn_into().unwrap();
                                    element.set_attribute(key, &input.value()).unwrap();
                                },
                            );

                            input.set_oninput(Some(&cb));

                            div.append_child(&input).unwrap();
                        }

                        properties_div.append_child(&div).unwrap();
                    }
                    ComponentPropertie::Boolean => {
                        let div = document.create_element("div").unwrap();
                        let div: HtmlElement = div.dyn_into().unwrap();
                        div.style().set_property("display", "flex").unwrap();
                        div.style().set_property("align-items", "center").unwrap();

                        {
                            let label = document.create_element("div").unwrap();
                            let label: HtmlElement = label.dyn_into().unwrap();
                            label.style().set_property("margin-right", "2px").unwrap();

                            label.set_inner_text(&key);

                            div.append_child(&label).unwrap();
                        }

                        {
                            let input = document.create_element("input").unwrap();
                            let input: HtmlInputElement = input.dyn_into().unwrap();

                            input.set_class_name("text-picker");
                            input.set_type("checkbox");

                            let value = comp.element().get_attribute(&key);

                            input.set_checked(value.unwrap_or_else(|| "false".into()) == "true");

                            let cb = utils::new_listener(
                                (comp.element().clone(), key.clone()),
                                |(element, key), e: web_sys::InputEvent| {
                                    let input: HtmlInputElement =
                                        e.target().unwrap().dyn_into().unwrap();
                                    element
                                        .set_attribute(
                                            key,
                                            if input.checked() { "true" } else { "false" },
                                        )
                                        .unwrap();
                                },
                            );

                            input.set_oninput(Some(&cb));

                            div.append_child(&input).unwrap();
                        }

                        properties_div.append_child(&div).unwrap();
                    }
                    ComponentPropertie::LayoutStyle => {
                        let div = document.create_element("div").unwrap();
                        let div: HtmlElement = div.dyn_into().unwrap();
                        div.style().set_property("display", "flex").unwrap();
                        div.style().set_property("align-items", "center").unwrap();

                        {
                            let label = document.create_element("div").unwrap();
                            let label: HtmlElement = label.dyn_into().unwrap();
                            label.style().set_property("margin-right", "2px").unwrap();

                            label.set_inner_text(&key);

                            div.append_child(&label).unwrap();
                        }

                        {
                            let input = document.create_element("textarea").unwrap();
                            let input: HtmlTextAreaElement = input.dyn_into().unwrap();

                            // input.set_class_name("text-picker");

                            let value = comp.element().get_attribute(&key);

                            input.set_value(&value.unwrap_or_else(|| "".into()));

                            let cb = utils::new_listener(
                                (comp.element().clone(), key.clone()),
                                |(element, key), e: web_sys::InputEvent| {
                                    let input: HtmlTextAreaElement =
                                        e.target().unwrap().dyn_into().unwrap();
                                    element.set_attribute(key, &input.value()).unwrap();
                                },
                            );

                            input.set_oninput(Some(&cb));

                            div.append_child(&input).unwrap();
                        }

                        properties_div.append_child(&div).unwrap();
                    }
                    _ => {}
                }
            }

            li.append_child(&properties_div).unwrap();

            {
                let cb = utils::new_listener(
                    comp.element().clone(),
                    |element, _: web_sys::MouseEvent| {
                        element.class_list().add_1("selected").unwrap();
                    },
                );

                li.set_onmouseenter(Some(&cb));
            }

            {
                let cb = utils::new_listener(
                    comp.element().clone(),
                    |element, _: web_sys::MouseEvent| {
                        element.class_list().remove_1("selected").unwrap();
                    },
                );

                li.set_onmouseleave(Some(&cb));
            }

            self.component_list.append_child(&li).unwrap();
        }
    }
}
