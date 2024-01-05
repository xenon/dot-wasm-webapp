# Graphviz WASM WebApp
Allows graphviz editing and preview in the browser using wasm.
Seems to be very fast from my small amount of testing.
## How does it work?
- Change the dot source code on the left
- Automatically recompiles to an svg on the right
## How does the code work?
- Uses a rust crate to compile Graphviz to an SVG in addition to the Leptos frontend
- So the graphviz engine and the frontend logic are wasm code
- Some basic HTML and CSS to make elements and position them
## Running the code
### Dependencies
- Rust compiler
- Wasm target compiler
- cargo install trunk
### Running
- trunk serve --open