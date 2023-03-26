// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::schema::*;
// use std::str::FromStr;
// use std::convert::TryFrom;

// impl FromStr for Vegalite {
//     type Err = serde_json::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         serde_json::from_str(s)
//     }
// }

impl Vegalite {
    /// Render the json for a graph
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// impl TryFrom<&Vegalite> for String {
//     type Error = serde_json::Error;
//     fn try_from(v: &Vegalite) -> Result<Self, Self::Error> {
//         v.to_string()
//     }
// }

// for every enum with String(String)
macro_rules! from_into_string{
    ( $( $x:ident ),* $(,)? ) => {
            $(
                impl From<&str> for $x
                {
                    fn from(v: &str) -> Self {
                        $x::String(v.into())
                    }
                }
            )*
    };
}

// enum that have a `String(String)`variant
// `grep -B 5 "String(String)" src/schema.rs | grep "pub enum" | sort | sed 's/pub enum \(.*\) {/\1/'`
from_into_string!(
    BindingValue,
    ClearUnion,
    Color,
    ConditionalAxisPropertyFontStyleNull,
    ConditionalAxisPropertyStringNull,
    ConditionalPredicateValueDefTextExprRefText,
    ConditionalValueDefGradientStringNullExprRefValue,
    ConditionalValueDefNumberExprRefPredicateComposition,
    ConditionalValueDefTextExprRefText,
    DateTimeValue,
    DayUnion,
    DomainElement,
    ElementUnion,
    Equal,
    Field,
    FluffyRange,
    Format,
    GridColorUnion,
    InlineDatasetValue,
    LegendText,
    Lt,
    MarkConfigColor,
    MarkConfigFill,
    Month,
    OnUnion,
    OverlayMarkDefTooltip,
    ParamValue,
    PredicateCompositionElement,
    PrimitiveValue,
    PurpleStream,
    RangeM,
    Scheme,
    SelectionInit,
    SelectionInitInterval,
    TextElement,
    TitleUnion,
    Toggle,
    UrlDataInlineDataset,
);

// for every enum with a variant that takes a vec of an enum with a String(String) variant
// could be better with const generics : https://github.com/rust-lang/rust/issues/44580
// as in https://github.com/rust-lang/rust/issues/61415. For now, macro will generate impl for arrays up to size 32
macro_rules! from_into_array_of_str{
    ( $( $e:ident::$v:ident(Vec<$t:ident>) ),* $(,)? ) => {
        from_into_array_of_str!($( $e::$v(Vec<$t>), )*, 32,31,30,29,28,27,26,25,24,23,
        22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
    };
    ( $( $e:ident::$v:ident(Vec<$t:ident>) ,)*, $end:expr ) => {
        // implementations for Vec
        $(
            impl From<Vec<&str>> for $e
            {
                fn from(v: Vec<&str>) -> Self {
                    $e::$v(v.iter().map(|s| $t::String(s.to_string())).collect())
                }
            }
            impl From<Vec<String>> for $e
            {
                fn from(v: Vec<String>) -> Self {
                    $e::$v(v.into_iter().map($t::String).collect())
                }
            }
        )*
    };
    ( $( $e:ident::$v:ident(Vec<$t:ident>) ,)*, $i:expr, $($tail:expr),+ ) => {
            // implementations for array of size $i
            $(
                impl From<[&str; $i]> for $e
                {
                    fn from(v: [&str; $i]) -> Self {
                        $e::$v(v.iter().map(|s| $t::String(s.to_string())).collect())
                    }
                }
            )*
            from_into_array_of_str!($( $e::$v(Vec<$t>), )*, $($tail),*);
    };
}

macro_rules! from_into_array_of_str_opt{
    ( $( $e:ident::$v:ident(Vec<Option<$t:ident>>) ),* $(,)? ) => {
        from_into_array_of_str_opt!($( $e::$v(Vec<Option<$t>>), )*, 32,31,30,29,28,27,26,25,24,23,
        22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
    };
    ( $( $e:ident::$v:ident(Vec<Option<$t:ident>>) ,)*, $end:expr ) => {
        // implementations for Vec
        $(
            impl From<Vec<&str>> for $e
            {
                fn from(v: Vec<&str>) -> Self {
                    $e::$v(v.iter().map(|s| Some($t::String(s.to_string()))).collect())
                }
            }
            impl From<Vec<String>> for $e
            {
                fn from(v: Vec<String>) -> Self {
                    $e::$v(v.into_iter().map($t::String).map(|v| Some(v)).collect())
                }
            }
        )*
    };
    ( $( $e:ident::$v:ident(Vec<Option<$t:ident>>) ,)*, $i:expr, $($tail:expr),+ ) => {
            // implementations for array of size $i
            $(
                impl From<[&str; $i]> for $e
                {
                    fn from(v: [&str; $i]) -> Self {
                        $e::$v(v.iter().map(|s| Some($t::String(s.to_string()))).collect())
                    }
                }
            )*
            from_into_array_of_str_opt!($( $e::$v(Vec<Option<$t>>), )*, $($tail),*);
    };
}

// enums that have a variant that take a Vec of an enum with a `String` variant
// grep -B 5 "String(String)" src/schema.rs | grep "pub enum" | sort | sed 's/pub enum \(.*\) {/\1/' | \
//   xargs -I % sh -c "grep '(Vec<%>),' src/schema.rs | \
//   xargs -I {} sh -c 'grep -B 5 \"{}\" src/schema.rs | grep \"pub enum\" | sed \"s/pub enum \(.*\) {/\1/\" | \
//     xargs -I $ sh -c \"echo \\\"$::{}\\\"\"'" | sort | uniq
from_into_array_of_str!(
    DateTimeValue::UnionArray(Vec<SelectionInitInterval>),
    ScaleRange::UnionArray(Vec<FluffyRange>),
    SortArray::UnionArray(Vec<SelectionInitInterval>),
    SortUnion::UnionArray(Vec<SelectionInitInterval>),
    Values::UnionArray(Vec<SelectionInitInterval>),
);
from_into_array_of_str_opt!(DomainUnion::UnionArray(Vec<Option<DomainElement>>),);

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_string_to_string() {
//         let json1 = r#"
//             {
//                 "$schema": "https://vega.github.io/schema/vega-lite/v3.json",
//                 "description": "Google's stock price over time.",
//                 "data": {"url": "data/stocks.csv"},
//                 "transform": [{"filter": "datum.symbol==='GOOG'"}],
//                 "mark": "line",
//                 "encoding": {
//                     "x": {"field": "date", "type": "temporal"},
//                     "y": {"field": "price", "type": "quantitative"}
//                 }
//             }
//         "#;
//         let vega1 = Vegalite::from_str(json1).unwrap();
//         //dbg!(vega1);
//         //let json2 = vega1.to_string().unwrap();
//         //let vega2 = Vegalite::from_str(json2).unwrap();
//         //assert_eq!(json1, json2);
//         //assert_eq!(vega1, vega2);
//     }
// }
