#!/bin/bash

set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." >/dev/null 2>&1 && pwd)"

file=${1:-"${DIR}/src/schema.rs"}

npm install quicktype --prefix "$DIR"

url=https://vega.github.io/schema/vega-lite/v4.json
escaped_url=$(echo $url | sed 's#/#\\\/#g')

curl -o schema.json $url

echo '-- replace "const" by enum with one value'
sed -i 's/"const": \(.*\),/"enum": [\1],/' schema.json

echo '-- generating file from schema'
"${DIR}/node_modules/.bin/quicktype" \
  --src schema.json \
  --src-lang schema \
  --out "$file" \
  --top-level Vegalite \
  --density dense \
  --visibility public \
  --derive-debug \
  --no-edition-2018

echo '-- remove extra comments'
sed -i '/^\/\/[^\/]*$/d' "$file"

echo '-- inserting license and lint'
cat <<EOF >tmp_schema.rs
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

#![allow(
    missing_docs,
    clippy::doc_lazy_continuation,
    clippy::large_enum_variant
)]

EOF
cat "$file" >>tmp_schema.rs
mv tmp_schema.rs "$file"

echo '-- fix serde import'
# sed -i 's/extern crate serde_json;/use serde::{Deserialize, Serialize};/' "$file"
sed -i 's/extern crate serde_derive;/use serde::{Deserialize, Serialize};/' "$file"

echo '-- set fields that have special meaning for null'
sed -i '/use serde::/i\
use crate::removable_value::RemovableValue;' "$file"
python3 scripts/if_null_fields.py "$file" tmp_schema.rs && mv tmp_schema.rs "$file"

echo '-- custom changes'
python3 scripts/custom_changes.py "$file" tmp_schema.rs && mv tmp_schema.rs "$file"

echo '-- skip serializing None by default'
sed -i 's/pub \(\w*\): Option/#[serde(skip_serializing_if = "Option::is_none")] pub \1: Option/' "$file"

echo '-- fix Box usage'
sed -i 's/Option<Box>/Option<DefBox>/' "$file"
sed -i 's/enum Box {/enum DefBox {/' "$file"

echo '-- make everything clonable and default'
sed -i 's/#\[derive(Debug, Serialize, Deserialize)\]/#[derive(Debug, Clone, Serialize, Deserialize)]/' "$file"
sed -i 's/pub struct/#[derive(Default)] pub struct/' "$file"

echo '-- setup builder'
sed -i '/use serde::/i\
use derive_builder::Builder;' "$file"
sed -i 's/pub struct/#[derive(Builder)] pub struct/' "$file"
sed -i 's/pub struct/#[builder(setter(into, strip_option))] pub struct/' "$file"
sed -i 's/pub \(\w*\): Option/#[builder(default)] pub \1: Option/' "$file"
sed -i "s/#\[builder(default)\] pub schema: Option/#[builder(default = \"Some(\\\\\"$escaped_url\\\\\".to_string())\")] pub schema: Option/" "$file"

sed -i 's/pub \(\w*\): \([^<]*\),$/#[serde(skip_serializing_if = "Option::is_none")] #[builder(default)] pub \1: Option<\2>,/' "$file"

echo '-- simplification'
sed -i 's/pub enum InlineDataset /#[allow(unused)]enum UnusedInlineDataset /' "$file"
sed -i 's/<InlineDataset>/<serde_json::value::Value>/' "$file"
sed -i 's/BoxPlotDefClass/MarkDefClass/g' "$file"
sed -i 's/BoxPlotDefExtent/MarkDefExtent/' "$file"
sed -i 's/Enum(BoxPlot)/Enum(Mark)/' "$file"
sed -i 's/<BoxPlot>/<Mark>/' "$file"
sed -i 's/pub enum BoxPlot /pub enum Mark /' "$file"

sed -i 's/pub \(\w*\): Box<Option<\(\S*\)>>/#[serde(skip_serializing_if = "Option::is_none")] #[builder(default)] pub \1: Option<\2>/' "$file"
#sed -i 's/pub filter: Option<Box<ConditionalValueDefGradientStringNullLogicalOperandPredicatePredicate>>,/pub filter: Option<ConditionalValueDefGradientStringNullLogicalOperandPredicatePredicate>,/' "$file"

echo '-- From for enums'
sed -i '/use serde::/i\
use derive_more::From;' "$file"
sed -i 's/#\[serde(untagged)\]$/#[serde(untagged)] #[derive(From)]/' "$file"

echo '-- Fix doc links'
sed -i 's/types#datetime/struct.DateTime.html/' "$file"

echo '-- allocation on heap to reduce stack use'
# Boxing of some fields to avoid the following error
#   test tests::serde_should_not_failed_on_empty ...
#   thread 'main' has overflowed its stack
#   fatal runtime error: stack overflow
# sed -i -E 's/: Option<([A-Z]\w*)>/: Option<Box<\1>>/g' "$file"
# sed -i 's/: Option<Box<String>>/: Option<String>/g' "$file"

sed -i 's/: Option<SpecSpec>/: Option<Box<SpecSpec>>/g' "$file"
sed -i 's/: Option<Autosize>/: Option<Box<Autosize>>/g' "$file"
sed -i 's/: Option<Bounds>/: Option<Box<Bounds>>/g' "$file"
sed -i 's/: Option<Center>/: Option<Box<Center>>/g' "$file"
sed -i 's/: Option<Projection>/: Option<Box<Projection>>/g' "$file"
sed -i 's/: Option<Resolve>/: Option<Box<Resolve>>/g' "$file"
sed -i 's/: Option<Spacing>/: Option<Box<Spacing>>/g' "$file"
sed -i 's/: Option<Facet>/: Option<Box<Facet>>/g' "$file"
sed -i 's/: Option<RepeatUnion>/: Option<Box<RepeatUnion>>/g' "$file"
sed -i 's/: Option<ViewBackground>/: Option<Box<ViewBackground>>/g' "$file"
sed -i 's/: Option<VegaliteSpec>/: Option<Box<VegaliteSpec>>/g' "$file"
sed -i 's/: Option<VegaliteAlign>/: Option<Box<VegaliteAlign>>/g' "$file"
sed -i 's/: Option<Config>/: Option<Box<Config>>/g' "$file"
sed -i 's/: Option<ConfigClass>/: Option<Box<ConfigClass>>/g' "$file"
sed -i 's/: Option<EdEncoding>/: Option<Box<EdEncoding>>/g' "$file"
sed -i 's/: Option<Color>/: Option<Box<Color>>/g' "$file"
sed -i 's/: Option<Padding>/: Option<Box<Padding>>/g' "$file"

sed -i 's/Step(Step)/Step(Box<Step>)/g' "$file"
sed -i 's/TitleParams(TitleParams)/TitleParams(Box<TitleParams>)/g' "$file"

cargo fmt -- "$file"

rm "${DIR}/schema.json"
