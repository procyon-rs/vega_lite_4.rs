use polars::prelude::*;
use vega_lite_4::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input data: a CSV reader
    let df = CsvReader::from_path("examples/res/data/stocks.csv")?
        .has_header(true)
        .finish()?;
    let df = df
        .lazy()
        .filter(
            col("symbol")
                .eq(lit("AMZN"))
                .or(col("symbol").eq(lit("AAPL"))),
        )
        .collect()?;
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price: Amazon vs Apple")
        .description("Amazon vs Apple stock price over time.")
        .data(df)
        .mark(Mark::Line)
        .encoding(
            EdEncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("date")
                    .position_def_type(Type::Temporal)
                    .axis(AxisBuilder::default().title("date").build()?)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("price")
                    .position_def_type(Type::Quantitative)
                    .axis(AxisBuilder::default().title("price").build()?)
                    .build()?)
                .color(
                    ColorClassBuilder::default()
                        .field("symbol")
                        .legend(
                            LegendBuilder::default()
                                .orient(LegendOrient::Right)
                                .title(RemovableValue::Remove)
                                .build()?,
                        )
                        .build()?,
                )
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
