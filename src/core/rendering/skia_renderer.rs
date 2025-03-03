// src/core/rendering/skia_renderer.rs

use skia_safe::{
    surfaces, Color, Color4f, EncodedImageFormat, Font, 
    Paint, Point, Rect, Surface, Typeface,
};
use crate::core::layout::WidgetData;
use crate::core::rendering::RenderingEngine;

/// A RenderingEngine implementation that draws widgets onto
/// a CPU-backed Skia surface, then can save to file.
pub struct SkiaRenderer {
    surface: Surface,
    pub width: i32,
    pub height: i32,
}

impl SkiaRenderer {
    /// Creates a CPU-backed raster surface (width x height).
    /// Use `surfaces::raster_n32_premul()` to allocate pixel memory.
    pub fn new(width: i32, height: i32) -> Self {
        // Creates a raster surface that is 32-bpp (N32) with premultiplied alpha.
        let surface = surfaces::raster_n32_premul((width, height))
            .expect("Failed to create raster surface.");

        SkiaRenderer {
            surface,
            width,
            height,
        }
    }

    /// Saves the current surface as a PNG file. Useful for testing or demos.
    pub fn save_to_file(&mut self, path: &str) {
        let image = self.surface.image_snapshot();
        // In skia-safe 0.82.0, we need to use encode() instead of encode_to_data
        // and we need to provide None for the context parameter
        let data = image
            // .encode()
            .encode(None, EncodedImageFormat::PNG, None)
            .expect("Failed to encode image to PNG data.");
        std::fs::write(path, data.as_bytes()).expect("Failed to write PNG file.");
    }
}

/// Implement the trait so this renderer can be used in your UI framework.
impl RenderingEngine for SkiaRenderer {
    /// Draw each `WidgetData` as a rectangle plus a text label.
    /// In a real system, you'd likely call each widget's `render` method.
    fn draw(&mut self, widgets: &[WidgetData]) {
        // Get the canvas from the surface
        let canvas = self.surface.canvas();

        // Clear the surface to white.
        canvas.clear(Color::WHITE);

        // Create a font with the default typeface
        // In skia-safe 0.82.0, we need to use Typeface::default_typeface()
        // let typeface = Typeface::default_typeface();
        // let font = Font::new(typeface, 18.0);
        let font = Font::default();

        // Prepare a black paint for text.
        let mut paint = Paint::default();
        paint.set_color4f(Color4f::new(0.0, 0.0, 0.0, 1.0), None);

        // Loop through widgets, draw rect + label.
        for (i, widget) in widgets.iter().enumerate() {
            // Draw a rectangle around the widget.
            let mut rect_paint = Paint::default();
            // SteelBlue color (70, 130, 180)
            rect_paint.set_color4f(Color4f::new(70.0/255.0, 130.0/255.0, 180.0/255.0, 1.0), None);

            canvas.draw_rect(
                Rect::from_xywh(widget.x, widget.y, widget.width, widget.height),
                &rect_paint,
            );

            // Draw a label with x/y coords.
            let label = format!("Widget #{} @ ({:.0},{:.0})", i, widget.x, widget.y);
            canvas.draw_str(&label, Point::new(widget.x, widget.y + 16.0), &font, &paint);
        }
    }
}
