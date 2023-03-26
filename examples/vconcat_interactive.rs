use vega_lite_5::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut selector_1 = vec![];
    selector_1.push(
        SelectionParameterBuilder::default()
            .name("brush")
            .select(SelectionConfigBuilder::default()
                    .selection_config_type(SelectionType::Interval)
                    .encodings(vec![SingleDefUnitChannel::X])
                    .build()?)
            .build()?,
    );
    let mut selector_2 = vec![];
    selector_2.push(
        SelectionParameterBuilder::default()
            .name("click")
            .select(ParamSelect::SelectionConfig(
                SelectionConfigBuilder::default()
                    .selection_config_type(SelectionType::Point)
                    .encodings(vec![SingleDefUnitChannel::Color])
                    .build()?,
            ))
            .build()?,
    );

    let chart = VegaliteBuilder::default()
    .title("Seattle Weather, 2012-2015")
    .data(
      UrlDataBuilder::default()
        .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/seattle-weather.csv")
        .build()?,
    )
    .vconcat(vec![
      SpecBuilder::default()
        .params(selector_1)
        .transform(vec![TransformBuilder::default()
            .filter(ConditionalValueDefNumberExprRefPredicateComposition::Predicate(Box::new(
            PredicateBuilder::default()
              .param("click")
              .build()?,
          )))
          .build()?])
        .mark(Mark::Point)
        .width(600.)
        .height(300.)
        .encoding(
          EdEncodingBuilder::default()
            .color(
              ColorClassBuilder::default()
                .condition(
                  ConditionalPValueDefGradientStringNullExprRefBuilder::default()
                    .param("brush")
                    .conditional_p_value_def_gradient_string_null_expr_ref_type(Type::Nominal)
                    .field("weather")
                    .title("Weather")
                    .scale(
                      ScaleBuilder::default()
                        .domain(["sun", "fog", "drizzle", "rain", "snow"])
                        .range(["#e7ba52", "#c7c7c7", "#aec7e8", "#1f77b4", "#9467bd"])
                        .build()?,
                    )
                    .build()?,
                )
                .value("lightgray")
                .build()?,
            )
            .x(
              XClassBuilder::default()
                .field("date")
                .position_def_type(Type::Temporal)
                .time_unit(TimeUnit::Monthdate)
                .axis(AxisBuilder::default().title("date").format("%b").build()?)
                .build()?,
            )
            .y(
              YClassBuilder::default()
                .field("temp_max")
                .position_def_type(Type::Quantitative)
                .scale(
                  ScaleBuilder::default()
                    .domain(vec![Some(DomainElement::Double(-5.0)), Some(DomainElement::Double(40.0))])
                    .build()?,
                )
                .axis(
                  AxisBuilder::default()
                    .title("Maximum Daily Temperature (C)")
                    .build()?,
                )
                .build()?,
            )
            .size(
              SizeClassBuilder::default()
                .title("Precipitation")
                .field("precipitation")
                .mark_prop_def_number_type(Type::Quantitative)
                .scale(
                  ScaleBuilder::default()
                    .domain(vec![Some(DomainElement::Double(-1.0)), Some(DomainElement::Double(50.0))])
                    .build()?,
                )
                .build()?,
            )
            .build()?,
        )
        .build()?,
      SpecBuilder::default()
        .width(600.)
        .mark(Mark::Bar)
        .params(selector_2)
        .transform(vec![TransformBuilder::default()
            .filter(ConditionalValueDefNumberExprRefPredicateComposition::Predicate(Box::new(
            PredicateBuilder::default()
            .param("brush")
            .build()?,
          )))
          .build()?])
        .encoding(
          EdEncodingBuilder::default()
            .color(
              ColorClassBuilder::default()
                .condition(
                  ConditionalPValueDefGradientStringNullExprRefBuilder::default()
                    .param("click")
                    .conditional_p_value_def_gradient_string_null_expr_ref_type(Type::Nominal)
                    .field("weather")
                    .scale(
                      ScaleBuilder::default()
                        .domain(["sun", "fog", "drizzle", "rain", "snow"])
                        .range(["#e7ba52", "#c7c7c7", "#aec7e8", "#1f77b4", "#9467bd"])
                        .build()?,
                    )
                    .build()?,
                )
                .value("lightgray")
                .build()?,
            )
            .x(
              XClassBuilder::default()
                .aggregate(NonArgAggregateOp::Count)
                .position_def_type(Type::Quantitative)
                .build()?,
            )
            .y(
              YClassBuilder::default()
                .title("Weather")
                .field("weather")
                .position_def_type(Type::Nominal)
                .build()?,
            )
            .build()?,
        )
        .build()?,
    ])
    .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
