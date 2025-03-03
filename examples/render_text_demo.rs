// examples/render_text_demo.rs

use skia_rust_ui::core::layout::{FlexboxLayout, LayoutContext, WidgetData};
use skia_rust_ui::core::rendering::skia_renderer::SkiaRenderer;
use skia_rust_ui::core::rendering::RenderingEngine;
use skia_rust_ui::core::event::{HitTester, Point};

fn main() {
    // 1. Define some sample widgets to lay out
    let mut widgets = vec![
        WidgetData { x: 0.0, y: 0.0, width: 100.0, height: 50.0 },
        WidgetData { x: 0.0, y: 0.0, width: 120.0, height: 50.0 },
        WidgetData { x: 0.0, y: 0.0, width: 80.0,  height: 50.0 },
    ];

    // 2. Perform a simple flexbox layout
    let layout_engine = FlexboxLayout::new();
    let layout_ctx = LayoutContext {
        available_width: 400.0,
        available_height: 300.0,
    };
    layout_engine.layout(&mut widgets, &layout_ctx);

    // 3. Create our Skia renderer (CPU-backed in this example)
    let mut renderer = SkiaRenderer::new(400, 300);

    // 4. (Optional) event test: let's see if a certain point hits any widget
    let hit_tester = HitTester::new();
    let test_point = Point { x: 50.0, y: 20.0 };
    if let Some(index) = hit_tester.hit_test(&test_point, &widgets) {
        println!("Point {:?} hit widget #{}", test_point, index);
    } else {
        println!("Point {:?} did not hit any widget", test_point);
    }

    // 5. Render the widgets
    renderer.draw(&widgets);

    // 6. Save the result as a PNG
    renderer.save_to_file("render_text_demo.png");
    println!("Saved demo output to render_text_demo.png");

    // (Optional) If you want a real-time window, you'd integrate winit/glfw here:
    // - Create a window
    // - Use an OpenGL context
    // - Repeatedly call renderer.draw() on each frame or event
}
