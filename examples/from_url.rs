use vega_lite_5::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price")
        .description("Google's stock price over time.")
        .data(UrlDataBuilder::default().url(
            "https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/data/stocks.csv"
        ).build()?)
        .transform(vec![
            TransformBuilder::default().filter("datum.symbol==='GOOG'")
        .build()?])
        .mark(Mark::Line)
        .encoding(EdEncodingBuilder::default()
            .x(XClassBuilder::default()
                .field("date")
                .position_def_type(Type::Temporal)
                .build()?)
            .y(YClassBuilder::default()
                .field("price")
                .position_def_type(Type::Quantitative)
                .build()?).build()?).build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
