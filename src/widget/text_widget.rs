// src/core/widget/text_widget.rs

use crate::core::layout::{LayoutContext, WidgetData};
use crate::core::rendering::Canvas;

/// A minimal widget trait.
pub trait Widget {
    fn layout(&mut self, ctx: &LayoutContext);
    fn render(&self, canvas: &mut Canvas);
}

/// A simple text widget that draws text on the screen.
pub struct TextWidget {
    pub text: String,
    pub data: WidgetData,
}

impl TextWidget {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            data: WidgetData {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 20.0,
            },
        }
    }
}

impl Widget for TextWidget {
    fn layout(&mut self, _ctx: &LayoutContext) {
        // For now, layout does nothing; widget data remains unchanged.
    }

    fn render(&self, canvas: &mut Canvas) {
        // Render the text widget using the canvas abstraction.
        canvas.draw_text(&self.text, self.data.x, self.data.y, 16.0);
    }
}
