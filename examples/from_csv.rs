use csv;
use std::path::Path;
use vega_lite_5::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input data: a CSV reader
    let rdr = csv::Reader::from_path(Path::new("examples/res/data/stocks.csv"))?;

    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(rdr)
        .transform(vec![TransformBuilder::default()
            .filter("datum[0]==='GOOG'")
            .build()?])
        .mark(Mark::Line)
        .encoding(
            EdEncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("1")
                    .position_def_type(Type::Temporal)
                    .axis(AxisBuilder::default().title("date").build()?)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("2")
                    .position_def_type(Type::Quantitative)
                    .axis(AxisBuilder::default().title("price").build()?)
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
