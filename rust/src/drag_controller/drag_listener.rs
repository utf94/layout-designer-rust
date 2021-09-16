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
pub fn add_drag_listener<MD>(element: &HtmlElement, mouse_down: MD)
where
    MD: FnMut(web_sys::MouseEvent) -> Box<dyn FnMut(DragEvent)> + 'static,
{
    let mouse_down =
        utils::new_listener(mouse_down, move |mouse_down, event: web_sys::MouseEvent| {
            event.prevent_default();

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // Check if there are other drags in progress
            // If so just ignore the event
            if DRAGING.load(atomic::Ordering::Acquire) {
                return;
            } else {
                // Drag is now in progress
                DRAGING.store(true, atomic::Ordering::Release);
            }

            // Call `mouse_down` callback provided by the caller
            //
            // It returns a new callback that we will use to handle drage events
            let cb = mouse_down(event);
            // It is needed in 2 callback so we ref count it
            let cb = Rc::new(RefCell::new(cb));

            let onmousemove = utils::new_listener(cb.clone(), |cb, event: web_sys::MouseEvent| {
                event.prevent_default();

                // Notify the caller that mouse was moved
                let mut cb = cb.borrow_mut();
                cb(DragEvent::MouseMove(event));
            });

            let onmouseup = utils::new_listener(
                (document.clone(), cb),
                |(document, cb), event: web_sys::MouseEvent| {
                    document.set_onmousemove(None);
                    document.set_onmouseup(None);

                    // Notify the caller that mouse is up, and drag has ended
                    let mut cb = cb.borrow_mut();
                    cb(DragEvent::MouseUp(event));

                    // Drag is no longer in progress
                    DRAGING.store(false, atomic::Ordering::Release);
                },
            );

            document.set_onmousemove(Some(&onmousemove));
            document.set_onmouseup(Some(&onmouseup));
        });

    element.set_onmousedown(Some(&mouse_down));
}
