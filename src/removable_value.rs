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

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

use crate::schema::*;

/// Wrapper for a field that can be either the default value, null or specified
#[derive(Clone, Debug, Default)]
pub enum RemovableValue<T: Clone> {
    /// The default value for this field
    #[default]
    Default,
    /// This field should be removed
    Remove,
    /// This field should have the specified value
    Specified(T),
}

// For serialization, this relies on serde field attribute
// `#[serde(skip_serializing_if = "RemovableValue::is_default")]` and `Serialize` trait implementation
// With this attribute, the field won't be serialized if it's `RemovableValue::Default`. If it's
// `RemovableValue::Remove` it's serialized as a `None`, which is a `null` in json. If it's
// `RemovableValue::Specified(value)`, the value itself is serialized

// For deserialization, this relies on serde field attribute `#[serde(default)]` and `Deserialize` trait implementation
// With this attribute, if the field is not present, it's deserialized using the `Default` implementation, which
// gives a `RemovableValue::Default`. If the field is present in the json but `null`, it's deserialized as a
// `RemovableValue::Remove`. If it has a valid value, it's deserialized as a `RemovableValue::Specified(value)`

impl<T: Clone> RemovableValue<T> {
    pub(crate) fn is_default(&self) -> bool {
        matches!(self, RemovableValue::Default)
    }
}

impl<T: Clone> From<T> for RemovableValue<T> {
    fn from(value: T) -> Self {
        RemovableValue::Specified(value)
    }
}

macro_rules! from_into_with_removable{
    ( $( $from:ty => $to:ty ),* $(,)? ) => {
            $(
                impl From<$from> for RemovableValue<$to>
                {
                    fn from(v: $from) -> Self {
                        RemovableValue::Specified(v.into())
                    }
                }
            )*
    };
}

from_into_with_removable! {
    &str => String,

    &str => BindingValue,
    &str => ClearUnion,
    &str => Color,
    &str => ConditionalAxisPropertyFontStyleNull,
    &str => ConditionalAxisPropertyStringNull,
    &str => ConditionalPredicateValueDefTextExprRefText,
    &str => ConditionalValueDefGradientStringNullExprRefValue,
    &str => ConditionalValueDefNumberExprRefPredicateComposition,
    &str => ConditionalValueDefTextExprRefText,
    &str => DateTimeValue,
    &str => DayUnion,
    &str => DomainElement,
    &str => ElementUnion,
    &str => Equal,
    &str => Field,
    &str => FluffyRange,
    &str => Format,
    &str => GridColorUnion,
    &str => InlineDatasetValue,
    &str => LegendText,
    &str => Lt,
    &str => MarkConfigColor,
    &str => MarkConfigFill,
    &str => Month,
    &str => OnUnion,
    &str => OverlayMarkDefTooltip,
    &str => ParamValue,
    &str => PredicateCompositionElement,
    &str => PrimitiveValue,
    &str => PurpleStream,
    &str => RangeM,
    &str => Scheme,
    &str => SelectionInit,
    &str => SelectionInitInterval,
    &str => TextElement,
    &str => TitleUnion,
    &str => Toggle,
    &str => UrlDataInlineDataset,

    Sort => SortUnion,
    EncodingSortField => SortUnion,
    // Vec<SelectionInitIntervalElement> => Sort,

    // DefWithConditionTextFieldDefValue => Tooltip,
    // Vec<TextFieldDef> => Tooltip,

    // bool => TooltipUnion,
    // f64 => TooltipUnion,
    // String => TooltipUnion,
    // TooltipContent => TooltipUnion,
}

impl<T> Serialize for RemovableValue<T>
where
    T: Serialize + Clone,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RemovableValue::Specified(ref value) => serializer.serialize_some(value),
            RemovableValue::Default => serializer.serialize_none(),
            RemovableValue::Remove => serializer.serialize_none(),
        }
    }
}

struct RemovableValueVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for RemovableValueVisitor<T>
where
    T: Deserialize<'de> + Clone,
{
    type Value = RemovableValue<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("option")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RemovableValue::Remove)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RemovableValue::Remove)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(RemovableValue::Specified)
    }

    #[doc(hidden)]
    fn __private_visit_untagged_option<D>(self, deserializer: D) -> Result<Self::Value, ()>
    where
        D: Deserializer<'de>,
    {
        Ok(match T::deserialize(deserializer) {
            Ok(v) => RemovableValue::Specified(v),
            _ => RemovableValue::Remove,
        })
    }
}

impl<'de, T> Deserialize<'de> for RemovableValue<T>
where
    T: Deserialize<'de> + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(RemovableValueVisitor {
            marker: PhantomData,
        })
    }
}
