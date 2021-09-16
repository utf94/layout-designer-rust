use crate::component::Component;

/// Struct responsible for the CSS transforms during the drag
pub struct CssMoveTransform {
    component: Component,

    /// Current absolute position of moved component
    absolute_pos: (i32, i32),

    /// Last know pos, used to calculate delta
    last_pos: (i32, i32),
    /// Starting pos
    start_pos: (i32, i32),
}

impl CssMoveTransform {
    /// Start the move
    pub fn start(component: Component, x: i32, y: i32) -> CssMoveTransform {
        let last_x = x;
        let last_y = y;

        let start_x = x;
        let start_y = y;

        let absolute_x = component.element().offset_left();
        let absolute_y = component.element().offset_top();

        Self {
            component,
            last_pos: (last_x, last_y),
            absolute_pos: (absolute_x, absolute_y),
            start_pos: (start_x, start_y),
        }
    }

    /// Called when mouse is being draged
    pub fn drag(&mut self, x: i32, y: i32) {
        self.absolute_pos.0 -= self.last_pos.0 - x;
        self.absolute_pos.1 -= self.last_pos.1 - y;

        self.last_pos.0 = x;
        self.last_pos.1 = y;

        self.component
            .element()
            .style()
            .set_property(
                "transform",
                &format!(
                    "translate({}px, {}px)",
                    x - self.start_pos.0,
                    y - self.start_pos.1
                ),
            )
            .unwrap();
    }

    /// Strop the css transfrom move
    pub fn stop(&mut self) -> (i32, i32) {
        // After the move is done we should no longer have any transfroms on the component
        // It should be positioned by the layout from now on
        self.component
            .element()
            .style()
            .remove_property("transform")
            .unwrap();

        self.absolute_pos
    }
}
