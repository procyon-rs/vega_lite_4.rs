# Vega-Lite V4 for Rust

[![license](https://img.shields.io/crates/l/vega_lite_4.svg)](https://spdx.org/licenses/Apache-2.0.html)
[![version](https://img.shields.io/crates/v/vega_lite_4.svg)](https://crates.io/crates/vega_lite_4)
[![Release Doc](https://docs.rs/vega_lite_4/badge.svg)](https://docs.rs/vega_lite_4)

[![Actions Status](https://github.com/procyon-rs/vega_lite_4.rs/workflows/ci-flow/badge.svg)](https://github.com/procyon-rs/vega_lite_4.rs/actions)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=procyon-rs/vega_lite_4.rs)](https://dependabot.com)

A Rust API for Vega-Lite V4 to build chart with a rusty API.

Similar to the [Altair](https://altair-viz.github.io/) project in python, this crate build upon [Vega-Lite](https://vega.github.io/vega-lite/) specifications. Vega-Lite is a high-level grammar of interactive graphics. It provides a concise JSON syntax for rapidly generating visualizations to support analysis. Vega-Lite specifications can be compiled to [Vega](https://vega.github.io/vega/)  specifications. Those specifications are then parsed by Vega’s JavaScript runtime to generate both static images or interactive web-based views.
This crate has a complete mapping of Vega-Lite 3.4 specification and can be found in `src/schema.rs`.
With all the types and structs, it's possible to create your Rust Vegalite graph that will be serialize into a Vega-Lite JSON. Thanks to [Showata](https://crates.io/crates/showata) the resulting visualization can be display in your Web-Browser or in a Rust [Jupyter Notebook](https://crates.io/crates/evcxr_jupyter).
It's also possible to use an existing Vega-Lite json and plug your data source seamlessly. This way you can leverage existing vizualisation and adapt it to your design.

## Examples

In order to have a complete mapping of the Vega-Lite V4 specification the code for the schema was automaticlly generated.
To help describe all the possible features a [gallery of example is provided on github](https://github.com/procyon-rs/vega_lite_4.rs/blob/master/examples/)

[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/screens/cloropleth_unemployment.png" height="150px">](https://github.com/procyon-rs/vega_lite_4.rs/blob/master/examples/cloropleth_unemployment.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/screens/diverging_stacked_bar_chart.png" height="150px">](https://github.com/procyon-rs/vega_lite_4.rs/blob/master/examples/diverging_stacked_bar_chart.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/screens/scatterplot.png" height="150px">](https://github.com/procyon-rs/vega_lite_4.rs/blob/master/examples/scatterplot.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/screens/stacked_bar_chart.png" height="150px">](https://github.com/procyon-rs/vega_lite_4.rs/blob/master/examples/stacked_bar_chart.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/screens/stock_graph.png" height="150px">](https://github.com/procyon-rs/vega_lite_4.rs/blob/master/examples/stock_graph.rs)
[<img src="https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/screens/line_with_interval.png" height="150px">](https://github.com/procyon-rs/vega_lite_4.rs/blob/master/examples/line_with_interval.rs)

To launch all examples

```sh
cargo install cargo-make
cargo make run-all-examples
```

### Simple chart using ndarray generated data

```rust
let values: Array2<f64> = Array::random((100, 2), StandardNormal);

let chart = VegaliteBuilder::default()
    .title("Random points")
    .data(values)
    .mark(Mark::Point)
    .encoding(
        EncodingBuilder::default()
            .x(XClassBuilder::default()
                .field("data.0")
                .def_type(StandardType::Quantitative)
                .build()?)
            .y(YClassBuilder::default()
                .field("data.1")
                .def_type(StandardType::Quantitative)
                .build()?)
            .build()?,
    )
    .build()?;
chart.show()?;
```

### Simple chart using existing json definition with new data

```rust
// Use existing vega-lite json specification
let spec = r##"{
    "$schema": "https://vega.github.io/schema/vega-lite/v4.0.json",
    "encoding": {
        "x": {
            "field": "data.0",
            "type": "quantitative"
        },
        "y": {
            "field": "data.1",
            "type": "quantitative"
        }
    },
    "mark": "point",
    "title": "Random points"
}"##;

// Use you own data to populate the chart
let values: Array2<f64> = Array::random((100, 2), StandardNormal);
let mut chart: Vegalite = serde_json::from_str(spec)?;
chart.data = values.into();

// display the chart using `showata`
chart.show()?;
```

## Features

| name      | enabled by default | functionnality                                     | related crate                                 |
| --------- | ------------------ | -------------------------------------------------- | --------------------------------------------- |
| show_vega | yes                | can display charts in the browser or in a notebook | [showata](https://crates.io/crates/showata)   |
| csv       | yes                | can load data from a csv                           | [csv](https://crates.io/crates/csv)           |
| ndarray   | yes                | can load data from a ndarray                       | [ndarray](https://crates.io/crates/ndarray)   |
| nalgebra  | no                 | can load data from a nalgebra::Matrix              | [nalgebra](https://crates.io/crates/nalgebra) |
| rulinalg  | no                 | can load data from a rulinalg::matrix::Matrix      | [rulinalg](https://crates.io/crates/rulinalg) |
| polars    | no                 | can load data from a polars::prelude::DataFrame    | [polars](https://crates.io/crates/polars)     |

## Links

- [Wiki - AGuideToRustGraphicsLibraries2019](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019)
- [A Dramatic Tour through Python’s Data Visualization Landscape (including ggplot and Altair) – Regress to Impress](https://dsaber.com/2016/10/02/a-dramatic-tour-through-pythons-data-visualization-landscape-including-ggplot-and-altair/)
- [Specifying Data in Altair — Altair 3.0.0 documentation](https://altair-viz.github.io/user_guide/data.html#long-form-vs-wide-form-data)
- [Visualization — list of Rust libraries/crates // Lib.rs](https://lib.rs/visualization)
- [Quicktype](https://quicktype.io/) (got issue with the [alternative](https://transform.now.sh/json-to-rust-serde)) was used to bootstrap `src/schema.rs` from the [vega-lite's json schema](https://vega.github.io/schema/vega-lite/v4.json)

## Troubleshoot

### Stack size

The vegalite json schema is large with lot of alternative, so the typed rust version create a large set of struct and enum (the generated source file before macro expension is 28K lines). So the size of a model in stack could be large (it's also why Box is used in the struct).

On wasm32, windows, with the default stack size, using vegalite_4 can raise error like:

- crash tab with `SIGSEVG` (on chromium based browser)
- `Uncaught (in promise) RuntimeError: memory access out of bounds` or simply `Uncaught (in promise) RuntimeError`
- `thread 'main' has overflowed its stack`
- `error: process didn't exit successfully: ... (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)`

The current work arround is to increase the stacksize (eg ~ 1.5 MB). For cargo based project you can add into `.cargo/config.toml` file of the project:

```toml
[target.wasm32-unknown-unknown]
rustflags = [
  "-C", "link-args=-z stack-size=1500000",
]

# 64 bit MSVC
[target.x86_64-pc-windows-msvc]
rustflags = [
	"-C", "link-arg=/STACK:1500000"
]

# 64 bit Mingw
[target.x86_64-pc-windows-gnu]
rustflags = [
    "-C", "link-arg=-Wl,--stack,1500000"
]
```

Increasing the stack size can also be done in Thread

```rust
    const N: usize = 1_500_000;

    std::thread::Builder::new()
        .stack_size(size_of::<f64>() * N)
        .spawn(|| work_with_vegalite()) // <-- launch your job
        .unwrap().join().unwrap()
```

see:

  - [Stack Overflow when trying to run from\_json\_spec example · Issue #35 · procyon-rs/vega\_lite\_4.rs](https://github.com/procyon-rs/vega_lite_4.rs/issues/35#issuecomment-2748511426)
  - [What can I do to avoid "thread 'main' has overflowed its stack" when working with large arrays - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/what-can-i-do-to-avoid-thread-main-has-overflowed-its-stack-when-working-with-large-arrays/77091/5)
  - [Stack overflow when compiling on Windows 10 - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/stack-overflow-when-compiling-on-windows-10/50818)
