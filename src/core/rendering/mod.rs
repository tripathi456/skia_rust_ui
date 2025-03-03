// src/core/rendering/mod.rs
pub mod skia_renderer;
pub mod pipeline;

use crate::core::layout::WidgetData;

#[derive(Debug, PartialEq, Clone)]
pub enum DrawCommand {
    Text {
        content: String,
        x: f32,
        y: f32,
        font_size: f32,
    },
    // Future expansions: Rect, Image, etc.
}

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: f32,
    pub height: f32,
    pub commands: Vec<DrawCommand>,
}

impl Canvas {
    pub fn new(width: f32, height: f32) -> Self {
        Canvas {
            width,
            height,
            commands: Vec::new(),
        }
    }

    /// Draw text by recording a draw command.
    pub fn draw_text(&mut self, content: &str, x: f32, y: f32, font_size: f32) {
        self.commands.push(DrawCommand::Text {
            content: content.to_string(),
            x,
            y,
            font_size,
        });
    }
}

/// Trait for a rendering engine.
pub trait RenderingEngine {
    fn draw(&mut self, widgets: &[WidgetData]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_text_on_canvas() {
        let mut canvas = Canvas::new(800.0, 600.0);
        canvas.draw_text("Hello World", 100.0, 200.0, 16.0);
        assert_eq!(canvas.commands.len(), 1);
        match &canvas.commands[0] {
            DrawCommand::Text {
                content,
                x,
                y,
                font_size,
            } => {
                assert_eq!(content, "Hello World");
                assert_eq!(*x, 100.0);
                assert_eq!(*y, 200.0);
                assert_eq!(*font_size, 16.0);
            }
            _ => panic!("Expected Text command"),
        }
    }
}
