use skia_rust_ui::{WidgetId, StateStore, FlexboxLayout, WidgetData, LayoutContext, HitTester, Point};
fn main() {
    // 1. Initialize a simple state store
    let mut store = StateStore::new();
    
    // 2. Create a WidgetId and store some state
    let widget_id = WidgetId::new();
    store.set_state(widget_id.clone(), Box::new("Example state".to_string()));
    
    // 3. Print retrieved state
    if let Some(state) = store.get_state_as::<String>(&widget_id) {
        println!("Retrieved state: {}", state);
    }
    
    // 4. Create and layout a few widgets using FlexboxLayout
    let mut widgets = vec![
        WidgetData { x: 0.0, y: 0.0, width: 50.0, height: 50.0 },
        WidgetData { x: 0.0, y: 0.0, width: 30.0, height: 30.0 },
        WidgetData { x: 0.0, y: 0.0, width: 20.0, height: 20.0 },
    ];
    
    let layout = FlexboxLayout::new();
    let ctx = LayoutContext {
        available_width: 800.0,
        available_height: 600.0,
    };
    layout.layout(&mut widgets, &ctx);
    
    // 5. Hit test a point against the newly laid out widgets
    let tester = HitTester::new();
    let test_point = Point { x: 35.0, y: 10.0 };
    if let Some(hit_index) = tester.hit_test(&test_point, &widgets) {
        println!("Point {:?} hit widget at index {}", test_point, hit_index);
    } else {
        println!("Point {:?} hit no widgets", test_point);
    }
}
