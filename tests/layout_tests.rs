use skia_rust_ui::{FlexboxLayout, LayoutContext, WidgetData};
use pretty_assertions::assert_eq;

#[test]
fn test_single_widget_layout() {
    // Red: Test that a single widget is laid out at (0,0) with correct size
    let layout = FlexboxLayout::new();
    let ctx = LayoutContext {
        available_width: 100.0,
        available_height: 100.0,
    };
    
    let mut widget = WidgetData {
        x: 0.0,
        y: 0.0,
        width: 50.0,
        height: 50.0,
    };
    
    let mut widgets = vec![widget];
    layout.layout(&mut widgets, &ctx);
    
    // Widget should be positioned at (0,0) with size 50x50
    assert_eq!(widgets[0].x, 0.0);
    assert_eq!(widgets[0].y, 0.0);
    assert_eq!(widgets[0].width, 50.0);
    assert_eq!(widgets[0].height, 50.0);
}

#[test]
fn test_two_widgets_row_layout() {
    // Red: Test that two widgets are laid out horizontally in sequence
    let layout = FlexboxLayout::new();
    let ctx = LayoutContext {
        available_width: 200.0,
        available_height: 100.0,
    };
    
    let widget1 = WidgetData {
        x: 0.0,
        y: 0.0,
        width: 50.0,
        height: 50.0,
    };
    
    let widget2 = WidgetData {
        x: 0.0,
        y: 0.0,
        width: 50.0,
        height: 50.0,
    };
    
    let mut widgets = vec![widget1, widget2];
    layout.layout(&mut widgets, &ctx);
    
    // First widget should be at (0,0)
    assert_eq!(widgets[0].x, 0.0);
    assert_eq!(widgets[0].y, 0.0);
    assert_eq!(widgets[0].width, 50.0);
    assert_eq!(widgets[0].height, 50.0);
    
    // Second widget should be placed at (50,0) (right after the first one)
    assert_eq!(widgets[1].x, 50.0);
    assert_eq!(widgets[1].y, 0.0);
    assert_eq!(widgets[1].width, 50.0);
    assert_eq!(widgets[1].height, 50.0);
}
