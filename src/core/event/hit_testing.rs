use crate::core::layout::WidgetData;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct HitTester;

impl HitTester {
    pub fn new() -> Self {
        HitTester
    }

    pub fn hit_test(&self, point: &Point, widgets: &[WidgetData]) -> Option<usize> {
        // Iterate through each widget and check if point is inside its bounds
        for (index, widget) in widgets.iter().enumerate() {
            if is_point_inside_widget(point, widget) {
                return Some(index);
            }
        }
        
        // If no widget contains the point, return None
        None
    }
}

// Helper function to check if a point is inside a widget's bounds
fn is_point_inside_widget(point: &Point, widget: &WidgetData) -> bool {
    point.x >= widget.x &&
    point.x <= widget.x + widget.width &&
    point.y >= widget.y &&
    point.y <= widget.y + widget.height
}
