use skia_rust_ui::{HitTester, Point, WidgetData};
use pretty_assertions::assert_eq;

#[test]
fn test_hit_inside_widget() {
    // Red: Test click inside widget bounds returns widget index
    let tester = HitTester::new();
    
    let widgets = vec![
        WidgetData {
            x: 10.0,
            y: 10.0,
            width: 100.0,
            height: 100.0,
        }
    ];
    
    let point = Point { x: 50.0, y: 50.0 }; // Inside the widget
    
    let result = tester.hit_test(&point, &widgets);
    assert_eq!(result, Some(0));
}

#[test]
fn test_hit_outside_widget() {
    // Red: Test click outside widget bounds returns None
    let tester = HitTester::new();
    
    let widgets = vec![
        WidgetData {
            x: 10.0,
            y: 10.0,
            width: 100.0,
            height: 100.0,
        }
    ];
    
    let point = Point { x: 150.0, y: 150.0 }; // Outside the widget
    
    let result = tester.hit_test(&point, &widgets);
    assert_eq!(result, None);
}
