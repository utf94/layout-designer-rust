use crate::component::Component;

/// Struct responsible for the CSS transforms during the drag
pub struct CssMoveTransform {
    pub component: Component,

    last_x: i32,
    last_y: i32,

    absolute_pos_x: i32,
    absolute_pos_y: i32,

    start_x: i32,
    start_y: i32,
}

impl CssMoveTransform {
    /// Start the move
    pub fn start(component: Component, x: i32, y: i32) -> CssMoveTransform {
        let last_x = x;
        let last_y = y;

        let start_x = x;
        let start_y = y;

        let absolute_pos_x = component.element().offset_left();
        let absolute_pos_y = component.element().offset_top();

        Self {
            component,
            last_x,
            last_y,
            absolute_pos_x,
            absolute_pos_y,
            start_x,
            start_y,
        }
    }

    /// Called when mouse is being draged
    pub fn drag(&mut self, x: i32, y: i32) {
        self.absolute_pos_x -= self.last_x - x;
        self.absolute_pos_y -= self.last_y - y;

        self.last_x = x;
        self.last_y = y;

        self.component
            .element()
            .style()
            .set_property(
                "transform",
                &format!("translate({}px, {}px)", x - self.start_x, y - self.start_y),
            )
            .unwrap();
    }

    /// Strop the css transfrom move
    pub fn stop(&mut self, offset: (i32, i32)) {
        self.component.set_position(
            self.absolute_pos_x - offset.0,
            self.absolute_pos_y - offset.1,
        );

        // After the move is done we should no longer have any transfroms on the component
        // It should be positioned by the layout from now on
        self.component
            .element()
            .style()
            .remove_property("transform")
            .unwrap();
    }
}
