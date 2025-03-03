pub mod core;
pub mod widget;

// Re-export commonly used types for convenience
pub use crate::core::state::WidgetId;
pub use crate::core::state::StateStore;
pub use crate::core::layout::FlexboxLayout;
pub use crate::core::layout::WidgetData;
pub use crate::core::layout::LayoutContext;
pub use crate::core::event::HitTester;
pub use crate::core::event::Point;
pub use crate::core::rendering::*;
pub use crate::widget::text_widget::TextWidget;

