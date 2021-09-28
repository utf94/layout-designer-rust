use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::page::layout::{Layout, LayoutKind};

mod flex;
mod free;
mod grid;

use flex::FlexSettings;

trait SettingsData {}
impl<T> SettingsData for T {}

fn title(title: &str) -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();

    let root = document.create_element("div").unwrap();
    let root: HtmlElement = root.dyn_into().unwrap();
    root.class_list().add_1("title").unwrap();
    root.set_inner_text(title);
    root
}

pub struct LayoutSettings {
    pub layout: Layout,
    pub root: HtmlElement,
    _data: Box<dyn SettingsData>,
}

impl LayoutSettings {
    pub fn new(layout: Layout) -> Self {
        let (root, _data): (HtmlElement, Box<dyn SettingsData>) = match &*layout.kind() {
            LayoutKind::Free { .. } => {
                let root = free::settings();
                (root, Box::new(()))
            }
            LayoutKind::Flex { .. } => {
                let mut data = FlexSettings::new(&layout);

                {
                    let layout = layout.clone();
                    data.icons.connect_justify(move |name| {
                        layout.set_flex_justify(name);
                    });
                }

                {
                    let layout = layout.clone();
                    data.icons.connect_align(move |name| {
                        layout.set_flex_align(name);
                    });
                }

                (data.root.clone(), Box::new(data))
            }
            LayoutKind::Grid { .. } => {
                let root = grid::settings();
                (root, Box::new(()))
            }
        };

        Self {
            layout,
            root,
            _data,
        }
    }
}
