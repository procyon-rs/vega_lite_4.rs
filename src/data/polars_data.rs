use crate::RemovableValue;
use crate::UrlData;
use crate::UrlDataBuilder;
use polars::prelude::{AnyValue, DataFrame, DataType};
use serde_json::json;
use std::collections::HashMap;
use std::iter::zip;


impl From<DataFrame> for UrlData {
    fn from(mut df: DataFrame) -> Self {
        df.as_single_chunk_par();
        let mut iters = df
            .get_columns()
            .iter()
            .map(|s| s.as_series().expect("Non empty datafarme").iter())
            .collect::<Vec<_>>();
        let columns: Vec<&str> = df.get_column_names_str();
        let mut res = vec![];
        for _ in 0..df.height() {
            let mut row = HashMap::new();
            for (column, iter) in zip(&columns, &mut iters) {
                let value = iter.next().expect("should have as many iterations as rows");
                let value = match value {
                    AnyValue::Null => json!(None::<String>),
                    AnyValue::Int64(val) => json!(val),
                    AnyValue::UInt64(val) => json!(val),
                    AnyValue::Int32(val) => json!(val),
                    AnyValue::UInt32(val) => json!(val),
                    AnyValue::Int16(val) => json!(val),
                    AnyValue::UInt16(val) => json!(val),
                    AnyValue::Int8(val) => json!(val),
                    AnyValue::UInt8(val) => json!(val),
                    AnyValue::Float32(val) => json!(val),
                    AnyValue::Float64(val) => json!(val),
                    AnyValue::String(val) => json!(val),
                    AnyValue::List(val) => match val.dtype() {
                        DataType::Int64 => {
                            let vec: Vec<Option<_>> = val.i64().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::UInt64 => {
                            let vec: Vec<Option<_>> = val.u64().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::Int32 => {
                            let vec: Vec<Option<_>> = val.i32().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::UInt32 => {
                            let vec: Vec<Option<_>> = val.u32().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::Int16 => {
                            let vec: Vec<Option<_>> = val.i16().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::UInt16 => {
                            let vec: Vec<Option<_>> = val.u16().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::Int8 => {
                            let vec: Vec<Option<_>> = val.i8().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::UInt8 => {
                            let vec: Vec<Option<_>> = val.u8().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::Float32 => {
                            let vec: Vec<Option<_>> = val.f32().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::Float64 => {
                            let vec: Vec<Option<_>> = val.f64().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        DataType::String => {
                            let vec: Vec<Option<_>> = val.str().unwrap().into_iter().collect();
                            json!(vec)
                        }
                        x => panic!(
                            "unable to parse list column: {} with value: {} and type: {:?}",
                            column,
                            x,
                            x.inner_dtype()
                        ),
                    },
                    AnyValue::Datetime(val, _, _) => json!(val),
                    AnyValue::Duration(val, _) => json!(val),
                    AnyValue::Time(val) => json!(val),
                    x => panic!("unable to parse column: {} with value: {}", column, x),
                };
                row.insert(*column, value);
            }
            res.push(serde_json::to_value(row).unwrap());
        }

        UrlDataBuilder::default().values(res).build().unwrap()
    }
}

impl From<DataFrame> for RemovableValue<UrlData> {
    fn from(df: DataFrame) -> Self {
        RemovableValue::Specified(df.into())
    }
}
