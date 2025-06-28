# [Implementation and Visualization of Plane Sweep](https://tanculau.github.io/plane-sweep/)

## Implemented Algorithm

- Plain Brute Force Algorithm
- Plane Sweep Algorithm
- Voronoi Diagrams (Planned)

## Screenshots

## How to use

## How to build

### Web

 [Website](https://tanculau.github.io/plane-sweep/)

### Web Locally

We use [Trunk](https://trunkrs.dev/) to build for web target.
You also need [Rust](https://www.rust-lang.org/).

1. [Install Rust](https://www.rust-lang.org/learn/get-started)
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
1. Install Trunk with `cargo install --locked trunk`.
1. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
1. Open `http://127.0.0.1:8080/index.html#dev` in a browser.

### Web Deploy

We use [Trunk](https://trunkrs.dev/) to build for web target.
You also need [Rust](https://www.rust-lang.org/).

1. [Install Rust](https://www.rust-lang.org/learn/get-started)
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
1. Install Trunk with `cargo install --locked trunk`.
1. Just run `trunk build --release -M`.
1. It will generate a `dist` directory as a "static html" website

### Native

1. [Install Rust](https://www.rust-lang.org/learn/get-started)

- to just run the program `cargo run`
- to build `cargo build`, you will then find the program in target/debug/plane-sweep
- to build the release version `cargo build --release`, you will then find the program in target/debug/plane-sweep
- to build the release version with optimization for performance instead of size, use `cargo build --profile release-native`
- to install the app `cargo install --locked --path .`
- to install with optimization for performance instead of size, use  `cargo install --locked --profile release-native --path .`

## Documentation

1. [Install Rust](https://www.rust-lang.org/learn/get-started)
1. Run `cargo doc --no-deps --all-features` to document only the own crates without dependencies

## Reference

- [1] M. De Berg, O. Cheong, M. Van Kreveld, and M. Overmars, Computational geometry. 2008. doi: 10.1007/978-3-540-77974-2.

- The algorithm used here are described in Computational Geometry from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8). This was also used as reference for this implementation.
- The project uses [egui](https://github.com/emilk/egui/), [eframe](https://github.com/emilk/egui/tree/main/crates/eframe) for the graphical user interface.
- Also the [eframe_template](https://github.com/emilk/eframe_template/tree/main) was used as base and got adapted.
