# Minimal UI Framework

This project is a lightweight UI framework written in Rust. It provides a foundation for building user interfaces with custom widgets, flexible layouts, and a simple rendering engine. The project also includes basic state management and event handling, making it easy to prototype and extend your own UI components. It leverages **Skia**—the high-performance graphics engine used by Chrome and Flutter—for its rendering capabilities.

---

## Getting Started

### Prerequisites

- **Rust Toolchain:**  
  Ensure you have the latest stable Rust toolchain installed. You can install Rust via [rustup](https://rustup.rs).

- **Skia Dependencies:**  
  The Skia renderer requires the `skia-safe` crate. Follow the [skia-safe documentation](https://github.com/rust-skia/rust-skia) for any system-specific setup.

### Building the Project

Clone the repository and build using Cargo:

```bash
git clone <repository-url>
cd <repository-directory>
cargo build
```

### Run examples

```bash
cargo run --examples render_text_demo

```
---

## Usage

Integrate the framework in your Rust project by using the provided modules. For example, you can create and render a text widget as follows:

```rust
use skia_rust_ui::widget::text_widget::TextWidget;
use skia_rust_ui::core::layout::FlexboxLayout;
use skia_rust_ui::core::rendering::Canvas;

fn main() {
    let mut text_widget = TextWidget::new("Hello, UI!");
    let layout_context = skia_rust_ui::core::layout::LayoutContext {
        available_width: 800.0,
        available_height: 600.0,
    };

    // Layout the widget (in this simple example, layout does not modify widget data)
    text_widget.layout(&layout_context);

    // Create a canvas and render the widget
    let mut canvas = Canvas::new(800.0, 600.0);
    text_widget.render(&mut canvas);

    // The canvas now contains the draw command for the text widget
    println!("Canvas commands: {:?}", canvas.commands);
}
```

For rendering with the Skia renderer, refer to the `SkiaRenderer` implementation and the `render_frame` pipeline in the `core/rendering` module.


## Overview

The framework is organized into several core modules:

- **Core:**  
  Contains the essential building blocks for UI development, including:
  - **State Management:** A simple store to manage widget states.
  - **Layout:** A flexbox-inspired layout engine that arranges widgets.
  - **Event Handling:** Hit testing and event management for user interactions.
  - **Rendering:** An abstraction over rendering, featuring a canvas for drawing and a Skia-based renderer.

- **Widget:**  
  Provides implementations for basic UI elements. For example, the `TextWidget` is a minimal widget that displays text on the screen.

- **Entry Point:**  
  The `main.rs` file currently prints "Hello, world!" and serves as a starting point for integrating the framework into an application.

---

## Key Features

- **Flexible Layout:**  
  Uses a simplified Flexbox layout to arrange widgets horizontally.
  
- **Custom Rendering:**  
  - A canvas abstraction records drawing commands.
  - A Skia-based renderer (`SkiaRenderer`) draws widgets on a CPU-backed surface and can output PNG files.
  
- **Widget Abstraction:**  
  Widgets implement a simple trait with methods for layout and rendering.
  
- **State Management:**  
  A `StateStore` allows storing and retrieving arbitrary widget states using unique widget IDs.
  
- **Event Handling:**  
  A hit tester module determines which widget is under a given point, facilitating user interaction handling.

---

## Directory Structure

```
├── src
│   ├── lib.rs              # Main library file, re-exports core types and widgets
│   ├── main.rs             # Entry point: prints "Hello, world!"
│   ├── core
│   │   ├── mod.rs          # Re-exports submodules: state, layout, event, rendering
│   │   ├── state
│   │   │   ├── mod.rs      # State management module re-exporting the store
│   │   │   └── store.rs    # Implementation of state storage using widget IDs
│   │   ├── layout
│   │   │   ├── mod.rs      # Re-exports layout types and implementations
│   │   │   └── flex.rs     # Flexbox layout engine implementation
│   │   ├── event
│   │   │   ├── mod.rs      # Re-exports event-related types
│   │   │   └── hit_testing.rs # Hit testing logic for event handling
│   │   └── rendering
│   │       ├── mod.rs      # Rendering module re-exporting canvas, commands, etc.
│   │       ├── pipeline.rs # Simplified render pipeline implementation
│   │       └── skia_renderer.rs # Skia-based rendering engine implementation
│   └── widget
│       ├── mod.rs          # Re-exports widget submodules
│       └── text_widget.rs  # Implementation of a simple text widget
```


---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to enhance the framework, add new features, or fix bugs.

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes.
4. Open a pull request detailing your changes.

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Acknowledgments

- [Rust](https://www.rust-lang.org/)
- [Skia](https://skia.org/) for providing the graphics engine foundation via `skia-safe`.

Feel free to reach out with any questions or feedback.

Happy coding!