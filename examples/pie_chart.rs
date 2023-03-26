use serde::{Deserialize, Serialize};
use vega_lite_5::*;

#[derive(Serialize, Deserialize)]
pub struct CategoricalItem {
    pub category: i32,
    pub value: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<CategoricalItem> = vec![
        CategoricalItem {
            category: 1,
            value: 4.0,
        },
        CategoricalItem {
            category: 1,
            value: 6.0,
        },
        CategoricalItem {
            category: 3,
            value: 10.0,
        },
        CategoricalItem {
            category: 4,
            value: 3.0,
        },
        CategoricalItem {
            category: 5,
            value: 7.0,
        },
        CategoricalItem {
            category: 6,
            value: 8.0,
        },
    ];

    // the chart
    let chart = VegaliteBuilder::default()
        .title("Pie Chart")
        .description("A simple pie chart with embedded data.")
        .data(&values)
        .mark(Mark::Arc)
        .encoding(
            EdEncodingBuilder::default()
                .theta(
                    ThetaClassBuilder::default()
                        .field("value")
                        .polar_def_type(Type::Quantitative)
                        .build()?,
                )
                .color(
                    ColorClassBuilder::default()
                        .field("category")
                        .mark_prop_def_gradient_string_null_type(Type::Nominal)
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
