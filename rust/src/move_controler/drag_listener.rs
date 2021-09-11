use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{self, AtomicBool},
};

use web_sys::HtmlElement;

use crate::utils;

pub enum DragEvent {
    MouseMove(web_sys::MouseEvent),
    MouseUp(web_sys::MouseEvent),
}

static DRAGING: AtomicBool = AtomicBool::new(false);

/// Register a drag listener on a source
pub fn add_drag_listener<MD, CB>(element: &HtmlElement, mouse_down: MD)
where
    MD: FnMut(web_sys::MouseEvent) -> CB + 'static,
    CB: FnMut(DragEvent) + 'static,
{
    let mouse_down = utils::new_listener(mouse_down, |mouse_down, event: web_sys::MouseEvent| {
        event.prevent_default();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        if DRAGING.load(atomic::Ordering::Acquire) {
            return;
        } else {
            DRAGING.store(true, atomic::Ordering::Release);
        }

        let cb = mouse_down(event);
        let cb = Rc::new(RefCell::new(cb));

        let onmousemove = utils::new_listener(cb.clone(), |cb, event: web_sys::MouseEvent| {
            event.prevent_default();

            let mut cb = cb.borrow_mut();
            cb(DragEvent::MouseMove(event));
        });

        let onmouseup = utils::new_listener(
            (document.clone(), cb),
            |(document, cb), event: web_sys::MouseEvent| {
                document.set_onmousemove(None);
                document.set_onmouseup(None);

                let mut cb = cb.borrow_mut();
                cb(DragEvent::MouseUp(event));

                DRAGING.store(false, atomic::Ordering::Release);
            },
        );

        document.set_onmousemove(Some(&onmousemove));
        document.set_onmouseup(Some(&onmouseup));
    });

    element.set_onmousedown(Some(&mouse_down));
}
