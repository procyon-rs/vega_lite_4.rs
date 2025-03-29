use vega_lite_4::*;

// On windows stack overflow:
// ```
// thread 'main' has overflowed its stack
// error: process didn't exit successfully: `target\debug\examples\from_json_spec.exe` (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)
// ```
// as a workaround, spawn new thread or change the compilation see .cargo/config.toml
// see
// - [Stack Overflow when trying to run from\_json\_spec example · Issue #35 · procyon-rs/vega\_lite\_4.rs](https://github.com/procyon-rs/vega_lite_4.rs/issues/35#issuecomment-2748511426)
// - [What can I do to avoid "thread 'main' has overflowed its stack" when working with large arrays - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/what-can-i-do-to-avoid-thread-main-has-overflowed-its-stack-when-working-with-large-arrays/77091/5)
// - [Stack overflow when compiling on Windows 10 - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/stack-overflow-when-compiling-on-windows-10/50818)
//
fn main() -> ()  {
  /*
    const N: usize = 1_000_000;

    std::thread::Builder::new()
        .stack_size(size_of::<f64>() * N)
        .spawn(|| main_2())
        .unwrap().join().unwrap()
  */
  main_2()
}

fn main_2() -> () {
    let spec = r##"
{
  "$schema": "https://vega.github.io/schema/vega-lite/v4.json",
  "description": "A population pyramid for the US in 2000, created using stack. See https://vega.github.io/vega-lite/examples/concat_population_pyramid.html for a variant of this created using concat.",
  "data": { "url": "https://raw.githubusercontent.com/vega/vega-datasets/master/data/population.json"},
  "transform": [
    {"filter": "datum.year == 2000"},
    {"calculate": "datum.sex == 2 ? 'Female' : 'Male'", "as": "gender"},
    {"calculate": "datum.sex == 2 ? -datum.people : datum.people", "as": "signed_people"}
  ],
  "width": 300,
  "height": 200,
  "mark": "bar",
  "encoding": {
    "y": {
      "field": "age", "type": "ordinal",
      "axis": null, "sort": "descending"
    },
    "x": {
      "aggregate": "sum", "field": "signed_people", "type": "quantitative",
      "axis": {"title": "population", "format": "s"}
    },
    "color": {
      "field": "gender", "type": "nominal",
      "scale": {"range": ["#e377c2", "#1f77b4"]},
      "legend": {"orient": "top", "title": null}
    }
  },
  "config": {
    "view": {"stroke": null},
    "axis": {"grid": false}
  }
}
"##;

    let chart: Vegalite = serde_json::from_str(spec).unwrap();

    // display the chart using `showata`
    chart.show().unwrap();

    // print the vega lite spec
    eprintln!("{}", chart.to_string().unwrap());

    eprintln!("{:#?}", chart);
}
