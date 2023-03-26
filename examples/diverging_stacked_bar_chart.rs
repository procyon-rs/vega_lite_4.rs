use vega_lite_5::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .description("A population pyramid for the US in 2000.")
        .data(UrlDataBuilder::default().url(
            "https://raw.githubusercontent.com/vega/vega-datasets/master/data/population.json"
        ).build()?)
        .height(200.)
        .width(300.)
        .transform(vec![
            TransformBuilder::default().filter("datum.year == 2000").build()?,
            TransformBuilder::default().calculate("datum.sex == 2 ? 'Female' : 'Male'").transform_as("gender").build()?,
            TransformBuilder::default().calculate("datum.sex == 2 ? -datum.people : datum.people").transform_as("signed_people").build()?,
        ])
        .mark(Mark::Bar)
        .encoding(EdEncodingBuilder::default()
            .x(XClassBuilder::default()
                .aggregate(NonArgAggregateOp::Sum)
                .field("signed_people")
                .position_def_type(Type::Quantitative)
                .axis(AxisBuilder::default().title("population").format("s").build()?)
                .build()?)
            .y(YClassBuilder::default()
                .field("age")
                .position_def_type(Type::Ordinal)
                .sort(Sort::Descending)
                .axis(RemovableValue::Remove)
                .build()?)
            .color(ColorClassBuilder::default()
                .field("gender")
                .mark_prop_def_gradient_string_null_type(Type::Nominal)
                .scale(ScaleBuilder::default().range(vec![
                    FluffyRange::String("#e377c2".to_string()),
                    FluffyRange::String("#1f77b4".to_string())
                ]).build()?)
                .legend(LegendBuilder::default().orient(LegendOrient::Top).title(RemovableValue::Remove).build()?)
                .build()?)
            .build()?)
        .config(ConfigClassBuilder::default()
            .view(ViewConfigBuilder::default().stroke(RemovableValue::Remove).build()?)
            .axis(AxisConfigBuilder::default().grid(false).build()?)
            .build()?)
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
