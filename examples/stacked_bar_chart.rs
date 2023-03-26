use vega_lite_5::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Weather in Seattle")
        .data(
            UrlDataBuilder::default()
                .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/seattle-weather.csv")
                .build()?
        )
        .mark(Mark::Bar)
        .encoding(
            EdEncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("date")
                    .time_unit(TimeUnit::Month)
                    .position_def_type(Type::Ordinal)
                    .title("Month of the year")
                    .build()?)
                .y(YClassBuilder::default()
                    .aggregate(NonArgAggregateOp::Count)
                    .build()?)
                .color(ColorClassBuilder::default()
                    .field("weather")
                    .scale(ScaleBuilder::default()
                        .domain([
                            "sun",
                            "fog",
                            "drizzle",
                            "rain",
                            "snow",
                        ])
                        .range([
                            "#e7ba52",
                            "#c7c7c7",
                            "#aec7e8",
                            "#1f77b4",
                            "#9467bd",
                        ])
                        .build()?)
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
