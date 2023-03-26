use std::collections::HashMap;
use vega_lite_5::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut selector_1 = HashMap::new();
    selector_1.insert(
        "brush".to_string(),
        SelectionDefBuilder::default()
            .encodings(vec![SingleDefUnitChannel::X])
            .selection_def_type(SelectionDefType::Interval)
            .build()?,
    );
    let mut selector_2 = HashMap::new();
    selector_2.insert(
        "click".to_string(),
        SelectionDefBuilder::default()
            .encodings(vec![SingleDefUnitChannel::Color])
            .selection_def_type(SelectionDefType::Multi)
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
      NormalizedSpecBuilder::default()
        .selection(selector_1)
        .transform(vec![TransformBuilder::default()
          .filter(ConditionalValueDefNumberExprRefPredicateComposition::Predicate(Box::new(
            PredicateBuilder::default()
              .selection(ConditionalValueDefNumberExprRefSelectionComposition::String("click".to_string()))
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
                  ConditionalPredicateValueDefGradientStringNullExprRefClassBuilder::default()
                    .selection(ConditionalValueDefNumberExprRefSelectionComposition::String("brush".to_string()))
                    .conditional_value_def_gradient_string_null_expr_ref_type(Type::Nominal)
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
      NormalizedSpecBuilder::default()
        .width(600.)
        .mark(Mark::Bar)
        .selection(selector_2)
        .transform(vec![TransformBuilder::default()
          .filter(ConditionalValueDefNumberExprRefPredicateComposition::Predicate(Box::new(
            PredicateBuilder::default()
              .selection(ConditionalValueDefNumberExprRefSelectionComposition::String("brush".to_string()))
              .build()?,
          )))
          .build()?])
        .encoding(
          EdEncodingBuilder::default()
            .color(
              ColorClassBuilder::default()
                .condition(
                  ConditionalPredicateValueDefGradientStringNullExprRefClassBuilder::default()
                    .selection(ConditionalValueDefNumberExprRefSelectionComposition::String("click".to_string()))
                    .conditional_value_def_gradient_string_null_expr_ref_type(Type::Nominal)
                    .field("weather")
                    .title("Weather")
                    .build()?,
                )
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
