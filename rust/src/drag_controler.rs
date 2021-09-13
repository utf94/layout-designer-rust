use crate::component::{Component, ComponentSource};

mod drag_transform;

mod drag_listener;
use drag_listener::{add_drag_listener, DragEvent};

mod move_controler;
use move_controler::MoveControler;

/// Register a drag listener on a source
pub fn add_drag_listener_from_source(component_source: &ComponentSource) {
    let root = &component_source.root;
    let component_source = component_source.clone();

    add_drag_listener(root, move |event| {
        let btn = event.button();

        if btn == 0 {
            let component = component_source.new_instance();

            let mut controler = MoveControler::new(component);

            Box::new(move |event| match event {
                DragEvent::MouseMove(event) => {
                    controler.mouse_move(event);
                }
                DragEvent::MouseUp(event) => {
                    add_drag_listener_from_instance(&controler.component);

                    controler.mouse_up(event);

                    // Element was parented, stop the spawn animation
                    controler
                        .component
                        .element()
                        .class_list()
                        .remove_1("spawn-animation")
                        .unwrap();
                }
            })
        } else {
            Box::new(|_| {})
        }
    });
}

/// Register a drag listener on an instance
fn add_drag_listener_from_instance(component: &Component) {
    let element = component.element();
    let component = component.clone();
    add_drag_listener(element, move |event| {
        let btn = event.button();

        if btn == 0 {
            let mut controler = MoveControler::new(component.clone());

            Box::new(move |event| match event {
                DragEvent::MouseMove(event) => {
                    controler.mouse_move(event);
                }
                DragEvent::MouseUp(event) => {
                    controler.mouse_up(event);
                }
            })
        } else if btn == 2 {
            // TODO: Resize
            // let mut controler = MoveControler::new(component.clone());

            // Box::new(move |event| match event {
            //     DragEvent::MouseMove(event) => {
            //         controler.mouse_move(event);
            //     }
            //     DragEvent::MouseUp(event) => {
            //         controler.mouse_up(event);
            //     }
            // })

            Box::new(|_| {})
        } else {
            Box::new(|_| {})
        }
    });
}
