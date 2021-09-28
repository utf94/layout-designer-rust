use std::{cell::RefCell, rc::Rc};

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::page::layout::Layout;

pub struct Icon {
    name: &'static str,
    root: HtmlElement,
    click_listener: Option<EventListener>,
}

impl Icon {
    fn new(name: &'static str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let root = document.create_element("div").unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();

        root.class_list().add_1("icon-btn-container").unwrap();

        let img = document.create_element("img").unwrap();
        img.set_attribute("src", &format!("./img/icons/flex/{}.svg", name))
            .unwrap();

        root.append_child(&img).unwrap();

        Self {
            name,
            root,
            click_listener: None,
        }
    }

    fn set_active(&self, is: bool) {
        if is {
            self.root.class_list().add_1("active").unwrap();
        } else {
            self.root.class_list().remove_1("active").unwrap();
        }
    }

    fn connect<F: FnMut(&'static str) + 'static>(&mut self, mut cb: F) {
        let name = self.name;
        let listener = EventListener::new(&self.root, "click", move |_| cb(name));

        self.click_listener = Some(listener);
    }
}

pub struct Icons {
    root: HtmlElement,

    justify_icons: [Icon; 3],
    align_icons: [Icon; 3],
    _more_icon: Icon,
}

impl Icons {
    fn new(layout: &Layout) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let root = document.create_element("div").unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();

        root.style().set_property("display", "flex").unwrap();

        let justify_icons = [
            Icon::new("justify-start"),
            Icon::new("justify-center"),
            Icon::new("justify-end"),
        ];

        if layout.html_element.class_list().contains("justify-start") {
            justify_icons[0].set_active(true);
        } else if layout.html_element.class_list().contains("justify-center") {
            justify_icons[1].set_active(true);
        } else if layout.html_element.class_list().contains("justify-end") {
            justify_icons[2].set_active(true);
        };

        for icon in justify_icons.iter() {
            root.append_child(&icon.root).unwrap();
        }

        let align_icons = [
            Icon::new("items-start"),
            Icon::new("items-center"),
            Icon::new("items-end"),
        ];

        if layout.html_element.class_list().contains("items-start") {
            align_icons[0].set_active(true);
        } else if layout.html_element.class_list().contains("items-center") {
            align_icons[1].set_active(true);
        } else if layout.html_element.class_list().contains("items-end") {
            align_icons[2].set_active(true);
        };

        for icon in align_icons.iter() {
            root.append_child(&icon.root).unwrap();
        }

        let more_icon = Icon::new("more");
        root.append_child(&more_icon.root).unwrap();

        Self {
            root,

            justify_icons,
            align_icons,
            _more_icon: more_icon,
        }
    }

    pub fn connect_justify<F: FnMut(&'static str) + 'static>(&mut self, cb: F) {
        let cb = Rc::new(RefCell::new(cb));

        let roots: [HtmlElement; 3] = [
            self.justify_icons[0].root.clone(),
            self.justify_icons[1].root.clone(),
            self.justify_icons[2].root.clone(),
        ];

        for i in self.justify_icons.iter_mut() {
            let roots = roots.clone();
            let cb = cb.clone();

            let self_root = i.root.clone();
            i.connect(move |name| {
                for root in roots.iter() {
                    root.class_list().remove_1("active").unwrap();
                }

                self_root.class_list().add_1("active").unwrap();

                cb.borrow_mut()(name);
            });
        }
    }

    pub fn connect_align<F: FnMut(&'static str) + 'static>(&mut self, cb: F) {
        let cb = Rc::new(RefCell::new(cb));

        let roots: [HtmlElement; 3] = [
            self.align_icons[0].root.clone(),
            self.align_icons[1].root.clone(),
            self.align_icons[2].root.clone(),
        ];

        for i in self.align_icons.iter_mut() {
            let roots = roots.clone();
            let cb = cb.clone();

            let self_root = i.root.clone();
            i.connect(move |name| {
                for root in roots.iter() {
                    root.class_list().remove_1("active").unwrap();
                }

                self_root.class_list().add_1("active").unwrap();

                cb.borrow_mut()(name);
            });
        }
    }
}

pub struct FlexSettings {
    pub root: HtmlElement,
    pub icons: Icons,
}

impl FlexSettings {
    pub fn new(layout: &Layout) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let root = document.create_element("section").unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();

        root.append_child(&super::title("Flex")).unwrap();

        let icons = Icons::new(layout);

        root.append_child(&icons.root).unwrap();

        Self { root, icons }
    }
}
