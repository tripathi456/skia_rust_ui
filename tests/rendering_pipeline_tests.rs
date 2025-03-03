// // // tests/rendering_pipeline_tests.rs

// // use skia_rust_ui::core::render::{Canvas, MockCanvas, Font, RenderError};
// // use skia_rust_ui::core::widget::text_widget::TextWidget;

// // /// Test that the canvas abstraction records a text draw call correctly using the MockCanvas.
// // #[test]
// // fn test_canvas_draw_text() {
// //     // Arrange: create a mock canvas and a sample font.
// //     let mut canvas = MockCanvas::new();
// //     let font = Font::new("Arial", 16.0);
// //     let text = "Hello, TDD!";
// //     let x = 30.0;
// //     let y = 40.0;

// //     // Act: call draw_text on the canvas.
// //     let result = canvas.draw_text(text, x, y, &font);

// //     // Assert: verify the operation succeeded and the call was recorded.
// //     assert!(result.is_ok(), "draw_text should return Ok(())");
// //     let calls = canvas.get_draw_calls();
// //     assert_eq!(calls.len(), 1, "One draw call should have been recorded");

// //     let call = &calls[0];
// //     assert_eq!(call.text, text, "Draw call text should match");
// //     assert_eq!(call.x, x, "Draw call x coordinate should match");
// //     assert_eq!(call.y, y, "Draw call y coordinate should match");
// //     assert_eq!(call.font, font, "Draw call font should match");
// // }

// // /// Test that a TextWidget renders correctly by issuing a draw call via the Canvas abstraction.
// // #[test]
// // fn test_text_widget_render() {
// //     // Arrange: create a TextWidget with specific text, position, and font.
// //     let font = Font::new("Arial", 14.0);
// //     let widget = TextWidget {
// //         text: "Rendering TextWidget".to_string(),
// //         font: font.clone(),
// //         x: 50.0,
// //         y: 75.0,
// //     };

// //     let mut canvas = MockCanvas::new();

// //     // Act: have the widget render itself onto the canvas.
// //     widget.render(&mut canvas);

// //     // Assert: check that a draw call was issued with the correct parameters.
// //     let calls = canvas.get_draw_calls();
// //     assert_eq!(calls.len(), 1, "TextWidget should issue one draw call");
    
// //     let call = &calls[0];
// //     assert_eq!(call.text, widget.text, "Rendered text should match widget text");
// //     assert_eq!(call.x, widget.x, "Rendered x position should match widget x");
// //     assert_eq!(call.y, widget.y, "Rendered y position should match widget y");
// //     assert_eq!(call.font, widget.font, "Rendered font should match widget font");
// // }


// // tests/rendering_pipeline_tests.rs

// use skia_rust_ui::core::layout::WidgetData;
// use skia_rust_ui::core::rendering::{Canvas, DrawCommand, RenderingEngine};
// use skia_rust_ui::core::rendering::pipeline::render_frame;
// use skia_rust_ui::core::rendering::skia_renderer::SkiaRenderer;
// use skia_rust_ui::widget::text_widget::{TextWidget, Widget};

// #[test]
// fn test_canvas_draw_text() {
//     let mut canvas = Canvas::new(800.0, 600.0);
//     let text = "Hello TDD";
//     let x = 100.0;
//     let y = 200.0;
//     let font_size = 16.0;
//     canvas.draw_text(text, x, y, font_size);
//     assert_eq!(canvas.commands.len(), 1);
//     match &canvas.commands[0] {
//         DrawCommand::Text {
//             content,
//             x: x_cmd,
//             y: y_cmd,
//             font_size: fs_cmd,
//         } => {
//             assert_eq!(content, text);
//             assert_eq!(*x_cmd, x);
//             assert_eq!(*y_cmd, y);
//             assert_eq!(*fs_cmd, font_size);
//         }
//         _ => panic!("Expected a Text draw command"),
//     }
// }

// #[test]
// fn test_text_widget_render() {
//     // Create a TextWidget.
//     let mut text_widget = TextWidget::new("Rendering TDD");
//     // Create a canvas.
//     let mut canvas = Canvas::new(800.0, 600.0);
//     // Render the widget.
//     text_widget.render(&mut canvas);
//     // Verify that the canvas recorded the draw command.
//     assert_eq!(canvas.commands.len(), 1);
//     match &canvas.commands[0] {
//         DrawCommand::Text {
//             content,
//             x,
//             y,
//             font_size,
//         } => {
//             assert_eq!(content, "Rendering TDD");
//             // Default position from widget data is (0.0, 0.0)
//             assert_eq!(*x, 0.0);
//             assert_eq!(*y, 0.0);
//             assert_eq!(*font_size, 16.0);
//         }
//         _ => panic!("Expected a Text draw command from TextWidget"),
//     }
// }

// #[test]
// fn test_render_pipeline_integration() {
//     // Create sample widget data.
//     let widget_data = WidgetData {
//         x: 50.0,
//         y: 100.0,
//         width: 100.0,
//         height: 50.0,
//     };
//     let widgets = vec![widget_data];
//     // Instantiate the renderer.
//     let renderer = SkiaRenderer::new();
//     // Call render_frame. Our stubbed SkiaRenderer should process the widgets.
//     render_frame(&renderer, &widgets);
//     // For this stub test, we simply ensure no errors occur.
//     assert!(true);
// }


// tests/rendering_tests.rs

use skia_rust_ui::core::rendering::{Canvas, DrawCommand, RenderingEngine};
use skia_rust_ui::core::layout::WidgetData;
// If you haven't exposed SkiaRenderer publicly, do so or inline it here:
use skia_rust_ui::core::rendering::skia_renderer::SkiaRenderer;

#[test]
fn test_canvas_draw_text() {
    let mut canvas = Canvas::new(800.0, 600.0);
    canvas.draw_text("Hello TDD", 100.0, 200.0, 16.0);

    assert_eq!(canvas.commands.len(), 1);
    match &canvas.commands[0] {
        DrawCommand::Text { content, x, y, font_size } => {
            assert_eq!(content, "Hello TDD");
            assert!(((*x - 100.0).abs() < f32::EPSILON));
            assert!(((*y - 200.0).abs() < f32::EPSILON));
            assert!(((*font_size - 16.0).abs() < f32::EPSILON));
        }
        _ => panic!("Expected a Text command"),
    }
}

#[test]
fn test_skia_renderer_draw() {
    let mut renderer = SkiaRenderer::new(800, 600);

    let widget = WidgetData {
        x: 10.0,
        y: 20.0,
        width: 100.0,
        height: 30.0,
    };
    let widgets = vec![widget];

    // Should not panic
    renderer.draw(&widgets);

    // Optionally, save to a file for manual inspection
    renderer.save_to_file("test_skia_renderer_draw.png");
    
    // If you want a programmatic pixel check, you'd read the PNG or do more advanced checks.
    assert!(std::path::Path::new("test_skia_renderer_draw.png").exists());
}
