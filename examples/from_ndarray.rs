use vega_lite_5::*;

use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input data: a random ndarray
    let values: Array2<f64> = Array::random((100, 2), StandardNormal);

    // the chart
    let chart = VegaliteBuilder::default()
        .title("Random points")
        .data(values)
        .mark(Mark::Point)
        .encoding(
            EdEncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("data.0")
                    .position_def_type(Type::Quantitative)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("data.1")
                    .position_def_type(Type::Quantitative)
                    .build()?)
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
