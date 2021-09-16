use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};

use crate::{editor::Workspace, elements::component::ComponentPropertie, utils};

// This is probably wrong place to store this
// Editor struct itself probably should be responsible for that
struct State {
    selected_component_label: HtmlElement,
    selected: HtmlElement,
}

pub struct ParametersPanel {
    _root: HtmlElement,
    component_list: HtmlElement,
    _state: Rc<RefCell<State>>,
}

impl ParametersPanel {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let root = document.get_element_by_id("parameters").unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();

        let selected_component_label = document
            .get_element_by_id("selected-component-label")
            .unwrap();
        let selected_component_label: HtmlElement = selected_component_label.dyn_into().unwrap();

        let page = document.get_element_by_id("page").unwrap();
        let page: HtmlElement = page.dyn_into().unwrap();

        let state = State {
            selected_component_label,
            selected: root.clone(),
        };

        let state = Rc::new(RefCell::new(state));

        let onmousedown =
            utils::new_listener(state.clone(), |state, event: web_sys::MouseEvent| {
                if let Some(target) = event.target() {
                    let element: Option<HtmlElement> = target.dyn_into().ok();

                    if let Some(element) = element {
                        state
                            .borrow()
                            .selected_component_label
                            .set_inner_text(&format!("<{}/>", element.tag_name().to_lowercase()));
                        state.borrow_mut().selected = element;
                    }
                }
            });

        page.set_onmousedown(Some(&onmousedown));

        let component_list = document.get_element_by_id("component-list").unwrap();
        let component_list: HtmlElement = component_list.dyn_into().unwrap();
        Self {
            _root: root,
            _state: state,
            component_list,
        }
    }

    pub fn update_components_tree(&self, workspace: &Workspace) {
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

                            input.set_checked(
                                if value.unwrap_or_else(|| "false".into()) == "true" {
                                    true
                                } else {
                                    false
                                },
                            );

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
