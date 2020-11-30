use vega_lite_4::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the chart
    let chart = Vegalite {
        title: Some(Text::String("Stock price".to_string())),
        description: Some("Google's stock price over time.".to_string()),
        data: RemovableValue::Specified(UrlData {
            url: Some("https://raw.githubusercontent.com/procyon-rs/vega_lite_4.rs/master/examples/res/data/stocks.csv".to_string()),
            ..Default::default()
        }),
        transform: Some(vec![Transform {
            filter: Some(ConditionalNumberValueDefPredicateComposition::String("datum.symbol==='GOOG'".to_string())),
            ..Default::default()
        }]),
        mark: Some(AnyMark::Enum(Mark::Line)),
        encoding: Some(
            Encoding {
                x: Some(XClass {
                    field: Some(Field::String("date".to_string())),
                    def_type: Some(Type::Temporal),
                    ..Default::default()
                }),
                y: Some(YClass {
                    field: Some(Field::String("price".to_string())),
                    def_type: Some(Type::Quantitative),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ),
        ..Default::default()
    };

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
