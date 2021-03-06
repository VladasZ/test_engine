use crate::ui::view::View;
use crate::ui::ViewBase;
use std::any::Any;
use tools::{AsAny, HasNew};

#[derive(Debug)]
pub struct DPadView {
    base: ViewBase,
}

impl View for DPadView {
    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl HasNew for DPadView {
    fn new() -> DPadView {
        DPadView {
            base: ViewBase::new(),
        }
    }
}

impl AsAny for DPadView {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
