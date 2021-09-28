use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn icon(name: &str) -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();

    let root = document.create_element("div").unwrap();
    let root: HtmlElement = root.dyn_into().unwrap();

    root.class_list().add_1("icon-btn-container").unwrap();

    let img = document.create_element("img").unwrap();
    img.set_attribute("src", &format!("./img/icons/flex/{}.svg", name))
        .unwrap();

    root.append_child(&img).unwrap();
    root
}

fn icons() -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();

    let root = document.create_element("div").unwrap();
    let root: HtmlElement = root.dyn_into().unwrap();

    root.class_list().add_1("felx-layout-icons").unwrap();

    let icons = [
        icon("justify-left"),
        icon("justify-center"),
        icon("justify-right"),
        //
        icon("align-top"),
        icon("align-center"),
        icon("align-bottom"),
        //
        icon("more"),
    ];

    for icon in icons.iter() {
        root.append_child(&icon).unwrap();
    }

    root
}

pub fn settings() -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();

    let root = document.create_element("section").unwrap();
    let root: HtmlElement = root.dyn_into().unwrap();

    root.append_child(&super::title("Flex")).unwrap();
    root.append_child(&icons()).unwrap();
    root
}
