// src/core/rendering/pipeline.rs

use crate::core::layout::WidgetData;
use crate::core::rendering::{Canvas, RenderingEngine};

/// This function represents a simplified render pipeline.
/// It creates a Canvas and then asks the rendering engine to draw the widgets.
pub fn render_frame<RE: RenderingEngine>(engine: &mut RE, widgets: &[WidgetData]) {
    // In a real app, the canvas would come from a window or a Skia surface.
    let _canvas = Canvas::new(800.0, 600.0);
    engine.draw(widgets);
}
