use nalgebra::Matrix4x2;
use vega_lite_5::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A new matrix with 4 rows and 2 columns.
    let values = Matrix4x2::new(1, 2, 3, 4, 5, 6, 7, 8);

    // the chart
    let chart = VegaliteBuilder::default()
        .title("Random points")
        .data(values)
        .mark(Mark::Point)
        .encoding(
            EdEncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("0")
                    .position_def_type(Type::Quantitative)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("1")
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
